use levelcrush::database;
use levelcrush::macros::{DatabaseRecord, DatabaseResult};
use levelcrush::types::{destiny::CharacterId, RecordId};
use sqlx::SqlitePool;

#[DatabaseRecord]
pub struct CharacterRecord {
    pub membership_id: i64,
    pub platform: i64,
    pub character_id: i64,
    pub class_hash: i64,
    pub light: i64,
    pub last_played_at: i64,
    pub emblem_hash: i64,
    pub emblem_url: String,
    pub emblem_background_url: String,
    pub minutes_played_session: i64,
    pub minutes_played_lifetime: i64,
}

#[DatabaseResult]
pub struct CharacterStatusRecord {
    pub id: RecordId,
    pub character_id: i64,
    pub membership_id: i64,
    pub platform: i64,
    pub seconds_since_update: i64,
}

/// get a full character record from the database
pub async fn get(character_id: CharacterId, pool: &SqlitePool) -> Option<CharacterRecord> {
    let query = sqlx::query_file_as!(CharacterRecord, "queries/character_get.sql", character_id)
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
pub async fn create(character: CharacterRecord, database: &SqlitePool) -> RecordId {
    let query = sqlx::query_file!(
        "queries/character_insert.sql",
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
        query.last_insert_rowid() as RecordId
    } else {
        database::log_error(query);
        -1
    }
}

/// updates a record in the database
pub async fn update(character: &CharacterRecord, database: &SqlitePool) -> bool {
    let query = sqlx::query_file!(
        "queries/character_update.sql",
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
