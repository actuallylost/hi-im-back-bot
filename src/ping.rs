use anyhow::Error;
use poise::command;

use crate::Context;

#[command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await.unwrap();

    Ok(())
}
