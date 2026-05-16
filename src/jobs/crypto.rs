mod job;

use crate::config::AppConfig;
use crate::jobs::Job;

pub const CHANNEL_KEY: &str = "CRYPTO";

pub fn register(app_config: &AppConfig) -> Box<dyn Job + Send + Sync> {
    Box::new(job::CryptoJob::new(
        CHANNEL_KEY.to_string(),
        app_config.crypto.crypto_ids.clone(),
    ))
}
