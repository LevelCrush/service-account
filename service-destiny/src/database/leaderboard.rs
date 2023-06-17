use levelcrush::{database, proc_macros::DatabaseResultSerde, BigDecimal};
use sqlx::MySqlPool;

#[DatabaseResultSerde]
pub struct LeaderboardEntry {
    pub display_name: String,
    pub amount: BigDecimal,
}

/// query the database and get leaderboard info by
/// at the moment this just gets **everyone** in the network clans
/// this works for now but will need to be adjusted later for sure
pub async fn seals(pool: &MySqlPool) -> Vec<LeaderboardEntry> {
    let query = sqlx::query_file_as!(LeaderboardEntry, "queries/leaderboard_seals.sql")
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
