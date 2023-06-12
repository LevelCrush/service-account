use levelcrush::database;
use levelcrush::types::{destiny::GroupId, destiny::MembershipId, RecordId};
use levelcrush::util::unix_timestamp;
use levelcrush_macros::{DatabaseRecord, DatabaseResult};
use sqlx::MySqlPool;
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
    pub is_network: i8,
}

#[DatabaseRecord]
pub struct ClanMemberRecord {
    pub group_id: i64,
    pub group_role: u8,
    pub membership_id: i64,
    pub platform: i32,
    pub joined_at: u64,
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
    pub is_network: i8,
    pub member_count: i64,
    pub updated_at: u64,
}

/// gets a HashMap of existing member ids for the  target clan
///
/// key of membership_id , value of record id
pub async fn existing_members(group_id: GroupId, database: &MySqlPool) -> HashMap<MembershipId, RecordId> {
    let mut results = HashMap::new();

    let query = sqlx::query!(
        r"
            SELECT
                clan_members.id,
                clan_members.membership_id
            FROM clan_members
            WHERE clan_members.group_id = ?
        ",
        group_id
    )
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
pub async fn find_by_membership(membership_ids: &[MembershipId], pool: &MySqlPool) -> HashMap<MembershipId, RecordId> {
    if membership_ids.is_empty() {
        return HashMap::new();
    }
    let mut results = HashMap::new();

    let in_prepared_pos = vec!["?"; membership_ids.len()].join(",");

    // for some reason sqlx or mysql (tbd) does not like a large amoount of membership ids being passed , so manually construct it
    // todo! find out what is going on here so we can use prepared statement
    // this is "ok" for now because we aren't passing manually strings and the data we are passing is strictly integers, but would be nice to have it as a full prepared statement
    let query_statement = format!(
        r"
            SELECT
                clan_members.id,
                clan_members.membership_id
            FROM clan_members
            WHERE clan_members.membership_id IN ({})
            ORDER BY clan_members.id ASC
        ",
        in_prepared_pos
    );

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
pub async fn add_member(member: ClanMemberRecord, pool: &MySqlPool) -> RecordId {
    let query = sqlx::query!(
        r"
        INSERT INTO clan_members
        SET
            id = 0,
            group_id = ?,
            group_role = ?,
            membership_id = ?,
            platform = ?,
            joined_at = ?,
            created_at = ?,
            updated_at = 0,
            deleted_at = 0

    ",
        member.group_id,
        member.group_role,
        member.membership_id,
        member.platform,
        member.joined_at,
        unix_timestamp()
    )
    .execute(pool)
    .await;

    if let Ok(query) = query {
        query.last_insert_id() as RecordId
    } else {
        database::log_error(query);
        -1
    }
}

/// update member record in database
pub async fn update_member(member: &ClanMemberRecord, pool: &MySqlPool) -> bool {
    let query = sqlx::query!(
        r"
        UPDATE clan_members
        SET
            group_id = ?,
            group_role = ?,
            membership_id = ?,
            platform = ?,
            joined_at = ?,
            updated_at = ?,
            deleted_at = ?
        WHERE id = ?
    ",
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
pub async fn remove_members(records: &[RecordId], pool: &MySqlPool) {
    let in_prepared_pos = vec!["?"; records.len()].join(",");
    let statement = format!(
        r"
        DELETE FROM clan_members
        WHERE clan_members.id IN ({})
    ",
        in_prepared_pos
    );

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
pub async fn get(group_id: i64, pool: &MySqlPool) -> Option<ClanRecord> {
    let query = sqlx::query_as!(
        ClanRecord,
        r"
        SELECT
            clans.*
        FROM clans
        WHERE clans.group_id = ?
        LIMIT 1
    ",
        group_id
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

/// get the group ids of all network clans
pub async fn get_network(pool: &MySqlPool) -> Vec<GroupId> {
    let query = sqlx::query!(
        r"
        SELECT 
            clans.group_id
        FROM clans
        WHERE clans.is_network = 1
    "
    )
    .fetch_all(pool)
    .await;

    if let Ok(results) = query {
        results.iter().map(|record| record.group_id).collect::<Vec<GroupId>>()
    } else {
        database::log_error(query);
        Vec::new()
    }
}

pub async fn from_membership(membership_id: MembershipId, pool: &MySqlPool) -> Option<ClanInfoResult> {
    let query = sqlx::query_as!(
        ClanInfoResult,
        r"
        SELECT

            clans.group_id,
            clans.name,
            clans.slug,
            clans.motto,
            clans.about,
            clans.call_sign,
            clans.is_network,
            clans.updated_at,
            COUNT(DISTINCT clan_members.membership_id) AS member_count
        FROM clans
        INNER JOIN clan_members AS target_member ON clans.group_id = target_member.group_id
        LEFT JOIN clan_members ON clans.group_id = clan_members.group_id
        WHERE target_member.membership_id = ?
        GROUP BY clans.group_id
        LIMIT 1
    ",
        membership_id
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

pub async fn get_network_roster(pool: &MySqlPool) -> Vec<MemberResult> {
    let query = sqlx::query_as!(
        MemberResult,
        r"
        SELECT
            members.membership_id,
            members.platform,
            members.last_played_at,
            members.display_name,
            members.display_name_global,
            members.updated_at,
            clans.group_id AS clan_group_id,
            clans.name AS clan_name,
            clans.call_sign AS clan_call_sign,
            clan_members.joined_at AS clan_joined_at,
            clan_members.group_role AS clan_group_role,
            clans.is_network AS clan_is_network
        FROM clan_members
        INNER JOIN clans ON clan_members.group_id = clans.group_id
        INNER JOIN members ON clan_members.membership_id = members.membership_id
        WHERE clans.is_network = 1
        ORDER BY  clan_members.group_role DESC, members.display_name_global ASC"
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

pub async fn get_info(group_id: GroupId, pool: &MySqlPool) -> Option<ClanInfoResult> {
    let query = sqlx::query_as!(
        ClanInfoResult,
        r"
        SELECT
            clans.group_id,
            clans.name,
            clans.slug,
            clans.motto,
            clans.about,
            clans.call_sign,
            clans.is_network,
            clans.updated_at,
            COUNT(DISTINCT clan_members.membership_id) AS member_count
        FROM clans
        LEFT JOIN clan_members ON clans.group_id = clan_members.group_id
        WHERE clans.group_id = ?
        GROUP BY clans.group_id
        LIMIT 1
    ",
        group_id
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

pub async fn get_roster(group_id: GroupId, pool: &MySqlPool) -> Vec<MemberResult> {
    let query = sqlx::query_as!(
        MemberResult,
        r" SELECT
            members.membership_id,
            members.platform,
            members.last_played_at,
            members.display_name,
            members.display_name_global,
            members.updated_at,
            clans.group_id AS clan_group_id,
            clans.name AS clan_name,
            clans.call_sign AS clan_call_sign,
            clan_members.joined_at AS clan_joined_at,
            clan_members.group_role AS clan_group_role,
            clans.is_network AS clan_is_network
        FROM clan_members
        INNER JOIN clans ON clan_members.group_id = clans.group_id
        INNER JOIN members ON clan_members.membership_id = members.membership_id
        WHERE clans.group_id = ? 
        ORDER BY  clan_members.group_role DESC, members.display_name_global ASC",
        group_id
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

pub async fn get_info_by_slug(slug: &str, pool: &MySqlPool) -> Option<ClanInfoResult> {
    let query = sqlx::query_as!(
        ClanInfoResult,
        r"
        SELECT
            clans.group_id,
            clans.name,
            clans.slug,
            clans.motto,
            clans.about,
            clans.call_sign,
            clans.is_network,
            clans.updated_at,
            COUNT(DISTINCT clan_members.membership_id) AS member_count
        FROM clans
        LEFT JOIN clan_members ON clans.group_id = clan_members.group_id
        WHERE clans.slug = ?
        GROUP BY clans.group_id, clans.is_network
        ORDER BY clans.is_network DESC, clans.group_id ASC
        LIMIT 1
    ",
        slug
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

pub async fn get_network_info(pool: &MySqlPool) -> Vec<ClanInfoResult> {
    let query = sqlx::query_as!(
        ClanInfoResult,
        r"
        SELECT
            clans.group_id,
            clans.name,
            clans.slug,
            clans.motto,
            clans.about,
            clans.call_sign,
            clans.is_network,
            clans.updated_at,
            COUNT(DISTINCT clan_members.membership_id) AS member_count
        FROM clans
        LEFT JOIN clan_members ON clans.group_id = clan_members.group_id
        WHERE clans.is_network = 1
        GROUP BY clans.group_id, clans.name
        ORDER BY clans.name ASC
    "
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

/// get group ids of all non network clans
pub async fn get_non_network(pool: &MySqlPool) -> Vec<GroupId> {
    let query = sqlx::query!(
        r"
        SELECT 
            clans.group_id
        FROM clans
        WHERE clans.is_network = 0
    "
    )
    .fetch_all(pool)
    .await;

    if let Ok(results) = query {
        results.iter().map(|record| record.group_id).collect::<Vec<GroupId>>()
    } else {
        database::log_error(query);
        Vec::new()
    }
}

/// insert a new clan into the database
pub async fn create(clan: ClanRecord, pool: &MySqlPool) -> RecordId {
    let query = sqlx::query!(
        r"
        INSERT INTO clans
        SET
            id = 0,
            group_id = ?,
            name = ?,
            slug = ?,
            motto = ?,
            about = ?,
            call_sign = ?,
            is_network = 0,
            created_at = ?,
            updated_at = 0,
            deleted_at = 0
    ",
        clan.group_id,
        clan.name,
        clan.slug,
        clan.motto,
        clan.about,
        clan.call_sign,
        clan.created_at
    )
    .execute(pool)
    .await;

    if let Ok(query) = query {
        query.last_insert_id() as RecordId
    } else {
        database::log_error(query);
        -1
    }
}

/// update clan record
pub async fn update(clan: &ClanRecord, pool: &MySqlPool) -> bool {
    let query = sqlx::query!(
        r"
        UPDATE clans
        SET
            group_id = ?,
            name = ?,
            slug = ?,
            motto = ?,
            about = ?,
            call_sign = ?,
            updated_at = ?,
            deleted_at = ?
        WHERE clans.id = ?
    ",
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
