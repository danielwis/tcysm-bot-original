/* Thanks to the serenity GitHub examples as well as
 * https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/
 * for providing the initial structure of this bot.
 * Especially the serenity GitHub, your examples have been fantastic for learning. */
use std::env;
use std::collections::HashSet;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::http::Http;
use serenity::framework::StandardFramework;
use serenity::framework::standard::{
    CommandResult,
};
use serenity::framework::standard::macros::{group, command};
use serenity::model::event::ResumedEvent;

struct Handler;

#[group] // Create a group of commands
#[description = "A group of general commands"] // ...with this description
#[summary = "General stuff"] // Summary is a short desc. for when listing multiple groups at once
#[commands(ping)]
//#[commands(about, am_i_admin, say, commands, ping, latency, some_long_command, upper_command)]
struct General; // The name of the command group

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

    /*
    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
    */
}

#[tokio::main]
async fn main() {
    // This will load the environment variables located at `./.env`, relative to
    // the CWD. See `./.env.example` for an example on how to structure this.
    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("DISCORD_TOKEN").
        expect("Failed to get Discord token from environment");
    println!("Got token");

    let http = Http::new(&token);

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
                   .delimiters(vec![", ", ","])
                   .owners(owners))
        .group(&GENERAL_GROUP);
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

    if let Err(why) = client.start().await {
        println!("Error starting client: {:?}", why);
    }
    println!("Started client");
}
