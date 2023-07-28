use levelcrush::{database, macros::DatabaseRecordSerde};
use sqlx::SqlitePool;

#[derive(serde::Serialize, serde::Deserialize, Clone, Default, Debug, sqlx::FromRow, ts_rs::TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct SettingModeRecord {
    pub id: i64,
    pub leaderboard: i8,
    pub dashboard: i8,
    pub name: String,
    pub value: String,
    pub description: String,
    pub order: i32,
    pub created_at: u64,
    pub updated_at: u64,
    pub deleted_at: u64,
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
