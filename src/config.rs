use serenity::all::ChannelId;
use std::collections::HashMap;

use anyhow::Context;

pub struct AppConfig {
    pub discord: DiscordConfig,
    pub finnhub: FinnhubConfig,
    pub stock: StockConfig,
    pub scheduler: SchedulerConfig,
    pub gemini: GeminiConfig,
}

pub struct DiscordConfig {
    pub token: String,
    pub channels: HashMap<String, ChannelId>,
}

pub struct FinnhubConfig {
    pub token: String,
}

pub struct StockConfig {
    pub watchlist: Vec<String>,
}

pub struct SchedulerConfig {
    pub run_on_start: bool,
}

pub struct GeminiConfig {
    pub api_key: String,
    pub model: String,
}


impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        //discord information
        let token = std::env::var("DISCORD_TOKEN")
        .context("Discord bot token not found")?;

        let mut channels = HashMap::new();
        for (key, value) in std::env::vars() {
            if key.starts_with("CHANNEL_") {
                let name = key.trim_start_matches("CHANNEL_").to_string();
                let id = value.parse::<u64>()
                .context("Invalid channel ID")?;
                channels.insert(name, ChannelId::new(id));
            }
        }


        // external API keys
        let finnhub_token = std::env::var("FINNHUB_TOKEN")
        .context("Finnhub token not found")?;

        // stock report information
        let watchlist_raw = std::env::var("WATCHLIST")
        .context("WATCHLIST Not found in env")?;

        let watchlist: Vec<String> = watchlist_raw
        .split(',')
        .map(|symbol| symbol.trim())
        .filter(|symbol| !symbol.is_empty())
        .map(|symbol| symbol.to_uppercase())
        .collect();

        //scheduler run on start
        let run_on_start = std::env::var("RUN_ON_START")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .context("RUN_ON_START must be true or flase")?;

        //Gemini LLM config
        let gemini_key = std::env::var("GEMINI_API_KEY")
        .context("Cannot find gemini API key")?;

        let gemini_model = std::env::var("GEMINI_MODEL")
        .unwrap_or_else(|_| "gemini-2.5-flash".to_string());


        Ok(Self {
            discord: DiscordConfig {
                token,
                channels
            },
            finnhub: FinnhubConfig {
                token: finnhub_token,
            },
            stock: StockConfig {
                watchlist,
            },
            scheduler: SchedulerConfig {
                 run_on_start, 
            },
            gemini: GeminiConfig { api_key: gemini_key, 
                model: gemini_model },
        })
        

    }
}
