mod job;

use crate::config::AppConfig;
use crate::jobs::Job;

pub const CHANNEL_KEY: &str = "NEWS";

pub fn register(app_config: &AppConfig) -> Box<dyn Job + Send + Sync> {
    Box::new(job::NewsJob::new(
        CHANNEL_KEY.to_string(),
        app_config.finnhub.token.clone(),
        app_config.gemini.api_key.clone(),
        app_config.gemini.model.clone(),
    ))
    
}
