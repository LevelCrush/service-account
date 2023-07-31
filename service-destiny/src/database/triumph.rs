use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::project_str;
use levelcrush::{database, types::destiny::MembershipId};
use lib_destiny::aliases::ManifestHash;
use sqlx::SqlitePool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct TriumphRecord {
    pub hash: ManifestHash,
    pub name: String,
    pub description: String,
    pub title: String,
    pub is_title: i64,
    pub gilded: i64,
}

#[DatabaseRecord]
pub struct MemberTriumphRecord {
    pub membership_id: MembershipId,
    pub hash: ManifestHash,
    pub state: i64,
    pub times_completed: i64,
}

#[DatabaseResult]
pub struct TriumphTitleResult {
    pub membership_id: MembershipId,
    pub title: String,
    pub has_gilded: i64,
    pub total_gilds: i64,
    pub can_equip: i64,
    pub can_equip_gilded: i64,
}

pub async fn member_read(
    membership_id: MembershipId,
    hashes: &[ManifestHash],
    pool: &SqlitePool,
) -> HashMap<ManifestHash, MemberTriumphRecord> {
    if hashes.is_empty() {
        return HashMap::new();
    }

    let prepared_pos = vec!["?"; hashes.len()].join(",");
    let statement = project_str!("queries/member_triumphs_read.sql", prepared_pos);
    let mut query_builder = sqlx::query_as::<_, MemberTriumphRecord>(&statement).bind(membership_id);

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

pub async fn read(hashes: &[ManifestHash], pool: &SqlitePool) -> HashMap<ManifestHash, TriumphRecord> {
    if hashes.is_empty() {
        return HashMap::new();
    }

    let prepared_pos = vec!["?"; hashes.len()].join(",");
    let statement = project_str!("queries/triumphs_read.sql", prepared_pos);

    let mut query_builder = sqlx::query_as::<_, TriumphRecord>(&statement);
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

pub async fn write(records: &[TriumphRecord], pool: &SqlitePool) {
    let prepared_pos = vec!["(?,?,?,?,?,?,?,?,?,?)"; records.len()].join(",");

    let statement = project_str!("queries/triumphs_write.sql", prepared_pos);

    let mut query_builder = sqlx::query(&statement);
    for record in records.iter() {
        query_builder = query_builder
            .bind(record.hash)
            .bind(record.name.clone())
            .bind(record.description.clone())
            .bind(record.title.clone())
            .bind(record.is_title)
            .bind(record.gilded)
            .bind(record.created_at)
            .bind(record.updated_at)
            .bind(record.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}

pub async fn member_write(records: &[MemberTriumphRecord], pool: &SqlitePool) {
    let prepared_pos = vec!["(?,?,?,?,?,?,?)"; records.len()].join(",");

    let statement = project_str!("queries/member_triumphs_write.sql", prepared_pos);

    let mut query_builder = sqlx::query(&statement);
    for record in records.iter() {
        query_builder = query_builder
            .bind(record.hash)
            .bind(record.membership_id)
            .bind(record.state)
            .bind(record.times_completed)
            .bind(record.created_at)
            .bind(record.updated_at)
            .bind(record.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}

pub async fn member_titles(membership_id: MembershipId, pool: &SqlitePool) -> Vec<TriumphTitleResult> {
    let query = sqlx::query_file_as!(TriumphTitleResult, "queries/member_titles_get.sql", membership_id)
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
