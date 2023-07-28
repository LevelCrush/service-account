use levelcrush::{database, proc_macros::DatabaseResult, BigDecimal};
use sqlx::SqlitePool;

#[DatabaseResult]
pub struct ChannelActiveUserResult {
    pub member_id: u64,
    pub message_timestamp: BigDecimal,
}

pub async fn active_users(
    guild: &str,
    channel: &str,
    timestamp: u64,
    pool: &SqlitePool,
) -> Vec<ChannelActiveUserResult> {
    let query = sqlx::query_file_as!(
        ChannelActiveUserResult,
        "queries/channel_active_users.sql",
        guild,
        channel,
        timestamp
    )
    .fetch_all(pool)
    .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
