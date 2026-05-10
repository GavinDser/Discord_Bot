mod job;

use crate::config::AppConfig;
use crate::jobs::Job;

pub const CHANNEL_KEY: &str = "NEWS";

pub fn register(_app_config: &AppConfig) -> Box<dyn Job + Send + Sync> {
    Box::new(job::NewsJob::new(CHANNEL_KEY.to_string()))
}
