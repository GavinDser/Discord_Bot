use std::time::Duration;
use serenity::all::ChannelId;
use serenity::prelude::Context;
use tokio::time::sleep;

use std::collections::HashMap;

use crate::config::MarketConfig;
use crate::sender::send_embed_message;
use crate::services::market;

pub async fn start_scheduler(ctx: Context, channels: HashMap<String, ChannelId>, market: MarketConfig) {
    
    if !market.enabled {
        println!("Market scheduler disabled");
        return;
    }

    loop {
        
        match market::build_daily_brief(&market.finnhub_token, &market.watchlist).await {
            Ok(brief) => send_embed_message(&ctx, get_channel(&channels, "DAILY_BRIEF").unwrap(), &brief).await,
            Err(e) => eprintln!("Failed to build daily brief: {:?}", e),
        }
    
        sleep(Duration::from_secs(600)).await;
    }
}

fn get_channel(channels: &HashMap<String, ChannelId>, name: &str) -> Option<ChannelId> {
    channels.get(name).copied()
}