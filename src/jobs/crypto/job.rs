use serenity::async_trait;

use crate::features::crypto;
use crate::jobs::output::{JobMessage, JobOutput};
use crate::jobs::Job;
use crate::presenters::quote;

pub struct CryptoJob {
    channel_key: String,
    crypto_ids: Vec<String>,
}

impl CryptoJob {
    pub fn new(channel_key: String, crypto_ids: Vec<String>) -> Self {
        Self {
            channel_key,
            crypto_ids,
        }
    }
}

#[async_trait]
impl Job for CryptoJob {
    fn name(&self) -> &str {
        "Crypto Quotes"
    }

    async fn run(&self) -> anyhow::Result<JobOutput> {
        let items = crypto::get_crypto_quote_items(&self.crypto_ids).await?;

        let embed_message = quote::build_quote_embed(
            "Crypto Prices".to_string(),
            Some("Crypto snapshot".to_string()),
            items,
            Some("Discord Market Bot".to_string()),
        );

        Ok(JobOutput {
            channel_key: self.channel_key.clone(),
            message: JobMessage::Embed(embed_message),
        })
    }
}
