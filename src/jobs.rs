pub mod output;
pub mod market_brief;
pub mod news;

use serenity::async_trait;
use crate::config::AppConfig;
use crate::jobs::output::JobOutput;

#[async_trait]
pub trait Job {
    fn name(&self) -> &str;

    async fn run(&self) -> anyhow::Result<JobOutput>;
}

pub struct JobRegistryEntry {
    pub channel_key: &'static str,
    pub register: fn(&AppConfig) -> Box<dyn Job + Send + Sync>,
}

pub fn registry() -> Vec<JobRegistryEntry> {
    vec![
        JobRegistryEntry {
            channel_key: market_brief::CHANNEL_KEY,
            register: market_brief::register,
        },
        JobRegistryEntry {
            channel_key: news::CHANNEL_KEY,
            register: news::register,
        },
    ]
}

pub fn build_jobs(app_config: &AppConfig) -> Vec<Box<dyn Job + Send + Sync>> {
    registry()
        .into_iter()
        .filter(|entry| app_config.discord.channels.contains_key(entry.channel_key))
        .map(|entry| (entry.register)(app_config))
        .collect()
}
