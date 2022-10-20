/* Commands appear to be defined as:
 * #[command]
 * #[description = ""]
 * async fn name(Context, Message) -> CommandResult {}
 * No self parameter. They should also return Ok(())
 * TODO: Break these into different files later with pub mod <filename> */


#[command]
#[description = "A simple ping command"]
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error: {:?}", why);
    }
    Ok(())
}

#[command]
async fn company(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Company called");
    if let Err(why) = msg.channel_id.say(&ctx.http, "Hi company x").await {
        println!("Error: {:?}", why);
    }
    Ok(())
}
