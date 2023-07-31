use crate::app::state::AppState;
use levelcrush::{anyhow, database, tracing};

pub async fn run() -> anyhow::Result<()> {
    tracing::info!("Connecting to database");
    let database = database::connect().await;

    tracing::info!("Purging database");
    sqlx::query_file!("schema/purge.sql").execute(&database).await?;

    Ok(())
}
