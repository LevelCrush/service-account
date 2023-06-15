pub mod cache;
pub mod macros;
pub mod retry_lock;
pub mod task_manager;
pub mod types;
pub mod util;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "database")]
pub mod queries;

#[cfg(feature = "server")]
pub mod server;

#[cfg(all(feature = "server", feature = "session", feature = "database"))]
pub mod mysql_session;

// rexports
pub use anyhow;
pub use chrono;
pub use dotenvy;
pub use dotenvy_macro;
pub use futures;
pub use levelcrush_macros as proc_macros;
pub use rand;
pub use serde;
pub use tokio;
pub use tracing;
pub use uuid;

#[cfg(feature = "server")]
pub use axum;

#[cfg(feature = "session")]
pub use axum_sessions;

#[cfg(feature = "database")]
pub use {bigdecimal, bigdecimal::BigDecimal, sqlx, sqlx::MySql, sqlx::MySqlPool};

/// setups tracing and loads settings from the local .env file
pub fn env() {
    // merge env file into std::env
    dotenvy::dotenv().ok();

    // setup better tracing
    tracing_subscriber::fmt::init();
}

pub fn hello() {
    tracing::info!("Heloo!");
}
