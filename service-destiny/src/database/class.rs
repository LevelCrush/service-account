use levelcrush::database;
use levelcrush::types::{destiny::ManifestHash, RecordId};
use levelcrush_macros::{DatabaseRecord, DatabaseResult};
use sqlx::MySqlPool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct ClassRecord {
    pub hash: u32,
    pub index: u32,

    #[sqlx(rename = "type")]
    pub class_type: u8,

    pub name: String,
}

#[DatabaseResult]
pub struct ClassSearchResult {
    pub id: RecordId,
    pub hash: u32,
}

pub async fn exists_bulk(hashes: &[ManifestHash], pool: &MySqlPool) -> HashMap<ManifestHash, RecordId> {
    if hashes.is_empty() {
        return HashMap::new();
    }
    let mut results = HashMap::new();

    let in_prepare_pos = vec!["?"; hashes.len()].join(",");
    let statement = format!(
        r"
        SELECT
            classes.id,
            classes.hash
        FROM classes
        WHERE classes.hash IN ({})
    ",
        in_prepare_pos
    );

    let mut query = sqlx::query_as::<_, ClassSearchResult>(statement.as_str());
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

pub async fn write(values: &[ClassRecord], pool: &MySqlPool) {
    if values.is_empty() {
        return;
    }
    // for every value we have in values, we need to have a patching VALUES() group
    let query_parameters = vec!["(?,?,?,?,?,?,?,?)"; values.len()];

    let query_parameters = query_parameters.join(", ");
    let statement = format!(
        r"
            INSERT INTO classes (
                `id`,
                `hash`,
                `index`,
                `type`,
                `name`,
                `created_at`,
                `updated_at`,
                `deleted_at`
            )
            VALUES {} 
            ON DUPLICATE KEY UPDATE
                `name` = VALUES(`name`),
                `type` = VALUES(`type`),
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
            .bind(data.index)
            .bind(data.class_type)
            .bind(data.name.as_str())
            .bind(data.created_at)
            .bind(data.updated_at)
            .bind(data.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}
