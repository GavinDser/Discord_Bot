use crate::jobs::output::EmbedField;

mod crypto;
mod stock;

use crypto::get_crypto_quote_text;
use stock::get_watchlist_quote_text;



pub async fn build_market_fields(finnhub_token: &str, watchlist: &[String]) -> anyhow::Result<Vec<EmbedField>> {
    

    let mut fields = get_watchlist_quote_text(watchlist, finnhub_token).await?;
    let crypto_quote = get_crypto_quote_text().await?;

    fields.extend(crypto_quote);
    Ok(fields)
}

