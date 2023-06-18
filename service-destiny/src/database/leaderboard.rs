use levelcrush::{database, proc_macros::DatabaseResult, BigDecimal};
use sqlx::MySqlPool;

#[DatabaseResult]
pub struct LeaderboardEntryResult {
    pub display_name: String,
    pub amount: BigDecimal,
}

/// query the database and get leaderboard info by
/// at the moment this just gets **everyone** in the network clans
/// this works for now but will need to be adjusted later for sure
pub async fn titles(pool: &MySqlPool) -> Vec<LeaderboardEntryResult> {
    let query = sqlx::query_file_as!(LeaderboardEntryResult, "queries/leaderboard_titles.sql")
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
