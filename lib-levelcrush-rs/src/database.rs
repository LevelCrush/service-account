use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions},
    ConnectOptions, MySqlPool,
};
use std::{str::FromStr, time::Duration};
use tracing::log::LevelFilter;

/// connects to the application database based off .env specific variables
pub async fn connect() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_default();
    let mut database_options = MySqlConnectOptions::from_str(database_url.as_str()).unwrap();
    database_options.log_statements(LevelFilter::Off);
    database_options.log_slow_statements(LevelFilter::Warn, Duration::from_secs(5));

    let max_connections = std::env::var("DATABASE_CONNECTIONS_MAX")
        .unwrap_or_default()
        .parse::<u32>()
        .unwrap_or(10);

    tracing::info!(
        "Allowing a maximum of {} total connections to the database",
        max_connections
    );

    MySqlPoolOptions::new()
        .max_connections(max_connections)
        .connect_with(database_options)
        .await
        .expect("Could not make database connection")

    // connect to database
    /*
    MySqlPool::connect_with(database_options)
        .await
        .expect("Could not make database connection") */
}

pub fn log_error<T>(query: Result<T, sqlx::Error>) {
    if let Err(query) = query {
        tracing::error!("{}", query);
        //  panic!("Figuring out this error");
    }
}
