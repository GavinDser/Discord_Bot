use std::collections::HashMap;

use serde::Deserialize;

use crate::presenters::quote::QuoteItem;

use tracing::warn;

#[derive(Deserialize)]
struct CoinPrice {
    usd: f64,
}

pub async fn get_crypto_quote_items(
    crypto_ids: &[String],
) -> anyhow::Result<Vec<QuoteItem>> {
    let ids = crypto_ids.join(",");
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        ids
    );

    let client = reqwest::Client::builder()
        .user_agent("discord-market_breif-bot/0.1")
        .build()?;

    let price_response = client
        .get(url)
        .send()
        .await?
        .json::<HashMap<String, CoinPrice>>()
        .await?;

    let mut items = Vec::new();

    for crypto_id in crypto_ids {
        if let Some(price) = price_response.get(crypto_id) {
            items.push(QuoteItem {
                name: crypto_id.to_string(),
                value: format!("**${:.2}**", price.usd),
                inline: true,
            });
        } else {
            warn!("Failed to fetch crypto price for id: {}", crypto_id);
            items.push(QuoteItem {
                name: crypto_id.to_string(),
                value: "**N/A**\n`Failed to fetch`".to_string(),
                inline: true,
            });
        }
    }

    Ok(items)
}
