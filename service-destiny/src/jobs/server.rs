use std::time::Duration;

use crate::app::state::AppState;
use crate::env::AppVariable;
use crate::{env, routes};
use levelcrush::server::Server;
use levelcrush::{tokio, tracing};

pub async fn run() {
    let server_port = env::get(AppVariable::ServerPort).parse::<u16>().unwrap_or(3003);

    let app_state = AppState::new().await;

    //println!("Listening Port (ENV): {}", server_port);
    //server::run(server_port).await;

    let mut app_state_bg = app_state.clone();
    let cache_task = tokio::spawn(async move {
        loop {
            app_state_bg.cache.prune().await;
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    let app_state_task = app_state.clone();
    let task_manager = tokio::spawn(async move {
        loop {
            let running = app_state_task.tasks.step().await;
            if running > 0 {
                tracing::info!("Running {} normal level task", running);
            }

            let running = app_state_task.priority_tasks.step().await;
            if running > 0 {
                tracing::info!("Running {} priority level task", running);
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let rate_limit = std::env::var("RATE_LIMIT")
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(100);
    let rate_limit_per = std::env::var("RATE_LIMIT_DURATION")
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(10);

    let rate_limit_buffer = std::env::var("RATE_LIMIT_BUFFER")
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or(1024);

    let server_task = tokio::spawn(async move {
        Server::new(server_port)
            .enable_cors()
            .enable_rate_limit(rate_limit, Duration::from_secs(rate_limit_per), rate_limit_buffer)
            .run(routes::router(), app_state)
            .await;
    });

    // run both  concurrently
    (_, _, _) = tokio::join!(server_task, cache_task, task_manager);
}
