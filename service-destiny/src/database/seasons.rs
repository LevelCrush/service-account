use levelcrush::database;
use levelcrush::macros::DatabaseRecord;
use levelcrush::project_str;
use lib_destiny::aliases::ManifestHash;
use sqlx::SqlitePool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct SeasonRecord {
    pub hash: ManifestHash,
    pub name: String,
    pub pass_hash: ManifestHash,
    pub number: i64,
    pub starts_at: i64,
    pub ends_at: i64,
}

pub async fn get(number: i64, pool: &SqlitePool) -> Option<SeasonRecord> {
    let query = sqlx::query_file_as!(SeasonRecord, "queries/season_get.sql", number)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_all_active(pool: &SqlitePool) -> Vec<SeasonRecord> {
    let query = sqlx::query_file_as!(SeasonRecord, "queries/season_get_all_active.sql")
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn read(hashes: &[u32], pool: &SqlitePool) -> HashMap<i64, SeasonRecord> {
    if hashes.is_empty() {
        return HashMap::new();
    }

    let prepared_pos = vec!["?"; hashes.len()].join(",");

    let statement = project_str!("queries/season_read.sql", prepared_pos);

    let mut query_builder = sqlx::query_as::<_, SeasonRecord>(&statement);
    for hash in hashes.iter() {
        query_builder = query_builder.bind(hash);
    }

    let query = query_builder.fetch_all(pool).await;
    if let Ok(query) = query {
        HashMap::from_iter(query.into_iter().map(|record| (record.hash, record)))
    } else {
        database::log_error(query);
        HashMap::new()
    }
}

pub async fn write(records: &[SeasonRecord], pool: &SqlitePool) {
    //

    let prepared_pos = vec!["(?,?,?,?,?,?,?,?,?)"; records.len()].join(",");

    let statement = project_str!("queries/season_write.sql", prepared_pos);

    let mut query_builder = sqlx::query(&statement);
    for record in records.iter() {
        query_builder = query_builder
            .bind(record.hash)
            .bind(record.name.as_str())
            .bind(record.pass_hash)
            .bind(record.number)
            .bind(record.starts_at)
            .bind(record.ends_at)
            .bind(record.created_at)
            .bind(record.updated_at)
            .bind(record.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}
