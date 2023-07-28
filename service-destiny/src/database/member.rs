use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::types::destiny::MembershipId;
use levelcrush::types::RecordId;
use levelcrush::util::unix_timestamp;
use levelcrush::{bigdecimal::ToPrimitive, BigDecimal};
use levelcrush::{database, project_str};
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

#[DatabaseRecord]
pub struct MemberRecord {
    pub membership_id: MembershipId,
    pub platform: i32,
    pub display_name: String,
    pub display_name_global: String,
    pub guardian_rank_current: u8,
    pub guardian_rank_lifetime: u8,
    pub last_played_at: u64,
}

#[DatabaseResult]
pub struct MembershipReadyCheckResult {
    pub membership_id: MembershipId,
    pub updated_at: u64,
}

#[DatabaseResult]
pub struct MemberResult {
    pub membership_id: MembershipId,
    pub platform: i32,
    pub last_played_at: u64,
    pub display_name: String,
    pub display_name_global: String,
    pub clan_group_id: i64,
    pub clan_name: String,
    pub clan_call_sign: String,
    pub clan_joined_at: u64,
    pub clan_group_role: i8,
    pub clan_is_network: i8,
    pub updated_at: u64,
}

#[DatabaseRecord]
pub struct MemberSnapshotRecord {
    pub membership_id: MembershipId,
    pub snapshot_name: String,
    pub version: i64,
    pub data: String,
}

