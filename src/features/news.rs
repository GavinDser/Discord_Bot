use serde::{Deserialize, Serialize};
use anyhow::anyhow;
use tracing::warn;

// Used to digest finnhub jsons
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub url: String,
    pub summary: Option<String>,
    pub datetime: Option<i64>,
    pub image: Option<String>,
}

// used to send out cleaned news structure
pub struct NewsDigest {
    pub summary: Option<String>,
    pub articles: Vec<NewsArticle>,
}


//finnhub API response types
#[derive(Deserialize)]
struct FinnhubNewsItem {
    headline: String,
    source: String,
    summary: String,
    url: String,
    datetime: i64,
    image: String,
}


//Gemini API request types
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>
}

#[derive(Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>
}

#[derive(Serialize)]
struct GeminiPart {
    text: String
}

//Gemini API response types
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiResponseContent
}

#[derive(Deserialize)]
struct GeminiResponseContent {
    parts: Vec<GeminiResponsePart>
}

#[derive(Deserialize)]
struct GeminiResponsePart {
    text: String
}


//function needed to call for news digest
pub async fn build_news_digest(finnhub_token: &str, gemini_api_key: &str, gemini_model: &str) -> anyhow::Result<NewsDigest>{
    let news_items = fetch_finnhub_news(finnhub_token).await?;

    if news_items.is_empty(){
        return Ok(NewsDigest{
            summary: Some("No summary available".to_string()),
            articles: Vec::new(),
        })
    }

    let summary = match generate_news_insight(&news_items, gemini_api_key, gemini_model).await {
        Ok(summary) => Some(summary),
        Err(err) => {
            warn!("Failed to generate Gemini news insight: {}",err);
            None
        }
    };

    let articles = news_items.into_iter().take(5).collect();

    Ok(NewsDigest {
        summary, 
        articles,
    })
}

// for safely fetching associated json value
fn empty_string_to_none(value: String) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

async fn fetch_finnhub_news(finnhub_token: &str) -> anyhow::Result<Vec<NewsArticle>> {

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
    .collect::<Vec<NewsArticle>>();

    Ok(news)


}

// converting json item to news_item
fn finnhub_item_to_news_item(item: FinnhubNewsItem) -> NewsArticle {
    NewsArticle { title: item.headline, 
        source: item.source, 
        url: item.url, 
        summary: empty_string_to_none(item.summary), 
        datetime: Some(item.datetime), 
        image: empty_string_to_none(item.image)
    }
}


// functions for creating news insight
async fn generate_news_insight(
    news_items: &[NewsArticle],
    gemini_api_key: &str,
    gemini_model: &str,
)-> anyhow::Result<String>{

    //prompt and post url for gemini assess
    let prompt = build_news_insight_prompt(news_items);

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        gemini_model,
        gemini_api_key
    );

    let gemini_request = GeminiRequest{
        contents: vec![
            GeminiContent {
                parts: vec![
                    GeminiPart {
                        text: prompt
                    }
                ]
            }
        ]
    };  

    let client = reqwest::Client::new();

    let res = client.post(&url)
    .json(&gemini_request)
    .send()
    .await?
    .error_for_status()?
    .json::<GeminiResponse>()
    .await?;
  
    let candidate = res.candidates.first().ok_or_else(||anyhow!("No Candidates found"))?;
    let part = candidate.content.parts.first().ok_or_else(|| anyhow!("No response"))?;

    Ok(part.text.clone())
}

fn build_news_insight_prompt(news_items: &[NewsArticle]) -> String{

    let mut prompt = "You are a financial market intelligence analyst.

            Analyze the following market news articles and synthesize today's key market information.
            Do not summarize each article one by one.
            Focus on market themes, key risks, affected sectors, and stocks that investors should watch.
            keep the total response under 1200 characters.

            Output exactly in this format:

            Today's Market Themes:
            1. ...
            2. ...

            Key Risks:
            - ...

            Affected Sectors:
            - ...

            Stocks to Watch:
            - ...

            Keep the total answer concise and suitable for a Discord daily brief.
            ".to_string();

    for (index, item) in news_items.iter().take(30).enumerate(){
        prompt.push_str(&format!("Article {}:\nTitle:{}\nSummary:{}\nSource:{}\n",index, item.title, item.summary.as_deref().unwrap_or("N/A"), item.source));
    }
    
    prompt
}
