use serenity::{async_trait};

use crate::jobs::Job;

use crate::jobs::output::{
    EmbedField,
    EmbedMessage,
    JobMessage,
    JobOutput
};



pub struct NewsJob {
    channel_key: String,
}

impl NewsJob {
    pub fn new(channel_key: String) -> Self{
        Self { channel_key }
    }
}

#[async_trait]
impl Job for NewsJob {
    fn name(&self) -> &str{
        "General News"
    }

    async fn run(&self) -> anyhow::Result<JobOutput> {
        let fields = vec![
            EmbedField {
                name: "Mock Headline 1".to_string(),
                value: "This is a placeholder news item.".to_string(),
                inline: false,
            },
            EmbedField {
                name: "Mock Headline 2".to_string(),
                value: "This will later be replaced by real news data.".to_string(),
                inline: false,
            },
        ];

        let embed_message = EmbedMessage {
            title: "General News Brief".to_string(),
            description: Some("Top Market and AI headlines".to_string()),
            fields,
            footer: Some("Discord Market Bot".to_string()),
        };
        
        Ok(JobOutput {
            channel_key: self.channel_key.clone(),
            message: JobMessage::Embed(embed_message),
        })
    }
}