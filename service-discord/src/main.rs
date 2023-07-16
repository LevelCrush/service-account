use app::state::AppState;
use env::AppVariable;
use levelcrush::{server::Server, tokio, tracing};
use std::time::Duration;

mod app;
mod database;
mod env;
mod routes;
mod sync;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    levelcrush::env();

    let server_port = env::get(AppVariable::ServerPort).parse::<u16>().unwrap_or(3000);
    let app_state = AppState::new().await;

    let server_port = server_port;
    Server::new(server_port)
        .enable_cors()
        .run(routes::router(), app_state.clone())
        .await;
}
