use levelcrush::{database, proc_macros::DatabaseRecord, util::unix_timestamp};
use sqlx::SqlitePool;

#[DatabaseRecord]
pub struct ChannelLogRecord {
    pub event_type: String,
    pub guild_id: u64,
    pub category_id: u64,
    pub category_name: String,
    pub channel_id: u64,
    pub channel_name: String,
    pub message_id: u64,
    pub message_timestamp: u64,
    pub member_id: u64,
    pub data: String,
}

pub async fn create(log: ChannelLogRecord, pool: &SqlitePool) {
    let query = sqlx::query_file!(
        "queries/channel_log_insert.sql",
        log.event_type,
        log.guild_id,
        log.category_id,
        log.category_name,
        log.channel_id,
        log.channel_name,
        log.message_id,
        log.message_timestamp,
        log.member_id,
        log.data,
        unix_timestamp()
    )
    .execute(pool)
    .await;

    database::log_error(query);
}
