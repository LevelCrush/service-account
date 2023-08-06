use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::project_str;
use levelcrush::{database, alias::RecordId};
use sqlx::SqlitePool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct ActivityTypeRecord {
    pub hash: i64,
    pub index: i64,
    pub name: String,
    pub description: String,
    pub icon_url: String,
}

#[DatabaseResult]
pub struct ActivityTypeSearchResult {
    pub id: RecordId,
    pub hash: i64,
}

pub async fn exists_bulk(hashes: &[i64], pool: &SqlitePool) -> HashMap<i64, RecordId> {
    if hashes.is_empty() {
        return HashMap::new();
    }
    let mut results = HashMap::new();

    let in_prepare_pos = vec!["?"; hashes.len()].join(",");
    let statement = project_str!("queries/activity_type_exist_multi.sql", in_prepare_pos);

    let mut query = sqlx::query_as::<_, ActivityTypeSearchResult>(statement.as_str());
    for hash in hashes.iter() {
        query = query.bind(*hash);
    }

    let query = query.fetch_all(pool).await;
    if let Ok(query_result) = query {
        for record in &query_result {
            results.insert(record.hash, record.id);
        }
    } else {
        database::log_error(query);
    }

    results
}

/// Send a bulk amount of activity type records to the database , insert when not found and update when found
pub async fn write(values: &[ActivityTypeRecord], pool: &SqlitePool) {
    if values.is_empty() {
        return;
    }
    // for every value we have in values, we need to have a patching VALUES() group
    let query_parameters = vec!["(?,?,?,?,?,?,?,?)"; values.len()];

    let query_parameters = query_parameters.join(", ");
    let statement = project_str!("queries/activity_types_write.sql", query_parameters);

    let mut query_builder = sqlx::query(statement.as_str());
    for data in values.iter() {
        query_builder = query_builder
            .bind(data.hash)
            .bind(data.name.as_str())
            .bind(data.description.as_str())
            .bind(data.icon_url.as_str())
            .bind(data.index)
            .bind(data.created_at)
            .bind(data.updated_at)
            .bind(data.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}
