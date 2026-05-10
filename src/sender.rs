use serenity::all::{ChannelId, Colour, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::prelude::Context;

use crate::jobs::output::{JobMessage, JobOutput};
use std::collections::HashMap;


pub async fn send_job_output(ctx: &Context, channels: &HashMap<String, ChannelId>, output: JobOutput){
    let Some(channel_id) = channels.get(&output.channel_key).copied() else {
        eprintln!("Channel not found: {}", &output.channel_key);
        return;
    };

    match output.message {
        JobMessage::Text(text) => {
            let result = channel_id.say(&ctx.http, text).await;

            if let Err(e) = result {
                eprintln!("Failed to send message: {:?}", e);
            }
        },
        JobMessage::Embed(embed_message) => {
            let mut embed = CreateEmbed::new()
            .title(embed_message.title)
            .color(Colour::DARK_GREEN);

            if let Some(description) = embed_message.description {
                embed = embed.description(description);
            }

            for field in embed_message.fields {
                embed = embed.field(field.name, field.value, field.inline);
            }
            
            if let Some(footer) = embed_message.footer {
                embed = embed.footer(CreateEmbedFooter::new(footer));
            }

            let message = CreateMessage::new().embed(embed);

            let result = channel_id.send_message(&ctx.http, message).await;

            if let Err(e) = result {
                eprintln!("Failed to send embed job: {:?}", e);
            }

        }
    }
}