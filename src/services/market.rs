use crate::services::stock::{get_watchlist_quote_text};
use crate::services::crypto::get_crypto_quote_text;


pub struct DailyBrief {
    pub title: String,
    pub stocks: Vec<BriefField>,
    pub crypto: Vec<BriefField>,
}

pub struct BriefField {
    pub name: String,
    pub value: String,
    pub inline: bool,
}

pub async fn build_daily_brief(finnhub_token: &str, watchlist: &[String]) -> anyhow::Result<DailyBrief> {
    


    let finn_quote = get_watchlist_quote_text(watchlist, finnhub_token).await?;
    let crypto_quote = get_crypto_quote_text().await?;


    // let body = client
    // .get(url)
    // .send()
    // .await?
    // .text()
    // .await?;

    // println!("API Response: {}", body);

    Ok(DailyBrief {
        title: "Daily Market Brief".to_string(),
        stocks: finn_quote,
        crypto: crypto_quote,
    })
}

