use serenity::async_trait;

use crate::jobs::Job;
use crate::jobs::output::{JobOutput, EmbedMessage, JobMessage};

use crate::services::market;

pub struct MarketBriefJob {
    finnhub_token: String,
    watchlist: Vec<String>,
    channel_key: String,
}

impl MarketBriefJob {
    pub fn new(finnhub_token: String, watchlist:Vec<String>, channel_key:String) -> Self{
        Self {
            finnhub_token,
            watchlist,
            channel_key,
        }
    }
}

#[async_trait]
impl Job for MarketBriefJob{
    fn name(&self) -> &str {
        "Market Brief"
    }

    async fn run(&self) -> anyhow::Result<JobOutput> {
        let fields = market::build_market_fields(
            &self.finnhub_token,
            &self.watchlist)
            .await?;

        let embed_message = EmbedMessage {
            title: "Daily Market Brief".to_string(),
            description: Some("Market Snapshot".to_string()),
            fields,
            footer: Some("Discord Market Bot".to_string()),
        };

        let job = JobOutput {
            channel_key: self.channel_key.clone(),
            message: JobMessage::Embed(embed_message)
        };


        Ok(job)
    }
}
