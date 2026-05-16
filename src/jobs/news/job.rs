use serenity::{async_trait};

use crate::jobs::Job;

use crate::jobs::output::{
    EmbedMessage, JobMessage, JobOutput
};
use crate::services::news;



pub struct NewsJob {
    channel_key: String,
    finnhub_token: String,
    gemini_api_key: String,
    gemini_model: String,
}

impl NewsJob {
    pub fn new(channel_key: String, token: String, gemini_key: String, model: String) -> Self{
        Self { 
            channel_key,
            finnhub_token: token,
            gemini_api_key: gemini_key,
            gemini_model:model,
        }
    }
}

#[async_trait]
impl Job for NewsJob {
    fn name(&self) -> &str{
        "General News"
    }

    async fn run(&self) -> anyhow::Result<JobOutput> {
        let news_digest = news::build_news_digest(
            &self.finnhub_token,
            &self.gemini_api_key,
            &self.gemini_model).await?;
        
        let embed_message = EmbedMessage {
            title: "Market News Brief".to_string(),
            description: news_digest.summary,
            fields: news_digest.fields,
            footer:Some("News Bot".to_string()),

        };

        Ok(JobOutput {
            channel_key: self.channel_key.clone(),
            message: JobMessage::Embed(embed_message),
        })
    }
}