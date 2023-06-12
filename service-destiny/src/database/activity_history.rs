use levelcrush::bigdecimal::{BigDecimal, ToPrimitive};
use levelcrush::types::destiny::MembershipId;
use levelcrush::types::{destiny::CharacterId, destiny::InstanceId, RecordId, UnixTimestamp};
use levelcrush::MySqlPool;
use levelcrush::{database, tracing};
use levelcrush_macros::{DatabaseRecord, DatabaseResult};
use std::collections::HashMap;

#[DatabaseRecord]
pub struct ActivityHistoryRecord {
    pub membership_id: i64,
    pub character_id: i64,
    pub platform_played: i32,
    pub activity_hash: u32,
    pub activity_hash_director: u32,
    pub instance_id: i64,
    pub mode: i32,
    pub modes: String,
    pub private: i8,
    pub occurred_at: u64,
}

#[DatabaseResult]
pub struct ActivityHistoryExistingResult {
    pub id: RecordId,
    pub instance_id: i64,
}

#[DatabaseResult]
pub struct ActivityHistoryLastEntryResult {
    pub timestamp: BigDecimal,
}

#[DatabaseResult]
pub struct ActivityInstanceResult {
    pub instance_id: i64,
}

/// returns a hash map (key = (character_id, instance id), value = record id) of existing records that match the instance ids passed tied to the passed character id
pub async fn existing(
    character_id: CharacterId,
    instance_ids: &[InstanceId],
    pool: &MySqlPool,
) -> HashMap<(CharacterId, InstanceId), RecordId> {
    if instance_ids.is_empty() {
        return HashMap::new();
    }

    let prepared_statement_ins = vec!["?"; instance_ids.len()].join(",");
    let statement = format!(
        r"
        SELECT member_activities
            member_activities.id,
            member_activities.instance_id
        FROM member_activities
        WHERE member_activities.character_id = ?
        AND member_activities.instance_id IN ({})
        ",
        prepared_statement_ins
    );

    let mut query = sqlx::query_as::<_, ActivityHistoryExistingResult>(statement.as_str());
    query = query.bind(character_id);
    for instance in instance_ids.iter() {
        query = query.bind(*instance);
    }
    let query = query.fetch_all(pool).await;

    let mut results = HashMap::new();
    if let Ok(query) = query {
        for record in query.iter() {
            results
                .entry((character_id, record.instance_id))
                .and_modify(|v| *v = record.id)
                .or_insert(record.id);
        }
    }

    results
}

