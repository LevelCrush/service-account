use levelcrush::types::destiny::InstanceId;
use levelcrush::{database, types::RecordId};
use levelcrush_macros::{DatabaseRecord, DatabaseResult};
use sqlx::MySqlPool;
use std::collections::HashMap;

#[DatabaseRecord]
pub struct InstanceRecord {
    pub instance_id: i64,
    pub occurred_at: u64,
    pub starting_phase_index: i32,
    pub started_from_beginning: i8,
    pub activity_hash: u32,
    pub activity_director_hash: u32,
    pub is_private: i8,
    pub completed: i8,
    pub completion_reasons: String,
}

#[DatabaseRecord]
pub struct InstanceMemberRecord {
    pub instance_id: i64,
    pub membership_id: i64,
    pub platform: i32,
    pub character_id: i64,
    pub class_name: String,
    pub class_hash: u32,
    pub emblem_hash: u32,
    pub light_level: i32,
    pub clan_name: String,
    pub clan_tag: String,
    pub completed: i8,
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
    pub platform: i32,
    pub timestamp: u64,
}

/// g et
pub async fn get(instance_id: InstanceId, pool: &MySqlPool) -> Option<InstanceRecord> {
    let query = sqlx::query_as!(
        InstanceRecord,
        r"
        SELECT instances.* FROM instances
        WHERE instances.instance_id = ?
        LIMIT 1
    ",
        instance_id
    )
    .fetch_optional(pool)
    .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_oldest(pool: &MySqlPool) -> Option<InstanceRecord> {
    let query = sqlx::query_as!(
        InstanceRecord,
        r"
            SELECT
                instances.* 
            FROM instances
            ORDER BY instances.occurred_at ASC
            LIMIT 1
        "
    )
    .fetch_optional(pool)
    .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_recent(pool: &MySqlPool) -> Option<InstanceRecord> {
    let query = sqlx::query_as!(
        InstanceRecord,
        r"
            SELECT
                instances.* 
            FROM instances
            ORDER BY instances.occurred_at DESC
            LIMIT 1
        "
    )
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
    start_date: u64,
    end_date: u64,
    limit: u64,
    pool: &MySqlPool,
) -> Vec<InstanceMissingProfileResult> {
    let query = sqlx::query_as!(
        InstanceMissingProfileResult,
        r"
        SELECT
            instance_members.membership_id,
            instance_members.platform,
            instances.occurred_at AS timestamp
        FROM instances
        INNER JOIN instance_members ON instances.instance_id = instance_members.instance_id
        LEFT JOIN members ON instance_members.membership_id = members.membership_id
        WHERE members.id IS NULL
        AND (instances.occurred_at > ? AND instances.occurred_at < ?)
        GROUP BY instance_members.membership_id, instance_members.platform, instances.occurred_at
        LIMIT ?
    ",
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
    pool: &MySqlPool,
) -> HashMap<InstanceId, InstanceGetCharactersResult> {
    let query = sqlx::query_as!(
        InstanceGetCharactersResult,
        r"
            SELECT
                instance_members.id,
                instance_members.membership_id,
                instance_members.character_id
            FROM instance_members   
            WHERE instance_members.instance_id = ?
        ",
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

pub async fn multi_get_members(instance_ids: &[InstanceId], pool: &MySqlPool) -> Vec<InstanceMemberRecord> {
    if instance_ids.is_empty() {
        return Vec::new();
    }

    let prepared_statement_pos = vec!["?"; instance_ids.len()].join(",");
    let statement = format!(
        r"
        SELECT 
            *
        FROM instance_members 
        WHERE instance_members.instance_id IN ({})
    ",
        prepared_statement_pos
    );

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
pub async fn write(record: &InstanceRecord, pool: &MySqlPool) {
    let query = sqlx::query!(
        r"
        INSERT INTO instances (
            `id`,
            `instance_id`,
            `occurred_at`,
            `starting_phase_index`,
            `started_from_beginning`,
            `activity_hash`,
            `activity_director_hash`,
            `is_private`,
            `completed`,
            `completion_reasons`,
            `created_at`,
            `updated_at`,
            `deleted_at`
        )
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?)
        ON DUPLICATE KEY UPDATE 
            `occurred_at` = VALUES(`occurred_at`),
            `starting_phase_index` = VALUES(`starting_phase_index`),
            `started_from_beginning` = VALUES(`started_from_beginning`),
            `activity_hash` = VALUES(`activity_hash`),
            `activity_director_hash` = VALUES(`activity_director_hash`),
            `is_private` = VALUES(`is_private`),
            `completed` = VALUES(`completed`),
            `completion_reasons` = VALUES(`completion_reasons`),
            `updated_at` =  VALUES(`created_at`),
            `deleted_at` = VALUES(`deleted_at`)
    ",
        record.id,
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
pub async fn write_members(values: &[InstanceMemberRecord], pool: &MySqlPool) {
    if values.is_empty() {
        return;
    }
    // for every value we have in values, we need to have a patching VALUES() group
    let query_parameters = vec!["(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)"; values.len()].join(",");

    let statement = format!(
        r"
        INSERT INTO instance_members (
            `id`,
            `instance_id`,
            `membership_id`,
            `platform`,
            `character_id`,
            `class_hash`,
            `class_name`,
            `emblem_hash`,
            `light_level`,
            `clan_name`,
            `clan_tag`,
            `completed`,
            `completion_reason`,
            `created_at`,
            `updated_at`,
            `deleted_at`
        )
        VALUES {}
        ON DUPLICATE KEY UPDATE
            `class_hash` = VALUES(`class_hash`),
            `class_name` = VALUES(`class_name`),
            `emblem_hash` = VALUES(`emblem_hash`),
            `light_level` = VALUES(`light_level`),
            `clan_name` = VALUES(`clan_name`),
            `clan_tag` = VALUES(`clan_tag`),
            `completed` = VALUES(`completed`),
            `completion_reason` = VALUES(`completion_reason`),
            `updated_at` = VALUES(`created_at`),
            `deleted_at` = VALUES(`deleted_at`)
    ",
        query_parameters
    );

    let mut query_builder = sqlx::query(statement.as_str());

    for value in values.iter() {
        query_builder = query_builder
            .bind(value.id)
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

pub async fn multi_get(instance_ids: &[InstanceId], pool: &MySqlPool) -> Vec<InstanceRecord> {
    if instance_ids.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; instance_ids.len()].join(",");
    let statement = format!(
        r"
        SELECT
            instances.* 
        FROM instances 
        WHERE instances.instance_id IN ({})
    ",
        prepared_pos
    );

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
