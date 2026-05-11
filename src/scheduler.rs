use std::sync::Arc;
use std::time::Duration;

use chrono::{Datelike, Duration as ChronoDuration, Local, TimeZone};
use serenity::prelude::Context;
use tokio::time::sleep;

use crate::config::AppConfig;
use crate::jobs::{build_jobs, Job};
use crate::sender::send_job_output;


async fn run_jobs(
    ctx: &Context,
    app_config: &AppConfig,
    jobs: &[Box<dyn Job + Send + Sync>],
) {
    for job in jobs {
        match job.run().await {
            Ok(output) => send_job_output(ctx, &app_config.discord.channels, output).await,
            Err(e) => eprintln!("Error happened for Job:{}: {:?}", job.name(), e),
        }
    }
}

fn duration_until_next_daily_run() -> Duration {
    let now = Local::now();

    let today_8pm = Local
        .with_ymd_and_hms(now.year(), now.month(), now.day(), 20, 0, 0)
        .single()
        .expect("valid local datetime");

    let next_run = if now < today_8pm {
        today_8pm
    } else {
        today_8pm + ChronoDuration::days(1)
    };

    next_run
        .signed_duration_since(now)
        .to_std()
        .expect("next run should be in the future")
}



pub async fn start_scheduler(ctx: Context, app_config: Arc<AppConfig>) {
    let jobs = build_jobs(&app_config);

    for job in &jobs {
        println!("Registered Job: {}", job.name());
    }

    if jobs.is_empty() {
        println!("No jobs enabled");
        return;
    }

    if app_config.scheduler.run_on_start {
        println!("RUN_ON_START=true, running jobs once now");
        run_jobs(&ctx, &app_config, &jobs).await;
    }

    loop {
        let sleep_duration = duration_until_next_daily_run();
        println!("Next daily brief in {:?}", sleep_duration);

        sleep(sleep_duration).await;

        run_jobs(&ctx, &app_config, &jobs).await;
    }
}