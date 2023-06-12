use levelcrush::database;
use levelcrush::types::{destiny::CharacterId, RecordId};
use levelcrush_macros::{DatabaseRecord, DatabaseResult};
use sqlx::MySqlPool;

#[DatabaseRecord]
pub struct CharacterRecord {
    pub membership_id: i64,
    pub platform: i32,
    pub character_id: i64,
    pub class_hash: u32,
    pub light: i32,
    pub last_played_at: u64,
    pub emblem_hash: u32,
    pub emblem_url: String,
    pub emblem_background_url: String,
    pub minutes_played_session: u32,
    pub minutes_played_lifetime: u32,
}

#[DatabaseResult]
pub struct CharacterStatusRecord {
    pub id: RecordId,
    pub character_id: i64,
    pub membership_id: i64,
    pub platform: i32,
    pub seconds_since_update: i64,
}

/// get a full character record from the database
pub async fn get(character_id: CharacterId, pool: &MySqlPool) -> Option<CharacterRecord> {
    let query = sqlx::query_as!(
        CharacterRecord,
        r"
            SELECT
                member_characters.*
            FROM member_characters
            WHERE member_characters.character_id = ?
            LIMIT 1
         ",
        character_id
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

/// inserts a new character record into the database
pub async fn create(character: CharacterRecord, database: &MySqlPool) -> RecordId {
    let query = sqlx::query!(
        r"
            INSERT INTO member_characters
            SET
                member_characters.membership_id = ?,
                member_characters.platform = ?,
                member_characters.character_id = ?,
                member_characters.class_hash = ?,
                member_characters.light = ?,
                member_characters.last_played_at = ?,
                member_characters.minutes_played_session = ?,
                member_characters.minutes_played_lifetime = ?,
                member_characters.emblem_hash = ?,
                member_characters.emblem_url = ?,
                member_characters.emblem_background_url = ?,
                member_characters.created_at = ?,
                member_characters.updated_at = 0,
                member_characters.deleted_at = 0,
                member_characters.id = 0
        ",
        character.membership_id,
        character.platform,
        character.character_id,
        character.class_hash,
        character.light,
        character.last_played_at,
        character.minutes_played_session,
        character.minutes_played_lifetime,
        character.emblem_hash,
        character.emblem_url,
        character.emblem_background_url,
        character.created_at
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

/// updates a record in the database
pub async fn update(character: &CharacterRecord, database: &MySqlPool) -> bool {
    let query = sqlx::query!(
        r"
            UPDATE member_characters
            SET
                member_characters.membership_id = ?,
                member_characters.platform = ?,
                member_characters.character_id = ?,
                member_characters.class_hash = ?,
                member_characters.light = ?,
                member_characters.last_played_at = ?,
                member_characters.minutes_played_session = ?,
                member_characters.minutes_played_lifetime = ?,
                member_characters.emblem_hash = ?,
                member_characters.emblem_url = ?,
                member_characters.emblem_background_url = ?,
                member_characters.updated_at = ?,
                member_characters.deleted_at = ?
            WHERE member_characters.id = ?
        ",
        character.membership_id,
        character.platform,
        character.character_id,
        character.class_hash,
        character.light,
        character.last_played_at,
        character.minutes_played_session,
        character.minutes_played_lifetime,
        character.emblem_hash,
        character.emblem_url,
        character.emblem_background_url,
        character.updated_at,
        character.deleted_at,
        character.id
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
