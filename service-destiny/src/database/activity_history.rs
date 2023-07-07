use levelcrush::bigdecimal::{BigDecimal, ToPrimitive};
use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::types::destiny::MembershipId;
use levelcrush::types::{destiny::CharacterId, destiny::InstanceId, RecordId, UnixTimestamp};
use levelcrush::{database, tracing};
use levelcrush::{project_str, MySqlPool};
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

#[DatabaseResult]
pub struct NetworkBreakdownResult {
    pub group_id: i64,
    pub name: String,
    pub total_members: i64,
    pub activity_attempts: i64,
    pub activities_completed_with_clan: i64,
    pub activities_completed: i64,
    pub percent_with_clan: BigDecimal,
    pub avg_clan_member_amount: BigDecimal,
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
    let statement = project_str!("queries/activity_history_existing.sql", prepared_statement_ins);

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
    let query = sqlx::query_file_as!(
        ActivityHistoryLastEntryResult,
        "queries/character_activity_last_timestamp.sql",
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
    let statement = project_str!("queries/activity_history_write.sql", prepared_statement_pos);

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
    let query = sqlx::query_file_as!(ActivityHistoryRecord, "queries/activity_history_oldest.sql")
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
    let query = sqlx::query_file_as!(ActivityHistoryRecord, "queries/activity_history_recent.sql")
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

    let statement = project_str!("queries/member_activity_history_range.sql", mode_string);
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
    let query = sqlx::query_file_as!(
        ActivityInstanceResult,
        "queries/activity_history_missing_instance_data.sql",
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

pub async fn network_breakdown(
    modes: &[i32],
    timestamp_start: u64,
    timestamp_end: u64,
    pool: &MySqlPool,
) -> HashMap<i64, NetworkBreakdownResult> {
    let modes_pos = vec!["?"; modes.len()].join(",");
    let statement = project_str!("queries/activity_network_breakdown.sql", modes_pos);
    let mut query_builder = sqlx::query_as::<_, NetworkBreakdownResult>(&statement);
    for mode in modes.iter() {
        query_builder = query_builder.bind(mode);
    }
    query_builder = query_builder.bind(timestamp_start).bind(timestamp_end);

    let query = query_builder.fetch_all(pool).await;

    if let Ok(results) = query {
        HashMap::from_iter(results.into_iter().map(|r| (r.group_id, r)))
    } else {
        database::log_error(query);
        HashMap::new()
    }
}
