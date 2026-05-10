pub mod output;
pub mod market_brief;

use serenity::async_trait;
use crate::jobs::output::JobOutput;

#[async_trait]
pub trait Job {
    fn name(&self) -> &str;

    async fn run(&self) -> anyhow::Result<JobOutput>;
}