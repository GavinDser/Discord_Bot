use serenity::all::{ChannelId, Colour, CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::prelude::Context;

use crate::services::market::DailyBrief;

#[warn(dead_code)]
pub async fn send_message(ctx: &Context, channel_id: ChannelId, message: &str) {
    let result = channel_id.say(&ctx.http, message).await;

    if let Err(e) = result {
        eprintln!("Failed to send message: {:?}", e);
    }
}

pub async fn send_embed_message(ctx: &Context, channel_id: ChannelId, brief: &DailyBrief){
    let mut embed = CreateEmbed::new()
    .title(&brief.title)
    .color(Colour::DARK_GREEN)
    .description("Market snapshot")
    .footer(CreateEmbedFooter::new("Discord Market Bot"));

    for stock in &brief.stocks {
        embed = embed.field(&stock.name, &stock.value, stock.inline);
    }

    for crypto in &brief.crypto {
        embed = embed.field(&crypto.name, &crypto.value, crypto.inline);
    }

    let message = CreateMessage::new().embed(embed);

    let result = channel_id.send_message(&ctx.http, message).await;

    if let Err(e) = result {
        eprintln!("Failed to send embed message {:?}", e);
    }
}