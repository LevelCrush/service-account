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

    let server_port = server_port;
    Server::new(server_port)
        .enable_cors()
        .enable_session()
        .run(routes::router(), app_state.clone())
        .await;

    Ok(())
}
