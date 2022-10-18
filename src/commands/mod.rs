/* Commands appear to be defined as:
 * #[command]
 * #[description = ""]
 * async fn name(Context, Message) -> CommandResult {}
 * No self parameter. They should also return Ok(()) */


#[command]
#[description = "A simple ping command"]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {

}
