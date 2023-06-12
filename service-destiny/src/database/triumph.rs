use std::collections::HashMap;
use levelcrush::{
    database,
    types::{destiny::MembershipId},
};
use levelcrush_macros::{DatabaseRecord, DatabaseResult};
use sqlx::MySqlPool;

#[DatabaseRecord]
pub struct TriumphRecord {
    pub hash: u32,
    pub name: String,
    pub description: String,
    pub title: String,
    pub is_title: i8,
    pub gilded: i8,
}

#[DatabaseRecord]
pub struct MemberTriumphRecord {
    pub membership_id: MembershipId,
    pub hash: u32,
    pub state: i32,
    pub times_completed: i32,
}

#[DatabaseResult]
pub struct TriumphTitleResult {
    pub membership_id: MembershipId,
    pub title: String,
    pub has_gilded: i64,
    pub total_gilds: i64,
    pub can_equip: i64,
    pub can_equip_gilded: i64
}

pub async fn member_read(
    membership_id: MembershipId,
    hashes: &[u32],
    pool: &MySqlPool,
) -> HashMap<u32, MemberTriumphRecord> {
    if hashes.is_empty() {
        return HashMap::new();
    }

    let prepared_pos = vec!["?"; hashes.len()].join(",");
    let statement = format!(
        r"
            SELECT
                *
            FROM member_triumphs
            WHERE member_triumphs.membership_id = ?
            AND member_triumphs.hash IN ({})
        ",
        prepared_pos
    );

    let mut query_builder = sqlx::query_as::<_, MemberTriumphRecord>(&statement).bind(membership_id);

    for hash in hashes.iter() {
        query_builder = query_builder.bind(hash);
    }

    let query = query_builder.fetch_all(pool).await;
    if let Ok(query) = query {
        HashMap::from_iter(query.into_iter().map(|record| (record.hash, record)))
    } else {
        database::log_error(query);
        HashMap::new()
    }
}

pub async fn read(hashes: &[u32], pool: &MySqlPool) -> HashMap<u32, TriumphRecord> {
    if hashes.is_empty() {
        return HashMap::new();
    }

    let prepared_pos = vec!["?"; hashes.len()].join(",");
    let statement = format!(
        r"
            SELECT
                *
            FROM triumphs
            WHERE triumphs.hash IN ({})
        ",
        prepared_pos
    );

    let mut query_builder = sqlx::query_as::<_, TriumphRecord>(&statement);
    for hash in hashes.iter() {
        query_builder = query_builder.bind(hash);
    }

    let query = query_builder.fetch_all(pool).await;
    if let Ok(query) = query {
        HashMap::from_iter(query.into_iter().map(|record| (record.hash, record)))
    } else {
        database::log_error(query);
        HashMap::new()
    }
}

pub async fn write(records: &[TriumphRecord], pool: &MySqlPool) {
    let prepared_pos = vec!["(?,?,?,?,?,?,?,?,?,?)"; records.len()].join(",");

    let statement = format!(
        r"
            INSERT INTO triumphs (
                `id`,
                `hash`,
                `name`,
                `description`,
                `title`,
                `is_title`,
                `gilded`,
                `created_at`,
                `updated_at`,
                `deleted_at`
            )
            VALUES {}
            ON DUPLICATE KEY UPDATE
                `name` = VALUES(`name`),
                `description` = VALUES(`description`),
                `title` = VALUES(`title`),
                `is_title` = VALUES(`is_title`),
                `gilded` = VALUES(`gilded`),
                `updated_at` = VALUES(`created_at`)
        ",
        prepared_pos
    );

    let mut query_builder = sqlx::query(&statement);
    for record in records.iter() {
        query_builder = query_builder
            .bind(record.id)
            .bind(record.hash)
            .bind(record.name.clone())
            .bind(record.description.clone())
            .bind(record.title.clone())
            .bind(record.is_title)
            .bind(record.gilded)
            .bind(record.created_at)
            .bind(record.updated_at)
            .bind(record.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}

pub async fn member_write(records: &[MemberTriumphRecord], pool: &MySqlPool) {
    let prepared_pos = vec!["(?,?,?,?,?,?,?,?)"; records.len()].join(",");

    let statement = format!(
        r"
            INSERT INTO member_triumphs (
                `id`,
                `hash`,
                `membership_id`,
                `state`,
                `times_completed`,
                `created_at`,
                `updated_at`,
                `deleted_at`
            )
            VALUES {}
            ON DUPLICATE KEY UPDATE
                `state` = VALUES(`state`),
                `times_completed` = VALUES(`times_completed`),
                `updated_at` = VALUES(`created_at`)
        ",
        prepared_pos
    );

    let mut query_builder = sqlx::query(&statement);
    for record in records.iter() {
        query_builder = query_builder
            .bind(record.id)
            .bind(record.hash)
            .bind(record.membership_id)
            .bind(record.state)
            .bind(record.times_completed)
            .bind(record.created_at)
            .bind(record.updated_at)
            .bind(record.deleted_at);
    }

    let query = query_builder.execute(pool).await;
    database::log_error(query);
}

pub async fn member_titles(membership_id: MembershipId, pool: &MySqlPool) -> Vec<TriumphTitleResult> {
    let query = sqlx::query_as!(TriumphTitleResult, 
    r"
    WITH
    target_member AS (
        SELECT members.*
        FROM members
        WHERE members.membership_id  = ?
    ),
    seals AS (
        SELECT
            triumphs.hash,
            triumphs.title,
            triumphs.gilded
        FROM triumphs
        WHERE triumphs.is_title = 1
    ), # grab all seals (titles)

    member_seals_gilded AS (
        SELECT
            target_member.membership_id,
            seals.title,
            member_triumphs.state,
            member_triumphs.times_completed,
            member_triumphs.state & 64 = 64 AS can_equip
        FROM member_triumphs
        INNER JOIN seals ON member_triumphs.hash = seals.hash
        INNER JOIN target_member ON member_triumphs.membership_id = target_member.membership_id
        AND seals.gilded = 1 # looking for seals that have a gilded version
        AND member_triumphs.times_completed > 0 # and our member has completed it > 0 which means they have gilded at some point
    ),
    member_seals AS (
        SELECT
            target_member.membership_id,
            seals.title,
            member_triumphs.state,
            member_triumphs.times_completed,
            member_triumphs.state & 64 = 64 AS can_equip
        FROM member_triumphs
        INNER JOIN seals ON member_triumphs.hash = seals.hash
        INNER JOIN target_member ON member_triumphs.membership_id = target_member.membership_id
        AND seals.gilded = 0 # looking for seals that dont have a gilded version
    )
    # The below will merge all the gilded versions with the non gilded versions by Title.
    # There can be multiple versions of the same title, they just have slightly different objectives but they are still the same Title regardless
    SELECT
        member_seals.membership_id,
        member_seals.title,
        COALESCE(member_seals_gilded.times_completed > 0, 0) AS has_gilded,
        COALESCE(member_seals_gilded.times_completed, 0) AS total_gilds,
        member_seals.can_equip,
        COALESCE(member_seals_gilded.can_equip, 0) AS can_equip_gilded
    FROM member_seals
    LEFT JOIN member_seals_gilded ON member_seals.title = member_seals_gilded.title
    WHERE member_seals.can_equip = 1
    GROUP BY member_seals.membership_id, member_seals.title, member_seals_gilded.times_completed, member_seals_gilded.can_equip
    ORDER BY member_seals.title ASC
    ", membership_id
    ).fetch_all(pool).await;

    if let Ok(query) = query {
        query
    } else {
        database::log_error(query);
        Vec::new()
    }

}
