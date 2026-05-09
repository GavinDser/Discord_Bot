mod config;
mod handler;
mod scheduler;
mod sender;
mod services;

use serenity::all::GatewayIntents;
use serenity::prelude::*;

use config::AppConfig;
use handler::Handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_config = AppConfig::from_env()?;

    let discord_config = app_config.discord;
    let market_config = app_config.market;

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        channels: discord_config.channels,
        market: market_config,
    };

    let mut client = Client::builder(discord_config.token, intents)
        .event_handler(handler)
        .await?;

    client.start().await?;

    Ok(())  

}
