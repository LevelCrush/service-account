use levelcrush::{database, proc_macros::DatabaseResult, project_str, BigDecimal};
use sqlx::MySqlPool;

#[DatabaseResult]
pub struct LeaderboardEntryResult {
    pub display_name: String,
    pub amount: BigDecimal,
    pub standing: u64,
    pub percent_distance: f64,
    pub percent_ranking: f64,
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

/// gets a leaderboard for raids
pub async fn raids(pool: &MySqlPool) -> Vec<LeaderboardEntryResult> {
    let query = sqlx::query_file_as!(LeaderboardEntryResult, "queries/leaderboard_raids.sql")
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn generic(modes: &[i32], pool: &MySqlPool) -> Vec<LeaderboardEntryResult> {
    if modes.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; modes.len()].join(",");
    let statement = project_str!("queries/leaderboard_activities.sql", prepared_pos);
    let mut query = sqlx::query_as::<_, LeaderboardEntryResult>(&statement);
    for mode in modes.iter() {
        query = query.bind(mode);
    }

    let query = query.fetch_all(pool).await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
