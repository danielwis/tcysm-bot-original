/* Thanks to the serenity GitHub examples as well as
 * https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/
 * for providing the initial structure of this bot. */
use std::env; // Environment variables
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message){
        // Don't respond to self
        if msg.is_own(&ctx.cache).await {
            if msg.content == "!help" {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Hey look, I'm a ghost!").await {
                    println!("Error sending help message: {:?}", why);
                }
            }
            else {
                println!("Incorrect, content = {}", msg.content);
            }
        }
        else {
            println!("Own message");
        }
    }

    async fn ready(&self, _: Context, ready: Ready){
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").
        expect("Failed to get Discord token from environment");
    println!("Got token");

    let mut client = Client::builder(token).
        event_handler(Handler).await.expect("Error creating client");
    println!("Build client");
    if let Err(why) = client.start().await {
        println!("Error starting client: {:?}", why);
    println!("Started client");
    }
}
