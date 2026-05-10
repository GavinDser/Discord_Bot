mod config;
mod handler;
mod scheduler;
mod sender;
mod services;
mod jobs;

use serenity::all::GatewayIntents;
use serenity::prelude::*;
use std::sync::Arc;

use config::AppConfig;
use handler::Handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_config = Arc::new(AppConfig::from_env()?);
    let token = app_config.discord.token.clone();

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        app_config: Arc::clone(&app_config),
    };

    let mut client = Client::builder(token, intents)
        .event_handler(handler)
        .await?;

    client.start().await?;

    Ok(())  

}
