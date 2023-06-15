use levelcrush::macros::{DatabaseRecord, DatabaseResult, DatabaseResultSerde};
use levelcrush::project_str;
use levelcrush::types::destiny::InstanceId;
use levelcrush::types::{destiny::ManifestHash, RecordId};
use levelcrush::{database, BigDecimal};
use sqlx::MySqlPool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct ActivityRecord {
    pub activity_type: u32,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub fireteam_min_size: u32,
    pub fireteam_max_size: u32,
    pub max_players: u32,
    pub requires_guardian_oath: bool,
    pub is_pvp: bool,
    pub matchmaking_enabled: bool,
    pub hash: u32,
    pub index: u32,
}

#[DatabaseResult]
pub struct ActivitySearchResult {
    pub id: RecordId,
    pub hash: u32,
}

#[DatabaseResultSerde]
pub struct ActivityInstanceResult {
    pub activity_name: String,
    pub activity_description: String,
    pub activity_hash: u32,
    pub director_activity_name: String,
    pub director_activity_description: String,
    pub director_activity_hash: u32,
    pub total: i64,
    pub total_completed: BigDecimal,
}

/// sends a set of u32 hashes into a query to check for existence.
/// Returns a HashMap<u32, i32> which represents HashMap<hash,record_id>
pub async fn exists_bulk(hashes: &[u32], pool: &MySqlPool) -> HashMap<ManifestHash, RecordId> {
    if hashes.is_empty() {
        return HashMap::new();
    }
    let mut results = HashMap::new();

    let in_prepare_pos = vec!["?"; hashes.len()].join(",");
    let statement = project_str!("queries/activity_exist_multi.sql", in_prepare_pos);

    // prepare statement and then bind every hash
    let mut query = sqlx::query_as::<_, ActivitySearchResult>(statement.as_str());
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
pub async fn write(values: &[ActivityRecord], pool: &MySqlPool) {
    if values.is_empty() {
        return;
    }
    // for every value we have in values, we need to have a patching VALUES() group
    let query_parameters = vec!["(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"; values.len()];

    let query_parameters = query_parameters.join(", ");
    let statement = project_str!("queries/activity_write.sql", query_parameters);

    let mut query_builder = sqlx::query(statement.as_str());
    for data in values.iter() {
        query_builder = query_builder
            .bind(data.id)
            .bind(data.hash)
            .bind(data.index)
            .bind(data.activity_type)
            .bind(data.name.as_str())
            .bind(data.description.as_str())
            .bind(data.image_url.as_str())
            .bind(data.fireteam_min_size)
            .bind(data.fireteam_max_size)
            .bind(data.max_players)
            .bind(data.is_pvp)
            .bind(data.matchmaking_enabled)
            .bind(data.requires_guardian_oath)
            .bind(data.created_at)
            .bind(data.updated_at)
            .bind(data.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}

pub async fn from_instances(instance_ids: &[InstanceId], pool: &MySqlPool) -> Vec<ActivityInstanceResult> {
    if instance_ids.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; instance_ids.len()].join(",");
    let statement = project_str!("queries/activity_from_instances.sql", prepared_pos);

    let mut query_builder = sqlx::query_as::<_, ActivityInstanceResult>(&statement);
    for instance in instance_ids.iter() {
        query_builder = query_builder.bind(instance);
    }

    let query = query_builder.fetch_all(pool).await;
    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
