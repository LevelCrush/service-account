use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::project_str;
use levelcrush::alias::destiny::InstanceId;
use levelcrush::{database, alias::RecordId};
use lib_destiny::aliases::ManifestHash;
use sqlx::SqlitePool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct InstanceRecord {
    pub instance_id: i64,
    pub occurred_at: i64,
    pub starting_phase_index: i64,
    pub started_from_beginning: i64,
    pub activity_hash: ManifestHash,
    pub activity_director_hash: ManifestHash,
    pub is_private: i64,
    pub completed: i64,
    pub completion_reasons: String,
}

#[DatabaseRecord]
pub struct InstanceMemberRecord {
    pub instance_id: i64,
    pub membership_id: i64,
    pub platform: i64,
    pub character_id: i64,
    pub class_name: String,
    pub class_hash: ManifestHash,
    pub emblem_hash: ManifestHash,
    pub light_level: i64,
    pub clan_name: String,
    pub clan_tag: String,
    pub completed: i64,
    pub completion_reason: String,
}

#[DatabaseResult]
pub struct InstanceGetCharactersResult {
    pub id: RecordId,
    pub membership_id: i64,
    pub character_id: i64,
}

#[DatabaseResult]
pub struct InstanceMissingProfileResult {
    pub membership_id: i64,
    pub platform: i64,
    pub timestamp: i64,
}

/// g et
pub async fn get(instance_id: InstanceId, pool: &SqlitePool) -> Option<InstanceRecord> {
    let query = sqlx::query_file_as!(InstanceRecord, "queries/instance_get.sql", instance_id)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_oldest(pool: &SqlitePool) -> Option<InstanceRecord> {
    let query = sqlx::query_file_as!(InstanceRecord, "queries/instance_get_oldest.sql")
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_recent(pool: &SqlitePool) -> Option<InstanceRecord> {
    let query = sqlx::query_file_as!(InstanceRecord, "queries/instance_get_recent.sql")
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_members_missing_profile(
    start_date: i64,
    end_date: i64,
    limit: i64,
    pool: &SqlitePool,
) -> Vec<InstanceMissingProfileResult> {
    let query = sqlx::query_file_as!(
        InstanceMissingProfileResult,
        "queries/instance_members_missing_profile.sql",
        start_date,
        end_date,
        limit,
    )
    .fetch_all(pool)
    .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn get_members(
    instance_id: InstanceId,
    pool: &SqlitePool,
) -> HashMap<InstanceId, InstanceGetCharactersResult> {
    let query = sqlx::query_file_as!(
        InstanceGetCharactersResult,
        "queries/instance_get_members.sql",
        instance_id
    )
    .fetch_all(pool)
    .await;

    // I am pretty sure this logic here can be written better,  but for now it works
    let mut results = HashMap::new();
    if let Ok(query_results) = query {
        for record in query_results.iter() {
            results.insert(record.character_id, record.clone());
        }
    } else {
        database::log_error(query);
    }

    results
}

pub async fn multi_get_members(instance_ids: &[InstanceId], pool: &SqlitePool) -> Vec<InstanceMemberRecord> {
    if instance_ids.is_empty() {
        return Vec::new();
    }

    let prepared_statement_pos = vec!["?"; instance_ids.len()].join(",");
    let statement = project_str!("queries/instance_multi_get_members.sql", prepared_statement_pos);

    let mut query_builder = sqlx::query_as::<_, InstanceMemberRecord>(&statement);
    for instance_id in instance_ids.iter() {
        query_builder = query_builder.bind(*instance_id);
    }

    let query = query_builder.fetch_all(pool).await;
    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

/// write an instance record to the database. This can be either an insert or update depending on if the db finds duplicate keys
pub async fn write(record: &InstanceRecord, pool: &SqlitePool) {
    let query = sqlx::query_file!(
        "queries/instance_write.sql",
        record.instance_id,
        record.occurred_at,
        record.starting_phase_index,
        record.started_from_beginning,
        record.activity_hash,
        record.activity_director_hash,
        record.is_private,
        record.completed,
        record.completion_reasons,
        record.created_at,
        record.updated_at,
        record.deleted_at,
    )
    .execute(pool)
    .await;

    database::log_error(query);
}

/// write instance characters to the database in bulk
pub async fn write_members(values: &[InstanceMemberRecord], pool: &SqlitePool) {
    if values.is_empty() {
        return;
    }
    // for every value we have in values, we need to have a patching VALUES() group
    let query_parameters = vec!["(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"; values.len()].join(",");

    let statement = project_str!("queries/instance_members_write.sql", query_parameters);
    let mut query_builder = sqlx::query(statement.as_str());

    for value in values.iter() {
        query_builder = query_builder
            .bind(value.instance_id)
            .bind(value.membership_id)
            .bind(value.platform)
            .bind(value.character_id)
            .bind(value.class_hash)
            .bind(value.class_name.as_str())
            .bind(value.emblem_hash)
            .bind(value.light_level)
            .bind(value.clan_name.as_str())
            .bind(value.clan_tag.as_str())
            .bind(value.completed)
            .bind(value.completion_reason.as_str())
            .bind(value.created_at)
            .bind(value.updated_at)
            .bind(value.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}

pub async fn multi_get(instance_ids: &[InstanceId], pool: &SqlitePool) -> Vec<InstanceRecord> {
    if instance_ids.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; instance_ids.len()].join(",");
    let statement = project_str!("queries/instance_multi_get.sql", prepared_pos);

    let mut query_builder = sqlx::query_as::<_, InstanceRecord>(&statement);
    for instance_id in instance_ids.iter() {
        query_builder = query_builder.bind(instance_id);
    }

    let query = query_builder.fetch_all(pool).await;

    if let Ok(data) = query {
        data
    } else {
        database::log_error(query);
        Vec::new()
    }
}
