use std::time::Duration;
use serenity::prelude::Context;
use tokio::time::sleep;
use std::sync::Arc;

use crate::sender::send_job_output;
use crate::config::AppConfig;
use crate::jobs::build_jobs;


pub async fn start_scheduler(ctx: Context, app_config: Arc<AppConfig>) {
    let jobs = build_jobs(&app_config);
    for job in &jobs {
        println!("Registered Job: {}", job.name());
    }

    if jobs.is_empty() {
        println!("No jobs enabled");
        return;
    }

    loop {
        for job in &jobs {
            match job.run().await {
            Ok(output) => send_job_output(&ctx, &app_config.discord.channels, output).await,
            Err(e) => eprintln!("Error happened for Job:{}: {:?}",job.name(), e)           
            }
        };

        sleep(Duration::from_secs(20)).await;
    }
}
