use serde::Deserialize;

use crate::services::market::BriefField;


#[derive(Deserialize)]
struct FinnhubQuote {
    c: f64,   // current price
    d: f64,   // change
    dp: f64,  // percent change
    h: f64,   // high
    l: f64,   // low
    o: f64,   // open
    pc: f64,  // previous close
}


pub async fn get_stock_quote_text(symbol: &str, finnhub_token: &str) -> anyhow::Result<String> {
    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, finnhub_token);

    let client = reqwest::Client::builder()
    .user_agent("discord-market_breif-bot/0.1")
    .build()?;

    let response = client.get(&url)
    .send()
    .await?
    .json::<FinnhubQuote>()
    .await?;


    let direction = if response.dp > 0.0 {
        "[UP]"
    }else if response.dp < 0.0 {
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

pub async fn get_watchlist_quote_text(watchlist: &[String], finnhub_token: &str) -> anyhow::Result<Vec<BriefField>>{
    let mut fields: Vec<BriefField> = Vec::new();
    for symbol in watchlist {
        match get_stock_quote_text(symbol, finnhub_token).await {
            Ok(current_stock) => {
                fields.push(BriefField {
                    name: symbol.to_string(),
                    value: current_stock,
                    inline: true,
                });
            }
            Err(e) => {
                eprintln!("Failed to fetch {}, error is {}", symbol,e);
                fields.push(BriefField { 
                    name: symbol.to_string(), 
                    value: "**N/A**\n`Failed to fetch`".to_string(), 
                    inline: true})
            }
        }; 

    }

    Ok(fields)
}