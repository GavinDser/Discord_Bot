use serenity::async_trait;

use crate::features::stock;
use crate::jobs::output::{JobMessage, JobOutput};
use crate::jobs::Job;
use crate::presenters::quote;

pub struct StockJob {
    channel_key: String,
    watchlist: Vec<String>,
    finnhub_token: String,
}

impl StockJob {
    pub fn new(channel_key: String, watchlist: Vec<String>, finnhub_token: String) -> Self {
        Self {
            channel_key,
            watchlist,
            finnhub_token,
        }
    }
}

#[async_trait]
impl Job for StockJob {
    fn name(&self) -> &str {
        "Stock Quotes"
    }

    async fn run(&self) -> anyhow::Result<JobOutput> {
        let items = stock::get_stock_quote_items(&self.watchlist, &self.finnhub_token).await?;

        let embed_message = quote::build_quote_embed(
            "Stock Watchlist".to_string(),
            Some("Equity snapshot".to_string()),
            items,
            Some("Discord Market Bot".to_string()),
        );

        Ok(JobOutput {
            channel_key: self.channel_key.clone(),
            message: JobMessage::Embed(embed_message),
        })
    }
}
