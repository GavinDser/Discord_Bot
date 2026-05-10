use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::all::ChannelId;
use std::collections::HashMap;

use crate::config::MarketConfig;
use crate::scheduler::start_scheduler;

pub struct Handler {
    pub channels: HashMap<String, ChannelId>,
    pub market: MarketConfig,

}


// Handler implements EventHandler trait to handle Discord events
#[async_trait]
impl EventHandler for Handler { 
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // ctx attribute for discord connection
        let ctx_clone = _ctx.clone();
        let channels = self.channels.clone();
        let market_config = self.market.clone();
        tokio::spawn(async move {
            start_scheduler(ctx_clone, channels, market_config).await;
        });
        
    }
}