pub async fn get_snapshot(
    membership_id: MembershipId,
    snapshot: &str,
    version: u8,
    pool: &SqlitePool,
) -> Option<MemberSnapshotRecord> {
    let query = sqlx::query_file_as!(
        MemberSnapshotRecord,
        "queries/member_snapshot_get.sql",
        membership_id,
        snapshot,
        version,
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

pub async fn write_snapshot(
    membership_id: MembershipId,
    snapshot: &str,
    version: i64,
    data: String,
    pool: &SqlitePool,
) {
    let timestamp = unix_timestamp();
    let query = sqlx::query_file!(
        "queries/member_snapshot_write.sql",
        membership_id,
        snapshot,
        version,
        data,
        timestamp,
        0,
        0,
    );

    let result = query.execute(pool).await;
    database::log_error(result);
}

/// similiar to the status function, except it searches by bungie name instead
/// NOTE: If in the instance that a user has an inactive linked account (not primary) and it finds it way into our system
/// we will only return the member record that has been most recently played
pub async fn get_by_bungie_name(bungie_name: &str, pool: &SqlitePool) -> Option<MemberResult> {
    let query = sqlx::query_file!("queries/member_get_by_bungie.sql", bungie_name)
        .fetch_optional(pool)
        .await;

    if let Ok(Some(record)) = query {
        // there has to be a better then forcing a map on sqlx like this when types mismatch
        // try_from is not **currently** working for me, maybe I'm doing something wrong
        // for now this works
        Some(MemberResult {
            membership_id: record.membership_id,
            platform: record.platform,
            last_played_at: record.last_played_at,
            display_name: record.display_name,
            updated_at: record.updated_at,
            display_name_global: record.display_name_global,
            clan_group_id: record.clan_group_id,
            clan_group_role: record.clan_group_role as i8,
            clan_call_sign: record.clan_call_sign,
            clan_joined_at: record.clan_joined_at.to_u64().unwrap_or_default(),
            clan_is_network: record.clan_is_network as i8,
            clan_name: record.clan_name,
        })
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get(membership_id: i64, pool: &SqlitePool) -> Option<MemberResult> {
    let query = sqlx::query_file!("queries/member_get.sql", membership_id)
        .fetch_optional(pool)
        .await;

    if let Ok(Some(record)) = query {
        // there has to be a better then forcing a map on sqlx like this when types mismatch
        // try_from is not **currently** working for me, maybe I'm doing something wrong
        // for now this works
        Some(MemberResult {
            membership_id: record.membership_id,
            platform: record.platform,
            last_played_at: record.last_played_at,
            display_name: record.display_name,
            updated_at: record.updated_at,
            display_name_global: record.display_name_global,
            clan_group_id: record.clan_group_id,
            clan_group_role: record.clan_group_role as i8,
            clan_call_sign: record.clan_call_sign,
            clan_joined_at: record.clan_joined_at.to_u64().unwrap_or_default(),
            clan_is_network: record.clan_is_network as i8,
            clan_name: record.clan_name,
        })
    } else {
        database::log_error(query);
        None
    }
}

pub async fn multi_get(membership_ids: &[i64], pool: &SqlitePool) -> Vec<MemberResult> {
    if membership_ids.is_empty() {
        return Vec::new();
    }

    let prepared_pos = vec!["?"; membership_ids.len()].join(",");

    let statement = project_str!("queries/member_multi_get.sql", prepared_pos);
    let mut query_builder = sqlx::query(&statement);
    for membership_id in membership_ids.iter() {
        query_builder = query_builder.bind(membership_id);
    }

    let query = query_builder
        .map(|row: MySqlRow| MemberResult {
            membership_id: row.get("membership_id"),
            platform: row.get("platform"),
            last_played_at: row.get("last_played_at"),
            display_name: row.get("display_name"),
            display_name_global: row.get("display_name_global"),
            updated_at: row.get("updated_at"),
            clan_group_id: row.get("clan_group_id"),
            clan_call_sign: row.get("clan_call_sign"),
            clan_joined_at: row.get::<BigDecimal, _>("clan_joined_at").to_u64().unwrap_or_default(),
            clan_group_role: row.get("clan_group_role"),
            clan_is_network: row.get("clan_is_network"),
            clan_name: row.get("clan_name"),
        })
        .fetch_all(pool)
        .await;

    if let Ok(records) = query {
        // there has to be a better then forcing a map on sqlx like this when types mismatch
        // try_from is not **currently** working for me, maybe I'm doing something wrong
        // for now this works
        records
    } else {
        database::log_error(query);
        Vec::new()
    }
}

/// fetches a member record from the database with only the membership_id
pub async fn get_record(membership_id: i64, pool: &SqlitePool) -> Option<MemberRecord> {
    let query = sqlx::query_file_as!(MemberRecord, "queries/member_record_get.sql", membership_id)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

/// update membership record
pub async fn update(member: &MemberRecord, database: &SqlitePool) -> bool {
    // record found, update it!
    let query = sqlx::query_file!(
        "queries/member_record_update.sql",
        member.platform,
        member.display_name,
        member.display_name_global,
        member.guardian_rank_current,
        member.guardian_rank_lifetime,
        member.last_played_at,
        member.updated_at,
        member.id
    )
    .execute(database)
    .await;

    if query.is_ok() {
        true
    } else {
        database::log_error(query);
        false
    }
}

pub async fn create(member: MemberRecord, database: &SqlitePool) -> RecordId {
    let query = sqlx::query_file!(
        "queries/member_record_insert.sql",
        member.platform,
        member.membership_id,
        member.display_name,
        member.display_name_global,
        member.guardian_rank_current,
        member.guardian_rank_lifetime,
        member.last_played_at,
        member.created_at
    )
    .execute(database)
    .await;

    if let Ok(query) = query {
        query.last_insert_id() as RecordId
    } else {
        database::log_error(query);
        -1
    }
}

pub async fn search_count<T: Into<String>>(display_name: T, pool: &SqlitePool) -> u32 {
    let normal_name = display_name.into();
    let escaped = normal_name.replace('%', "\\%").replace('_', "\\_");
    let wildcard_search = format!("%{}%", escaped);

    let query = sqlx::query_file!("queries/member_search_count.sql", wildcard_search, wildcard_search)
        .fetch_optional(pool)
        .await;

    if let Ok(Some(query)) = query {
        query.total as u32
    } else {
        database::log_error(query);
        0
    }
}

pub async fn search<T: Into<String>>(display_name: T, offset: u32, limit: u32, pool: &SqlitePool) -> Vec<MemberResult> {
    let normal_name = display_name.into();
    let escaped = normal_name.replace('%', "\\%").replace('_', "\\_");
    let wildcard_search = format!("%{}%", escaped);
    let query = sqlx::query_file!(
        "queries/member_search.sql",
        wildcard_search,
        wildcard_search,
        offset,
        limit
    )
    .fetch_all(pool)
    .await;

    if let Ok(query) = query {
        // there has to be a better then forcing a map on sqlx like this when types mismatch
        // try_from is not **currently** working for me, maybe I'm doing something wrong
        // for now this works
        query
            .into_iter()
            .map(|record| MemberResult {
                membership_id: record.membership_id,
                platform: record.platform,
                last_played_at: record.last_played_at,
                updated_at: record.updated_at,
                display_name: record.display_name,
                display_name_global: record.display_name_global,
                clan_group_id: record.clan_group_id,
                clan_group_role: record.clan_group_role as i8,
                clan_call_sign: record.clan_call_sign,
                clan_joined_at: record.clan_joined_at.to_u64().unwrap_or_default(),
                clan_is_network: record.clan_is_network as i8,
                clan_name: record.clan_name,
            })
            .collect::<Vec<MemberResult>>()
    } else {
        database::log_error(query);
        Vec::new()
    }
}
