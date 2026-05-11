use serenity::{async_trait};

use crate::jobs::Job;

use crate::jobs::output::{
    EmbedMessage, JobMessage, JobOutput
};
use crate::services::news;



pub struct NewsJob {
    channel_key: String,
    finnhub_token: String,
}

impl NewsJob {
    pub fn new(channel_key: String, token: String) -> Self{
        Self { channel_key,
        finnhub_token: token }
    }
}

#[async_trait]
impl Job for NewsJob {
    fn name(&self) -> &str{
        "General News"
    }

    async fn run(&self) -> anyhow::Result<JobOutput> {
        let fields= news::build_news_fields(&self.finnhub_token).await?;
        
        let embed_message = EmbedMessage {
            title: "Market News Brief".to_string(),
            description: Some("Top general market headlines from Finnhub".to_string()),
            fields,
            footer:Some("News Bot".to_string()),

        };

        Ok(JobOutput {
            channel_key: self.channel_key.clone(),
            message: JobMessage::Embed(embed_message),
        })
    }
}