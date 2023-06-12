use levelcrush::cache::MemoryCache;
use levelcrush::server::Server;
use levelcrush::{database, tokio};
use sqlx::MySqlPool;

use crate::env::AppVariable;

mod env;
mod routes;

#[derive(Clone, Debug)]
pub struct AppState {
    pub database: MySqlPool,
    pub cache: MemoryCache<String>,
}

impl AppState {
    /// Construct an app state
    ///
    /// Note: This will create a new database pool as well as a new bungie client
    pub async fn new() -> AppState {
        let database = database::connect().await;
        let cache = MemoryCache::new();
        AppState { database, cache }
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    levelcrush::env();

    let server_port = env::get(AppVariable::ServerPort).parse::<u16>().unwrap_or(3000);
    let app_state = AppState::new().await;

    Server::new(server_port)
        .enable_cors()
        .run(routes::router(), app_state)
        .await;
}
