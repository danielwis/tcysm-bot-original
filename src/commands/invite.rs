use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

/* The aim here is to...:
 * 1. Create an invite with `inv new ...`
 * 2. This triggers the "new invite" event, upon which we can add it to the db
 * 3. Associate roles with invites, e.g. `inv associate <invite-code> <role>`
 * 4. Profit
 */

#[derive(Deserialize, Serialize)]
struct Invite {
    invite_id: String,
    roles: Vec<RoleId>,
    uses: u32,
}

#[command]
// #[allowed_roles("mod")] // Commented out for debugging purposes
async fn new(ctx: &Context, msg: &Message) -> CommandResult {
    let json_file_path = std::path::PathBuf::from(env::var("JSON_PATH")
        .expect("Error getting JSON path from environment."));
    let contents = fs::read_to_string(json_file_path)
        .expect("Unable to read JSON file");

    if let Err(why) = msg.channel_id.say(&ctx,contents).await {
        println!("Error creating invite: {:?}", why);
    }
    Ok(())
}

#[command]
async fn delete(ctx: &Context, msg: &Message) -> CommandResult {
    unimplemented!();
}

#[command]
#[allowed_roles("Mod")]
async fn check(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx, "Checking invites").await {
        println!("Error checking invites: {:?}", why);
    }
    Ok(())
}
