use levelcrush::{database, proc_macros::DatabaseResult, types::UnixTimestamp, BigDecimal};
use sqlx::MySqlPool;

#[DatabaseResult]
pub struct CategoryActiveUserResult {
    pub member_id: u64,
    pub message_timestamp: BigDecimal,
}

pub async fn active_users(
    category_name: &str,
    timestamp: UnixTimestamp,
    pool: &MySqlPool,
) -> Vec<CategoryActiveUserResult> {
    let query = sqlx::query_file_as!(
        CategoryActiveUserResult,
        "queries/category_active_users.sql",
        category_name,
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
