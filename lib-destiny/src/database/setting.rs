use levelcrush::{database, macros::DatabaseRecord};
use sqlx::SqlitePool;

#[DatabaseRecord]
pub struct SettingModeRecord {
    pub leaderboard: i64,
    pub dashboard: i64,
    pub name: String,
    pub value: String,
    pub description: String,
    pub order: i64,
}

pub async fn modes(pool: &SqlitePool) -> Vec<SettingModeRecord> {
    let query = sqlx::query_file_as!(SettingModeRecord, "queries/settings_modes_get.sql")
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
