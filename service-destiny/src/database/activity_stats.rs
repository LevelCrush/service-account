use levelcrush::database;
use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::project_str;
use levelcrush::types::destiny::{CharacterId, InstanceId};
use levelcrush::types::RecordId;
use sqlx::SqlitePool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct ActivityStatRecord {
    pub membership_id: i64,
    pub character_id: i64,
    pub instance_id: i64,
    pub activity_hash: u32,
    pub activity_hash_director: u32,
    pub name: String,
    pub value: f64,
    pub value_display: String,
}

#[DatabaseResult]
pub struct ActivityStatExistingResult {
    pub id: RecordId,
    pub instance_id: i64,
}

#[DatabaseResult]
pub struct ActivityStatResult {
    pub membership_id: i64,
    pub instance_id: i64,
    pub value: f64,
    pub value_display: String,
}

pub enum StatFilter {
    Value(f64),
    ValueDisplay(String),
    None,
}

pub async fn existing(
    character_id: CharacterId,
    instance_ids: &[InstanceId],
    pool: &SqlitePool,
) -> HashMap<i64, RecordId> {
    // make sure we have instance ids otherwise return now
    if instance_ids.is_empty() {
        return HashMap::new();
    }

    let prepared_statement_ins = vec!["?"; instance_ids.len()].join(",");
    let statement = project_str!("queries/activity_stats_existing.sql", prepared_statement_ins);

    let mut query_builder = sqlx::query_as::<_, ActivityStatExistingResult>(statement.as_str());
    // bind character id first
    query_builder = query_builder.bind(character_id);
    for instance in instance_ids.iter() {
        query_builder = query_builder.bind(*instance);
    }

    let query = query_builder.fetch_all(pool).await;
    let mut results = HashMap::new();
    if let Ok(query) = query {
        for record in query.iter() {
            results
                .entry(record.instance_id)
                .and_modify(|v| *v = record.id)
                .or_insert(record.id);
        }
    } else {
        database::log_error(query);
    }

    results
}

pub async fn write(values: &[ActivityStatRecord], pool: &SqlitePool) {
    if values.is_empty() {
        return;
    }

    let query_parameters = vec!["(?,?, ?,?,?,?,?,?,?)"; values.len()].join(",");
    let statement = project_str!("queries/activity_stats_insert.sql", query_parameters);

    let mut query_builder = sqlx::query(statement.as_str());
    for record in values.iter() {
        query_builder = query_builder
            .bind(record.membership_id)
            .bind(record.character_id)
            .bind(record.instance_id)
            .bind(record.name.clone())
            .bind(record.value)
            .bind(record.value_display.clone())
            .bind(record.created_at)
            .bind(record.updated_at)
            .bind(record.deleted_at)
    }

    // execute query
    let query = query_builder.execute(pool).await;
    database::log_error(query);
}

pub async fn get_instances(
    stat: &str,
    membership_id: i64,
    instance_ids: &[InstanceId],
    value_filter: StatFilter,
    pool: &SqlitePool,
) -> Vec<ActivityStatResult> {
    if instance_ids.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; instance_ids.len()].join(",");
    let filter_str = match &value_filter {
        StatFilter::Value(_) => "AND member_activity_stats.value = ?".to_string(),
        StatFilter::ValueDisplay(_) => "AND member_activity_stats.value_display = ?".to_string(),
        StatFilter::None => String::new(),
    };

    let statement = project_str!("queries/activity_stats_from_instances.sql", filter_str, prepared_pos);
    let mut query_builder = sqlx::query_as::<_, ActivityStatResult>(&statement)
        .bind(stat)
        .bind(membership_id);

    query_builder = match value_filter {
        StatFilter::Value(data) => query_builder.bind(data),
        StatFilter::ValueDisplay(data) => query_builder.bind(data),
        StatFilter::None => query_builder,
    };

    for instance_id in instance_ids.iter() {
        query_builder = query_builder.bind(instance_id);
    }

    let query = query_builder.fetch_all(pool).await;
    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}
