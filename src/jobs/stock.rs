mod job;

use crate::config::AppConfig;
use crate::jobs::Job;

pub const CHANNEL_KEY: &str = "STOCK";

pub fn register(app_config: &AppConfig) -> Box<dyn Job + Send + Sync> {
    Box::new(job::StockJob::new(
        CHANNEL_KEY.to_string(),
        app_config.stock.watchlist.clone(),
        app_config.finnhub.token.clone(),
    ))
}
