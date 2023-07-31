use crate::{
    app::state::AppState,
    env::{self, AppVariable},
    routes,
};
use levelcrush::{anyhow, server::Server};
use std::time::Duration;

pub async fn run() -> anyhow::Result<()> {
    let server_port = env::get(AppVariable::ServerPort).parse::<u16>().unwrap_or(3000);
    let app_state = AppState::new().await;

    let rate_limit = env::get(AppVariable::RateLimit).parse::<u64>().unwrap_or(100);
    let rate_limit_per = env::get(AppVariable::RateLimitDuration).parse::<u64>().unwrap_or(10);
    let rate_limit_buffer = env::get(AppVariable::RateLimitBuffer).parse::<u64>().unwrap_or(1024);

    let server_port = server_port;
    Server::new(server_port)
        .enable_cors()
        .enable_session()
        .enable_rate_limit(rate_limit, Duration::from_secs(rate_limit_per), rate_limit_buffer)
        .run(routes::router(), app_state.clone())
        .await;

    Ok(())
}
