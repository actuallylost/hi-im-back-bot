use std::env;

use anyhow::Error;
use dotenv::dotenv;
use event::event_handler;
use poise::serenity_prelude as serenity;
use poise::FrameworkError::*;
use tracing::{error, info};

mod event;
mod ping;

// User data
pub struct Data {}

#[allow(unused)]
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Error handler
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        Command { error, ctx, .. } => {
            error!("Error in command `{}`: {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!("Error while handling error: {:?}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Loads env file
    dotenv().ok();

    // Subscribes to
    tracing_subscriber::fmt::init();

    // Defines token
    let token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN, check .env!");

    // Defines intents
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Defines framework options
    let framework = poise::Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping()],
            // How events are handled
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            // On setup or command error
            on_error: |error| Box::pin(on_error(error)),
            // This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    info!("[LOG] Executed command {}!", ctx.command().qualified_name);
                })
            },
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
