use std::time::Duration;

use levelcrush::{
    axum::{
        extract::State,
        http::Request,
        middleware::Next,
        response::{IntoResponse, Redirect, Response},
        Json, Router,
    },
    server::Server,
    tokio, tracing,
};

use app::state::AppState;
use env::AppVariable;

mod app;
mod database;
mod env;
mod routes;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    levelcrush::env();

    let server_port = env::get(AppVariable::ServerPort).parse::<u16>().unwrap_or(3000);

    let app_state = AppState::new().await;

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

    let server_port = server_port;
    Server::new(server_port)
        .enable_cors()
        .enable_session()
        .enable_rate_limit(rate_limit, Duration::from_secs(rate_limit_per), rate_limit_buffer)
        .run(routes::router(), app_state.clone())
        .await;
}
