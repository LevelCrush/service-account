use levelcrush::{database, proc_macros::DatabaseResult, project_str, BigDecimal};
use sqlx::MySqlPool;

use super::leaderboard::LeaderboardEntryResult;

/// query the database and get rank info by
/// at the moment this just gets **everyone** in the network clans
/// this works for now but will need to be adjusted later for sure
pub async fn titles(display_name: &str, pool: &MySqlPool) -> Vec<LeaderboardEntryResult> {
    let query: Result<Vec<LeaderboardEntryResult>, sqlx::Error> = sqlx::query_file_as!(
        LeaderboardEntryResult,
        "queries/leaderboard_titles_rank.sql",
        display_name,
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

/// gets a leaderboard for raids
pub async fn raids(display_name: &str, pool: &MySqlPool) -> Vec<LeaderboardEntryResult> {
    let query = sqlx::query_file_as!(
        LeaderboardEntryResult,
        "queries/leaderboard_raids_rank.sql",
        display_name
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

pub async fn pvp_based(display_name: &str, modes: &[i32], pool: &MySqlPool) -> Vec<LeaderboardEntryResult> {
    if modes.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; modes.len()].join(",");
    let statement = project_str!("queries/leaderboard_pvp_rank.sql", prepared_pos);
    let mut query = sqlx::query_as::<_, LeaderboardEntryResult>(&statement);
    for mode in modes.iter() {
        query = query.bind(mode);
    }

    query = query.bind(display_name);

    let query = query.fetch_all(pool).await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn generic(display_name: &str, modes: &[i32], pool: &MySqlPool) -> Vec<LeaderboardEntryResult> {
    if modes.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; modes.len()].join(",");
    let statement = project_str!("queries/leaderboard_activities_rank.sql", prepared_pos);
    let mut query = sqlx::query_as::<_, LeaderboardEntryResult>(&statement);
    for mode in modes.iter() {
        query = query.bind(mode);
    }
    query = query.bind(display_name);

    let query = query.fetch_all(pool).await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
