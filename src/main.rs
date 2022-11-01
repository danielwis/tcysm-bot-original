/* Thanks to the serenity GitHub examples as well as
 * https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/
 * for providing the initial structure of this bot.
 * Especially the serenity GitHub, your examples have been fantastic for learning. */

mod commands;

use std::io::Write;
use std::{env, fs};
use std::collections::{HashSet, HashMap};
use std::sync::Arc;
use serenity::model::prelude::{GuildId, Member, Role, RoleId, InviteCreateEvent, ResumedEvent, InviteDeleteEvent};
use serenity::{
    async_trait,
    model::gateway::Ready,
    prelude::*,
};
use serenity::http::Http;
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use serde::{Deserialize, Serialize};
// use serenity::model::event::ResumedEvent;

use crate::commands::*; // Update to crate::commands::filename::* when filename is no longer
                        // "mod.rs"
use crate::commands::invite::*;

#[derive(Serialize, Deserialize, Debug)]
struct InviteRoles {
    code: String,
    roles: Vec<Role>,
}


// We want an `InviteTracker` object to look like: "<invite-id>: ([roles], uses)"
struct InviteTracker;
impl TypeMapKey for InviteTracker {
    type Value = Arc<RwLock<HashMap<String, (Vec<Role>, u64)>>>;
}

struct Handler;

#[group] // Create a group of commands
#[description = "A group of general commands"] // ...with this description
#[summary = "General stuff"] // Summary is a short desc. for when listing multiple groups at once
#[commands(ping)]
//#[commands(about, am_i_admin, say, commands, ping, latency, some_long_command, upper_command)]
struct General; // The name of the command group

#[group]
#[description = "Link invites to specific roles that will be assigned on member join"]
#[summary = "Change link-roles associations"]
#[prefixes("invite", "inv")]
#[default_command("list")]
#[commands("link", "unlink", "list", "sync", "create")]
#[allowed_roles("Mod")]
struct Invite;

