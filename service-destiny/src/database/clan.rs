use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::types::{destiny::GroupId, destiny::MembershipId, RecordId};
use levelcrush::util::unix_timestamp;
use levelcrush::{database, project_str};
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::database::member::MemberResult;

#[DatabaseRecord]
pub struct ClanRecord {
    pub group_id: i64,
    pub name: String,
    pub slug: String,
    pub motto: String,
    pub about: String,
    pub call_sign: String,
    pub is_network: i64,
}

#[DatabaseRecord]
pub struct ClanMemberRecord {
    pub group_id: i64,
    pub group_role: i64,
    pub membership_id: i64,
    pub platform: i64,
    pub joined_at: i64,
}

#[DatabaseResult]
pub struct ClanMemberSearchResult {
    pub id: RecordId,
    pub membership_id: i64,
}

#[derive(sqlx::FromRow, Clone, Default, Debug, serde::Serialize)]
pub struct ClanInfoResult {
    pub group_id: i64,
    pub name: String,
    pub slug: String,
    pub motto: String,
    pub about: String,
    pub call_sign: String,
    pub is_network: i64,
    pub member_count: i64,
    pub updated_at: i64,
}

/// gets a HashMap of existing member ids for the  target clan
///
/// key of membership_id , value of record id
pub async fn existing_members(group_id: GroupId, database: &SqlitePool) -> HashMap<MembershipId, RecordId> {
    let mut results = HashMap::new();

    let query = sqlx::query_file!("queries/clan_existing_members.sql", group_id)
        .fetch_all(database)
        .await;

    if let Ok(query) = query {
        for record in query.iter() {
            results.insert(record.membership_id, record.id);
        }
    } else {
        database::log_error(query);
    }

    results
}

/// searches the clan_members table for matching membership ids/record ids from the specified input
pub async fn find_by_membership(membership_ids: &[MembershipId], pool: &SqlitePool) -> HashMap<MembershipId, RecordId> {
    if membership_ids.is_empty() {
        return HashMap::new();
    }
    let mut results = HashMap::new();

    let in_prepared_pos = vec!["?"; membership_ids.len()].join(",");

    // for some reason sqlx or mysql (tbd) does not like a large amoount of membership ids being passed , so manually construct it
    // todo! find out what is going on here so we can use prepared statement
    // this is "ok" for now because we aren't passing manually strings and the data we are passing is strictly integers, but would be nice to have it as a full prepared statement
    let query_statement = project_str!("queries/clan_member_by_membership.sql", in_prepared_pos);

    // prepare statement
    let mut query = sqlx::query_as::<_, ClanMemberSearchResult>(query_statement.as_str());

    // now bind each membership id
    for membership_id in membership_ids.iter() {
        query = query.bind(*membership_id);
    }

    let query = query.fetch_all(pool).await;
    if let Ok(all_records) = query {
        for record in &all_records {
            results.insert(record.membership_id, record.id);
        }
    }

    results
}

/// insert a clan member into the table
pub async fn add_member(member: ClanMemberRecord, pool: &SqlitePool) -> RecordId {
    let timestamp = unix_timestamp();
    let query = sqlx::query_file!(
        "queries/clan_add_member.sql",
        member.group_id,
        member.group_role,
        member.membership_id,
        member.platform,
        member.joined_at,
        timestamp,
    )
    .execute(pool)
    .await;

    if let Ok(query) = query {
        query.last_insert_rowid() as RecordId
    } else {
        database::log_error(query);
        -1
    }
}

/// update member record in database
pub async fn update_member(member: &ClanMemberRecord, pool: &SqlitePool) -> bool {
    let query = sqlx::query_file!(
        "queries/clan_update_member.sql",
        member.group_id,
        member.group_role,
        member.membership_id,
        member.platform,
        member.joined_at,
        member.updated_at,
        member.deleted_at,
        member.id
    )
    .execute(pool)
    .await;

    if query.is_ok() {
        true
    } else {
        database::log_error(query);
        false
    }
}

