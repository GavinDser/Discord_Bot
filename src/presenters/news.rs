use crate::jobs::output::{EmbedField, EmbedMessage};
use crate::features::news::NewsArticle;

pub fn build_news_embed(
    title: String,
    summary: Option<String>,
    articles: Vec<NewsArticle>,
    footer: Option<String>,
) -> EmbedMessage {
    let fields = if articles.is_empty() {
        vec![EmbedField {
            name: "No Market news available".to_string(),
            value: "Finnhub did not return general market news right now.".to_string(),
            inline: false,
        }]
    } else {
        articles.into_iter().map(news_article_to_field).collect()
    };

    EmbedMessage {
        title,
        description: summary,
        fields,
        footer,
    }
}

fn news_article_to_field(article: NewsArticle) -> EmbedField {
    EmbedField {
        name: article.title,
        value: format!(
            "Source: {} | [Read more]({})",
            article.source,
            article.url,
        ),
        inline: false,
    }
}
