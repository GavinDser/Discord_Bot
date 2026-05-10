use std::time::Duration;
use serenity::all::ChannelId;
use serenity::prelude::Context;
use tokio::time::sleep;

use std::collections::HashMap;

use crate::jobs::Job;
use crate::jobs::market_brief::MarketBriefJob;
use crate::sender::send_job_output;

use crate::config::MarketConfig;


pub async fn start_scheduler(ctx: Context, channels: HashMap<String, ChannelId>, market: MarketConfig) {
    let mut jobs: Vec<Box<dyn Job + Send + Sync>> = Vec::new();
    if market.enabled{
        let market_job = MarketBriefJob::new(
            market.finnhub_token.clone(),
            market.watchlist.clone(),
            "DAILY_BRIEF".to_string(),
        );
        jobs.push(Box::new(market_job));
    }

    if jobs.is_empty() {
        println!("No jobs enabled");
        return;
    }

    loop {
        for job in &jobs {
            match job.run().await {
            Ok(output) => send_job_output(&ctx, &channels, output).await,
            Err(e) => eprintln!("Error happened for Job:{}: {:?}",job.name(), e)           
            }
        };

        sleep(Duration::from_secs(20)).await;
    }
}
