use serenity::{framework::standard::macros::command, utils::MessageBuilder};
use serenity::framework::standard::CommandResult;
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
async fn link(ctx: &Context, msg: &Message) -> CommandResult {
    unimplemented!();
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
