use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Invite {
    invite_id: String,
    roles: Vec<String>,
    uses: u32,
}

#[command]
// #[allowed_roles("mod")] // Commented out for debugging purposes
// TODO: Can we avoid reading the file every time? E.g. read it in
// `main` and access it from here?
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
#[allowed_roles("mod")]
async fn check(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx, "Checking invites").await {
        println!("Error checking invites: {:?}", why);
    }
    Ok(())
}