#[group]
#[owners_only]
// Limit all commands to be guild-restricted.
#[only_in(guilds)] // Guild = server. ID?
// Summary only appears when listing multiple groups.
#[summary = "Commands for server admins"]
#[commands(company)]
struct Owner;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);
    }

    /// On guild member addition, we want to:
    /// 1. Check which invite they have used by comparing our cached invite count
    ///    to our server's invite count.
    /// 2. Assign the new member all roles associated with the invite. Associations
    ///    are based on the InviteTracker struct loaded at start and updated by the
    ///    role association commands.
    async fn guild_member_addition(&self, ctx: Context, mut newmem: Member) {
        if let Ok(active_invites) = newmem.guild_id.invites(&ctx.http).await {
            // We can assume that the same invite codes are present in both
            // the cached invites and the ones we get from the API call, as the (TODO)
            // invite_add and invite_delete events will update the cached ones
            let data = ctx.data.read().await;
            let cached_invites = data.get::<InviteTracker>()
                .expect("Could not find cached InviteTracker object");

            for inv in active_invites {
                if let Some( (rls, cached_count) ) = cached_invites.read().await.get(&inv.code) {
                    if inv.uses > *cached_count {
                        println!("Invite changed: {}", inv.code);
                        println!("Roles: {:?}", rls);
                        // For some reason, we need to specify the type to collect into...
                        let roleids = rls.iter().map(|r| r.id).collect::<Vec<RoleId>>();
                        if let Err(why) = newmem.add_roles(&ctx.http, &roleids).await {
                            println!("Error adding roles: {:?}", why);
                        }
                        break;
                    }
                }
            }
        } else {
            panic!("Error getting invites");
        }
    }

    async fn invite_delete(&self, ctx: Context, inv_event: InviteDeleteEvent) {
        // Add the invite to the hashmap without any roles linked to it
        let data_locked = {
            let data = ctx.data.read().await;
            data.get::<InviteTracker>().expect("Expected InviteTracker in data/typemap").clone()
        };

        {
            let mut invites = data_locked.write().await;
            invites.remove(&inv_event.code);
        }
    }

    async fn invite_create(&self, ctx: Context, inv_event: InviteCreateEvent) {
        // Add the invite to the hashmap without any roles linked to it
        let data_locked = {
            let data = ctx.data.read().await;
            data.get::<InviteTracker>().expect("Expected InviteTracker in data/typemap").clone()
        };

        {
            let mut invites = data_locked.write().await;
            invites.entry(inv_event.code).or_insert((Vec::<Role>::new(), 0));
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN")
        .expect("Failed to get Discord token from environment.");
    println!("Got token");

    let http = Http::new(&token);

    // Get the bot's owners + the bot's id.
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            println!("\nCurrent application info:\n{:?}\n", info);
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot ID: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access the application info: {:?}", why)
    };


    let framework = StandardFramework::new()
        .configure(|c| c
                   .with_whitespace(true)
                   .on_mention(Some(bot_id))
                   .prefix("!")
                   .delimiters(vec![", ", ",", " "])
                   .owners(owners))
        .group(&GENERAL_GROUP)
        .group(&INVITE_GROUP);
    /* TODO: Read up on the functionality below and configure it after proper understanding to
     * avoid copy pasting too much.
    // Set a function to be called prior to each command execution. This
    // provides the context of the command, the message that was received,
    // and the full name of the command that will be called.
    //
    // Avoid using this to determine whether a specific command should be
    // executed. Instead, prefer using the `#[check]` macro which
    // gives you this functionality.
    //
    // **Note**: Async closures are unstable, you may use them in your
    // application if you are fine using nightly Rust.
    // If not, we need to provide the function identifiers to the
    // hook-functions (before, after, normal, ...).
    .before(before)
    // Similar to `before`, except will be called directly _after_
    // command execution.
    .after(after)
    // Set a function that's called whenever an attempted command-call's
    // command could not be found.
    .unrecognised_command(unknown_command)
    // Set a function that's called whenever a message is not a command.
    .normal_message(normal_message)
    // Set a function that's called whenever a command's execution didn't complete for one
    // reason or another. For example, when a user has exceeded a rate-limit or a command
    // can only be performed by the bot owner.
    .on_dispatch_error(dispatch_error)
    // Can't be used more than once per 5 seconds:
    .bucket("emoji", |b| b.delay(5)).await
    // Can't be used more than 2 times per 30 seconds, with a 5 second delay applying per channel.
    // Optionally `await_ratelimits` will delay until the command can be executed instead of
    // cancelling the command invocation.
    .bucket("complicated", |b| b.limit(2).time_span(30).delay(5)
    // The target each bucket will apply to.
    .limit_for(LimitedFor::Channel)
    // The maximum amount of command invocations that can be delayed per target.
    // Setting this to 0 (default) will never await/delay commands and cancel the invocation.
    .await_ratelimits(1)
    // A function to call when a rate limit leads to a delay.
    .delay_action(delay_action)).await
    // The `#[group]` macro generates `static` instances of the options set for the group.
    // They're made in the pattern: `#name_GROUP` for the group instance and `#name_GROUP_OPTIONS`.
    // #name is turned all uppercase
    .help(&MY_HELP)
    .group(&GENERAL_GROUP)
    .group(&EMOJI_GROUP)
    .group(&MATH_GROUP)
    .group(&OWNER_GROUP)
    */

    // Give bot access to all channels for reading etc
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    println!("Built client");

    // Construct a hash map to be contained in an InviteTracker object
    // where it will be wrapped in Arc<RwLock> for thread safety.
    // Used to track invites' associated roles and auto-assign them on join
    let db_path = env::var("JSON_PATH")
        .expect("Could not find the JSON_PATH variable in environment");
    let db_string = {
        match fs::read_to_string(&db_path) {
            Ok(contents) => contents,
            Err(why) => {
                // Create file and return empty string
                println!("Could not read db file due to the following error: {:?}.", why);
                let mut ans = String::new();
                loop {
                    println!("Do you want to create the file? y/n");
                    std::io::stdin()
                        .read_line(&mut ans)
                        .expect("Failed to read input");
                    match ans.to_lowercase().trim() {
                        "y" => {
                            // Create file
                            {
                                let mut f = fs::File::create(&db_path)
                                    .expect("Failed to create DB file");
                                f.write_all(b"[]")
                                    .expect("Failed to write to file");
                            }
                            break;
                        }, 
                        "n" => break,
                        _ => continue
                    }
                }

                "[]".into()
            }
        }
    };
    println!("Db string read: {}", db_string);
    let local_invite_mappings: Vec<InviteRoles> = serde_json::from_str(&db_string)
        .expect("Error getting invite mappings");

    // Get known invites from discord api
    // for each of the local invites:
    // check if the code (key) exists in active_invites
    // if it doesn't, remove it from the json file
    let mut cached_invite_map = HashMap::<String, (Vec<Role>, u64)>::default();

    let guild_id = env::var("GUILD_ID")
        .expect("Could not find the GUILD_ID variable in environment").parse().expect("Unable to parse numeric guild id.");

    if let Ok(active_invites) = GuildId(guild_id).invites(http).await {
        'new_local: for inv in local_invite_mappings {
            for ac_inv in &active_invites {
                if ac_inv.code == inv.code {
                    // println!("Inv roles for {} are {:?}", inv.code, inv.roles);
                    // active_invites contains invite from disk
                    cached_invite_map
                        .entry(inv.code.to_string())
                        .or_insert((Vec::<Role>::new(), ac_inv.uses)).0 = inv.roles;
                    continue 'new_local; // Break to avoid further borrows of moved variable `inv.code` that
                           // would happen if we moved the value in `entry()` and then kept on
                           // looping (since `inv` doesn't change until the outer loop runs again).
                }
            }
            // Value in local db is not present in Discord anymore
            println!("Removing invite {} from the local DB as it is no longer present in the guild", inv.code);
            cached_invite_map.remove(&inv.code);
        }
        println!("Cached invite map between adding local and http invites: {:?}", cached_invite_map);

        // Add invite codes for things not in local
        for ac_inv in active_invites {
            cached_invite_map
                .entry(ac_inv.code)
                .or_insert((Vec::<Role>::new(), ac_inv.uses));
        }

        // Serialise the new vector and write it back to file?
        let f = fs::File::create(db_path)
            .expect("Failed to create new file");
        let mut roles_to_write = Vec::<InviteRoles>::new();
        for (code, (roles, _uses)) in cached_invite_map.iter() {
            roles_to_write.push(InviteRoles { code: code.to_string(), roles: roles.to_vec() });
        }
        println!("{:?}", roles_to_write);
        serde_json::to_writer_pretty(f, &roles_to_write)
            .expect("Failed to write updated JSON");
    } else {
        panic!("Error getting active invites from the Discord API");
    }

    // Explicitly scope this to release the lock after write
    {
        let mut data = client.data.write().await;

        // Insert an InviteTracker object into the client data.
        // This is done so that we can access it within events and other
        // methods, as `data` is available through `ctx.data`.
        data.insert::<InviteTracker>(Arc::new(RwLock::new(cached_invite_map)));
    }


    if let Err(why) = client.start().await {
        println!("Error starting client: {:?}", why);
    }
    println!("Started client");
}
