use levelcrush::{database, macros::DatabaseRecordSerde};
use sqlx::MySqlPool;

#[DatabaseRecordSerde]
pub struct SettingModeRecord {
    pub name: String,
    pub value: String,
    pub order: i32,
}

pub async fn modes(pool: &MySqlPool) -> Vec<SettingModeRecord> {
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
