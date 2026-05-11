mod job;

use crate::config::AppConfig;
use crate::jobs::Job;

pub const CHANNEL_KEY: &str = "DAILY_BRIEF";

pub fn register(app_config: &AppConfig) -> Box<dyn Job + Send + Sync> {
    Box::new(job::MarketBriefJob::new(
        app_config.finnhub.token.clone(),
        app_config.stock.watchlist.clone(),
        CHANNEL_KEY.to_string(),
    ))
}