/// remove clan members from the database by directly passing their record ids
pub async fn remove_members(records: &[RecordId], pool: &SqlitePool) {
    let in_prepared_pos = vec!["?"; records.len()].join(",");
    let statement = project_str!("queries/clan_remove_members.sql", in_prepared_pos);

    let mut query_builder = sqlx::query(statement.as_str());
    for record in records.iter() {
        query_builder = query_builder.bind(*record);
    }

    let query = query_builder.execute(pool).await;

    // if we have any errors report them
    // note: this function will only report if there is an error so safe to just use it like this
    database::log_error(query);
}

/// get a clan by directly querying by the group id
pub async fn get(group_id: i64, pool: &SqlitePool) -> Option<ClanRecord> {
    let query = sqlx::query_file_as!(ClanRecord, "queries/clan_get.sql", group_id)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

/// get the group ids of all network clans
pub async fn get_network(pool: &SqlitePool) -> Vec<GroupId> {
    let query = sqlx::query_file!("queries/network_get_clans.sql").fetch_all(pool).await;
    if let Ok(results) = query {
        results.iter().map(|record| record.group_id).collect::<Vec<GroupId>>()
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn from_membership(membership_id: MembershipId, pool: &SqlitePool) -> Option<ClanInfoResult> {
    let query = sqlx::query_file_as!(ClanInfoResult, "queries/clan_info_from_membership.sql", membership_id)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_network_roster(pool: &SqlitePool) -> Vec<MemberResult> {
    let query = sqlx::query_file_as!(MemberResult, "queries/network_roster.sql")
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn get_info(group_id: GroupId, pool: &SqlitePool) -> Option<ClanInfoResult> {
    let query = sqlx::query_file_as!(ClanInfoResult, "queries/clan_info_get.sql", group_id)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_roster(group_id: GroupId, pool: &SqlitePool) -> Vec<MemberResult> {
    let query = sqlx::query_file_as!(MemberResult, "queries/clan_roster_get.sql", group_id)
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn get_info_by_slug(slug: &str, pool: &SqlitePool) -> Option<ClanInfoResult> {
    let query = sqlx::query_file_as!(ClanInfoResult, "queries/clan_info_by_slug.sql", slug)
        .fetch_optional(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        None
    }
}

pub async fn get_network_info(pool: &SqlitePool) -> Vec<ClanInfoResult> {
    let query = sqlx::query_file_as!(ClanInfoResult, "queries/network_info_get.sql")
        .fetch_all(pool)
        .await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }
}

/// get group ids of all non network clans
pub async fn get_non_network(pool: &SqlitePool) -> Vec<GroupId> {
    let query = sqlx::query_file!("queries/clan_non_network.sql").fetch_all(pool).await;

    if let Ok(results) = query {
        results.iter().map(|record| record.group_id).collect::<Vec<GroupId>>()
    } else {
        database::log_error(query);
        Vec::new()
    }
}

/// insert a new clan into the database
pub async fn create(clan: ClanRecord, pool: &SqlitePool) -> RecordId {
    let query = sqlx::query_file!(
        "queries/clan_insert.sql",
        clan.group_id,
        clan.name,
        clan.slug,
        clan.motto,
        clan.about,
        clan.call_sign,
        clan.created_at,
    )
    .execute(pool)
    .await;

    if let Ok(query) = query {
        query.last_insert_rowid() as RecordId
    } else {
        database::log_error(query);
        -1
    }
}

/// update clan record
pub async fn update(clan: &ClanRecord, pool: &SqlitePool) -> bool {
    let query = sqlx::query_file!(
        "queries/clan_update.sql",
        clan.group_id,
        clan.name,
        clan.slug,
        clan.motto,
        clan.about,
        clan.call_sign,
        clan.updated_at,
        clan.deleted_at,
        clan.id
    )
    .execute(pool)
    .await;

    if query.is_ok() {
        true
    } else {
        database::log_error(query);
        false
    }
}
