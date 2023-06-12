use levelcrush::{database, types::RecordId};
use levelcrush_macros::{DatabaseRecord, DatabaseResult};
use sqlx::MySqlPool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct ActivityTypeRecord {
    pub hash: u32,
    pub index: u32,
    pub name: String,
    pub description: String,
    pub icon_url: String,
}

#[DatabaseResult]
pub struct ActivityTypeSearchResult {
    pub id: RecordId,
    pub hash: u32,
}

pub async fn exists_bulk(hashes: &[u32], pool: &MySqlPool) -> HashMap<u32, RecordId> {
    if hashes.is_empty() {
        return HashMap::new();
    }
    let mut results = HashMap::new();

    let in_prepare_pos = vec!["?"; hashes.len()].join(",");
    let statement = format!(
        r"
        SELECT
            activity_types.id,
            activity_types.hash
        FROM activity_types
        WHERE activity_types.hash IN ({})
    ",
        in_prepare_pos
    );

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
pub async fn write(values: &[ActivityTypeRecord], pool: &MySqlPool) {
    if values.is_empty() {
        return;
    }
    // for every value we have in values, we need to have a patching VALUES() group
    let query_parameters = vec!["(?,?,?,?,?,?,?,?,?)"; values.len()];

    let query_parameters = query_parameters.join(", ");
    let statement = format!(
        r"
            INSERT INTO activity_types (`id`, `hash`, `name`, `description`, `icon_url`, `index`,`created_at`, `updated_at`, `deleted_at`)
            VALUES {}
            ON DUPLICATE KEY UPDATE
                `name` = VALUES(`name`),
                `description` = VALUES(`description`),
                `icon_url` = VALUES(`icon_url`),
                `index` = VALUES(`index`),
                `updated_at` = VALUES(`created_at`),
                `deleted_at` = VALUES(`deleted_at`)
        ",
        query_parameters
    );

    let mut query_builder = sqlx::query(statement.as_str());
    for data in values.iter() {
        query_builder = query_builder
            .bind(data.id)
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
