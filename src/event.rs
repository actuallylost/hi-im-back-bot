use anyhow::{anyhow, Error};
use poise::serenity_prelude as serenity;
use regex::Regex;
use serenity::json::JsonMap;
use serenity::json::Value;
use serenity::HttpBuilder;
use tracing::{error, info};

use std::env;

use crate::Data;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    let pattern = Regex::new(r"(?i)\b(?:i['’]?m|i am)\s+(\w+)\b").unwrap();
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            // If the message sender is a bot, return
            if new_message.author.bot {
                return Err(anyhow!("Sender is a bot"));
            }

            // The message's content
            let hay = &new_message.content;
            // The user's name
            let username = &new_message.author.name;

            // if it matches "im x"
            if let Some(capture) = pattern.captures(hay) {
                if let Some(nickname) = capture.get(1) {
                    let remaining = &hay[capture.get(0).unwrap().end()..].trim();
                    // check if there's more than one word after the "im" and error if so
                    if !remaining.is_empty() {
                        return Err(anyhow!("More than one word follows, no match"));
                    }

                    // the captured new nickname
                    let nickname = nickname.as_str();

                    let http = HttpBuilder::new(
                        env::var("DISCORD_TOKEN").expect("No token in HttpBuilder"),
                    )
                    .build();
                    let mut payload = JsonMap::new();
                    payload.insert("nick".to_string(), Value::from(nickname));

                    // change the user's nickname
                    http.edit_member(
                        new_message.guild_id.expect("No guild_id present"),
                        new_message.author.id,
                        &payload,
                        Some("Hi back I'm bot"),
                    )
                    .await
                    .expect(format!("{}: Failed to set nickname", &username).as_str());

                    // tell them their nickname has changed
                    new_message
                        .reply_ping(&ctx.http, format!("Hi {}, I'm bot!", nickname))
                        .await
                        .expect("Could not send reply");

                    return Ok(());
                } else {
                    error!("Couldn't capture nickname");
                }
            }
        }
        _ => {}
    }
    Ok(())
}
