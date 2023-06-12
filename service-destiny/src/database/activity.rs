use levelcrush::types::destiny::InstanceId;
use levelcrush::types::{destiny::ManifestHash, RecordId};
use levelcrush::{database, BigDecimal};
use levelcrush_macros::{DatabaseRecord, DatabaseResult, DatabaseResultSerde};
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
    let statement = format!(
        r"
            SELECT 
                activities.id,
                activities.hash
            FROM activities
            WHERE activities.hash IN ({})
        ",
        in_prepare_pos
    );

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
    let statement = format!(
        r"
            INSERT INTO activities (
                `id`, 
                `hash`, 
                `index`, 
                `activity_type`,
                `name`, 
                `description`, 
                `image_url`, 
                `fireteam_min_size`,
                `fireteam_max_size`,
                `max_players`,
                `requires_guardian_oath`,
                `is_pvp`,
                `matchmaking_enabled`,
                `created_at`,
                `updated_at`, 
                `deleted_at`
            )
            VALUES {}
            ON DUPLICATE KEY UPDATE
                `name` = VALUES(`name`),
                `description` = VALUES(`description`),
                `image_url` = VALUES(`image_url`),
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
    let statement = format!(
        r"
        SELECT
            COALESCE(activities.name, 'Classified') AS activity_name,
            COALESCE(activities.description, 'N/A') AS activity_description,
            instances.activity_hash AS activity_hash,
            COALESCE(director_activity.name, 'Classified') as director_activity_name,
            COALESCE(director_activity.description,'N/A') AS director_activity_description,
            instances.activity_director_hash AS director_activity_hash,
            COUNT(DISTINCT instances.instance_id) AS total,
            SUM(instances.completed) AS total_completed
        FROM instances
        LEFT JOIN activities ON instances.activity_hash = activities.hash
        LEFT JOIN activities AS director_activity ON instances.activity_director_hash = director_activity.hash
        WHERE instances.instance_id IN ({})
        GROUP BY  instances.activity_hash, instances.activity_director_hash
    ",
        prepared_pos
    );

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
