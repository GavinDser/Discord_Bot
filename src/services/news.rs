use crate::jobs::output::EmbedField;
use serde::Deserialize;

use reqwest;



pub struct NewsItem {
    pub title: String,
    pub source: String,
    pub url: String,
    pub summary: Option<String>,
    pub datetime: Option<i64>,
    pub image: Option<String>,
}

#[derive(Deserialize)]
struct FinnhubNewsItem {
    headline: String,
    source: String,
    summary: String,
    url: String,
    datetime: i64,
    image: String,
}


pub async fn build_news_fields(finnhub_token: &str) -> anyhow::Result<Vec<EmbedField>>{
    let mut news_items = fetch_finnhub_news(finnhub_token).await?;

    if news_items.is_empty(){
        return Ok(vec![EmbedField {
            name: "No Market news available".to_string(),
            value: "Finnhub did not return general market news right now.".to_string(),
            inline: false,
        }])
    }

    news_items.truncate(5);

    let fields = news_items
    .iter()
    .map(news_item_to_field)
    .collect();
    
    Ok(fields)
}


fn news_item_to_field(item: &NewsItem) -> EmbedField {

    EmbedField {
        name: item.title.clone(),
        value: format!(
            "Source: {} | [Read more]({})",
            item.source,
            item.url,
        ),
        inline: false,
    }
}


// for safely fetching associated json value
fn empty_string_to_none(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

// converting json item to news_item
fn finnhub_item_to_news_item(item: FinnhubNewsItem) -> NewsItem {
    NewsItem { title: item.headline, 
        source: item.source, 
        url: item.url, 
        summary: empty_string_to_none(item.summary), 
        datetime: Some(item.datetime), 
        image: empty_string_to_none(item.image)
    }
}

async fn fetch_finnhub_news(finnhub_token: &str) -> anyhow::Result<Vec<NewsItem>> {

    let category = "general";

    let url = format!(
    "https://finnhub.io/api/v1/news?category={}&token={}",
    category,
    finnhub_token
    );

    let client = reqwest::Client::builder()
    .user_agent("discord-market_breif-bot/0.1")
    .build()?;

    let res = client.get(&url)
    .send()
    .await?
    .error_for_status()?
    .json::<Vec<FinnhubNewsItem>>()
    .await?;

    let news= res.into_iter()
    .map(|finnhub_item| finnhub_item_to_news_item(finnhub_item))
    .collect::<Vec<NewsItem>>();

    Ok(news)


}