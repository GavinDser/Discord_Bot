# AI-Powered Market Intelligence Discord Bot

A Rust-based Discord bot that delivers scheduled market intelligence updates, including stock quotes, crypto prices, market news, and AI-generated news insights.

This project is built as a backend-style learning project focused on async Rust, external API integration, modular job execution, and Docker/NAS deployment.

## Features

- Scheduled Discord delivery through Serenity and Tokio
- Feature-per-job architecture for independent stock, crypto, and news jobs
- Config-driven Discord channel routing through `.env`
- Stock watchlist quotes from Finnhub
- Crypto prices from CoinGecko
- Market news from Finnhub
- Gemini-powered market news insight generation
- Reusable presenters for quote-style and news-style Discord embeds
- Structured logging with `tracing`
- Docker/NAS deployment path

## Configuration

Create a local `.env` file. Do not commit it.

```env
DISCORD_TOKEN=
FINNHUB_TOKEN=

GEMINI_API_KEY=
GEMINI_MODEL=gemini-2.5-flash

WATCHLIST=AAPL,MSFT,NVDA
CRYPTO_IDS=bitcoin,ethereum,solana

CHANNEL_CRYPTO=
CHANNEL_STOCK=
CHANNEL_NEWS=

RUN_ON_START=true
```

Channel keys are dynamic. For example:

```env
CHANNEL_CRYPTO=123456789
```

is loaded as:

```text
CRYPTO -> ChannelId(123456789)
```

If a channel key is missing, the corresponding job is not registered.

## Running Locally

```bash
cargo run
```

For local testing, set:

```env
RUN_ON_START=true
```

For production/NAS deployment, prefer:

```env
RUN_ON_START=false
```

## Docker Deployment

If using Docker Compose:

```bash
docker compose build
docker compose up -d
docker compose logs -f
```

If only `.env` changed, a rebuild is usually not required:

```bash
docker compose up -d
```

## Tech Stack

- Rust
- Tokio
- Serenity
- Reqwest
- Serde
- Anyhow
- Chrono
- Tracing
- Finnhub API
- CoinGecko API
- Gemini API
- Docker / Docker Compose

## Roadmap

- Slash commands such as `/quote`, `/news`, and `/brief now`
- SQLite persistence for alerts, watchlists, job logs, and news history
- Price and news alert system
- Retry/backoff for external APIs
- GitHub Actions and container registry deployment
- Additional intelligence features such as macro indicators, valuation trackers, and sector-specific news

## Security

Never commit `.env`, Discord tokens, API keys, or local deployment credentials.
