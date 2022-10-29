use serenity::{framework::standard::macros::command, utils::MessageBuilder};
use serenity::framework::standard::{CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::{Deserialize, Serialize};

use crate::InviteTracker;

/* The aim here is to...:
 * 1. Create an invite with `inv new ...`
 * 2. This triggers the "new invite" event, upon which we can add it to the db
 * 3. Associate roles with invites, e.g. `inv associate <invite-code> <role>`
 * 4. Profit
 */

#[derive(Deserialize, Serialize)]
struct Invite {
    invite_id: String,
    roles: Vec<Role>,
    uses: u32,
}

#[command]
// #[allowed_roles("mod")] // Commented out for debugging purposes
async fn link(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data_locked = {
        let data = ctx.data.read().await;
        data.get::<InviteTracker>().expect("Expected InviteTracker in data/typemap").clone()
    };

    println!("{:?}", args);

    // Check that we get the guild OK
    if let Some(guild) = msg.guild(&ctx.cache) {
        // Get one argument (the invite code) and advance the arg iterator
        match args.single_quoted::<String>() {
            Ok(invite) => {
                {
                    // Is the invite in the cache?
                    let mut invites = data_locked.write().await;
                    if let std::collections::hash_map::Entry::Occupied(mut entry) = invites.entry(invite.clone()) {
                        // Get the roles (rest of the args) and add them to the cache
                        if args.is_empty() {
                            if let Err(why) = msg.channel_id.say(&ctx, "Role arguments required: !invite <invite-code> <[roles]>").await {
                                println!("Failed to send message: {:?}", why);
                            }
                            println!("No role arguments given");
                        }

                        // We have at least one role specified
                        for arg in args.iter::<String>() {
                            if let Some(role) = guild.role_by_name(&arg.unwrap_or("".to_string())) {
                                println!("Adding role: {:?}", role);
                                // Add the specified role to the hashmap. Does this even need extra scoping?
                                entry.get_mut().0.push(role.to_owned());
                            } else {
                                if let Err(why) = msg.channel_id.say(&ctx, "No invite code ".to_string() + &invite + " found.").await {
                                    println!("Error sending message: {:?}", why);
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                print!("An argument is required to run this command.");
            }
        }
    } else {
        println!("No guild found");
    }
    // React or show error
    Ok(())
}

#[command]
async fn unlink(ctx: &Context, msg: &Message) -> CommandResult {
    unimplemented!();
}

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let data_locked = {
        let data_read = ctx.data.read().await;

        // Clone as the contents of data_locked otherwise go out of scope and get dropped after
        // this block
        data_read.get::<InviteTracker>().expect("Expected InviteTracker in data/TypeMap").clone()
    };

    let invites = data_locked.read().await;

    let mut response = MessageBuilder::new();
    response.push_bold_line("Active invites:");

    // Make an iterator out of the RwLock
    for (code, (roles, _uses)) in invites.iter() {
        response.push(code.to_string() + ": ");
        response.push(roles.iter().map(|r| r.name.to_string()).collect::<Vec<String>>().join(", "));
        println!("{:?}",invites);
        if let Err(why) = msg.channel_id.say(&ctx, &response).await {
            println!("Error checking invites: {:?}", why);
        }
    }
    Ok(())
}
