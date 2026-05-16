use serde::Deserialize;
use tracing::warn;

use crate::presenters::quote::QuoteItem;

#[derive(Deserialize)]
struct FinnhubQuote {
    c: f64,
    d: f64,
    dp: f64,
    h: f64,
    l: f64,
    o: f64,
    pc: f64,
}

pub async fn get_stock_quote_text(symbol: &str, finnhub_token: &str) -> anyhow::Result<String> {
    let url = format!(
        "https://finnhub.io/api/v1/quote?symbol={}&token={}",
        symbol, finnhub_token
    );

    let client = reqwest::Client::builder()
        .user_agent("discord-market_breif-bot/0.1")
        .build()?;

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<FinnhubQuote>()
        .await?;

    let direction = if response.dp > 0.0 {
        "[UP]"
    } else if response.dp < 0.0 {
        "[DOWN]"
    } else {
        "[FLAT]"
    };

    let quote = format!(
        "**{}**\nPrice: **${:.2}**\n{} `{:+.2}` (`{:+.2}%`)\nH/L: `${:.2}` / `${:.2}`\nOpen: `${:.2}` | Prev: `${:.2}`",
        symbol,
        response.c,
        direction,
        response.d,
        response.dp,
        response.h,
        response.l,
        response.o,
        response.pc
    );

    Ok(quote)
}

pub async fn get_stock_quote_items(
    watchlist: &[String],
    finnhub_token: &str,
) -> anyhow::Result<Vec<QuoteItem>> {
    let mut items = Vec::new();

    for symbol in watchlist {
        match get_stock_quote_text(symbol, finnhub_token).await {
            Ok(current_stock) => {
                items.push(QuoteItem {
                    name: symbol.to_string(),
                    value: current_stock,
                    inline: true,
                });
            }
            Err(e) => {
                warn!("Failed to fetch {}, error is {}", symbol, e);
                items.push(QuoteItem {
                    name: symbol.to_string(),
                    value: "**N/A**\n`Failed to fetch`".to_string(),
                    inline: true,
                });
            }
        };
    }

    Ok(items)
}
