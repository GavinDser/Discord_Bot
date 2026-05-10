use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;

use crate::config::AppConfig;
use crate::scheduler::start_scheduler;

pub struct Handler {
    pub app_config: Arc<AppConfig>,
}


// Handler implements EventHandler trait to handle Discord events
#[async_trait]
impl EventHandler for Handler { 
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // ctx attribute for discord connection
        let ctx_clone = _ctx.clone();
        let app_config = Arc::clone(&self.app_config);
        tokio::spawn(async move {
            start_scheduler(ctx_clone, app_config).await;
        });
        
    }
}