/// queries the database for the most recent timestamp of the activity that the character ran
pub async fn last_activity_timestamp(character_id: CharacterId, pool: &MySqlPool) -> UnixTimestamp {
    let query = sqlx::query_as!(
        ActivityHistoryLastEntryResult,
        r"
        SELECT 
            COALESCE(MAX(member_activities.occurred_at), 0) AS  timestamp
        FROM member_activities 
        WHERE member_activities.character_id = ?
        LIMIT 1
    ",
        character_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(query) = query {
        query.timestamp.to_u64().unwrap_or_default()
    } else {
        0
    }
}

/// write bulk amount of activity history records to the database
/// will insert or update automatically
pub async fn write(values: &[ActivityHistoryRecord], pool: &MySqlPool) {
    if values.is_empty() {
        return;
    }

    let prepared_statement_pos = vec!["(?,?,?,?,?,?,?,?,?,?,?,?,?,?)"; values.len()].join(",");

    let statement = format!(
        r"
        INSERT INTO member_activities 
        (
            `id`,
            `membership_id`,
            `character_id`,
            `platform_played`,
            `activity_hash`,
            `activity_hash_director`,
            `instance_id`,
            `mode`,
            `modes`,
            `private`,
            `occurred_at`,
            `created_at`,
            `updated_at`,
            `deleted_at`
        )
        VALUES {}
        ON DUPLICATE KEY UPDATE
            `platform_played` = VALUES(`platform_played`),
            `activity_hash` = VALUES(`activity_hash`),
            `activity_hash_director` = VALUES(`activity_hash_director`),
            `mode` = VALUES(`mode`),
            `modes` = VALUES(`modes`),
            `occurred_at` = VALUES(`occurred_at`),
            `updated_at` = VALUES(`created_at`),
            `deleted_at` = VALUES(`deleted_at`)
    ",
        prepared_statement_pos
    );

    let mut query_builder = sqlx::query(statement.as_str());
    for data in values.iter() {
        query_builder = query_builder
            .bind(data.id)
            .bind(data.membership_id)
            .bind(data.character_id)
            .bind(data.platform_played)
            .bind(data.activity_hash)
            .bind(data.activity_hash_director)
            .bind(data.instance_id)
            .bind(data.mode)
            .bind(data.modes.clone())
            .bind(data.private)
            .bind(data.occurred_at)
            .bind(data.created_at)
            .bind(data.updated_at)
            .bind(data.deleted_at)
    }

    // execute the query
    let result = query_builder.execute(pool).await;
    if result.is_err() {
        let err = result.err().unwrap();
        tracing::error!("{}", err);
    }
}

pub async fn get_oldest(pool: &MySqlPool) -> Option<ActivityHistoryRecord> {
    let query = sqlx::query_as!(
        ActivityHistoryRecord,
        r"
        SELECT 
            * 
        FROM member_activities
        ORDER BY member_activities.occurred_at ASC
        LIMIT 1
    ",
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

pub async fn get_recent(pool: &MySqlPool) -> Option<ActivityHistoryRecord> {
    let query = sqlx::query_as!(
        ActivityHistoryRecord,
        r"
        SELECT 
            * 
        FROM member_activities
        ORDER BY member_activities.occurred_at DESC
        LIMIT 1
    ",
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

pub async fn member(
    membership_id: MembershipId,
    timestamp_start: u64,
    timestamp_end: u64,
    modes: &[i32],
    pool: &MySqlPool,
) -> Vec<ActivityHistoryRecord> {
    let mode_string = if modes.is_empty() {
        String::new()
    } else {
        let prepared_statement_pos = vec!["?"; modes.len()].join(",");
        format!("AND member_activities.mode IN ({})", prepared_statement_pos)
    };

    let statement = format!(
        r"
        SELECT
            *
        FROM member_activities
        INNER JOIN members ON member_activities.membership_id = members.membership_id 
        WHERE members.membership_id = ?
        AND (member_activities.occurred_at >= ? AND member_activities.occurred_at <= ?)
        {}
        ORDER BY member_activities.occurred_at DESC
    ",
        mode_string
    );

    let mut query_builder = sqlx::query_as::<_, ActivityHistoryRecord>(&statement)
        .bind(membership_id)
        .bind(timestamp_start)
        .bind(timestamp_end);

    if !modes.is_empty() {
        for mode in modes.iter() {
            query_builder = query_builder.bind(*mode);
        }
    }

    let query = query_builder.fetch_all(pool).await;
    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn missing_instance_data(
    start_timestamp: u64,
    end_timestamp: u64,
    amount: u64,
    pool: &MySqlPool,
) -> Vec<InstanceId> {
    let query = sqlx::query_as!(
        ActivityInstanceResult,
        r"
        WITH
        target_activities  AS
        (
            SELECT DISTINCT member_activities.instance_id FROM member_activities
            WHERE (member_activities.occurred_at > ? AND member_activities.occurred_at < ?)
        ),
        instance_member_count AS (
            SELECT
                target_activities.instance_id,
                COUNT(instance_members.id) AS instance_members
            FROM target_activities
            LEFT JOIN instance_members ON target_activities.instance_id = instance_members.instance_id
            GROUP BY target_activities.instance_id
        )
        SELECT
            target_activities.instance_id
        FROM target_activities
        INNER JOIN instance_member_count ON target_activities.instance_id = instance_member_count.instance_id
        WHERE instance_member_count.instance_members = 0
        LIMIT ?
        ",
        start_timestamp,
        end_timestamp,
        amount
    )
    .fetch_all(pool)
    .await;

    if let Ok(results) = query {
        results
            .iter()
            .map(|record| record.instance_id)
            .collect::<Vec<InstanceId>>()
    } else {
        database::log_error(query);
        Vec::new()
    }
}
