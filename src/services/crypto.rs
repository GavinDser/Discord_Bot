use serde::Deserialize;
use crate::services::market::BriefField;

#[derive(Deserialize)]
struct CryptoPriceResponse {
    bitcoin: CoinPrice,
    ethereum: CoinPrice,
}

#[derive(Deserialize)]
struct CoinPrice {
    usd: f64,
}

pub async fn get_crypto_quote_text() -> anyhow::Result<Vec<BriefField>> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd";

    let client = reqwest::Client::builder()
    .user_agent("discord-market_breif-bot/0.1")
    .build()?;

    let response = client.get(url)
    .send()
    .await?
    .json::<CryptoPriceResponse>()
    .await?;


    Ok(vec![
        BriefField {
            name: "Bitcoin".to_string(),
            value: format!("**${:.2}**", response.bitcoin.usd),
            inline: true,
        },
        BriefField {
            name: "Ethereum".to_string(),
            value: format!("**${:.2}**", response.ethereum.usd),
            inline: true,
        },
    ])
}