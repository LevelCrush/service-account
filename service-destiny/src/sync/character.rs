use crate::bungie::schemas::DestinyCharacterComponent;
use crate::database;
use crate::database::character::CharacterRecord;
use levelcrush::futures;
use levelcrush::types::{
    destiny::CharacterId, destiny::MembershipId, destiny::MembershipType, RecordId, UnixTimestamp,
};
use levelcrush::util::unix_timestamp;
use sqlx::MySqlPool;

pub async fn single(character: &DestinyCharacterComponent, database: &MySqlPool) -> RecordId {
    let membership_id = character.membership_id.parse::<MembershipId>().unwrap_or_default();
    let membership_type = character.membership_type as MembershipType;

    let mut record = CharacterRecord {
        id: 0,
        membership_id,
        platform: membership_type,
        character_id: character.character_id.parse::<CharacterId>().unwrap_or_default(),
        class_hash: character.class_hash,
        light: character.light,
        last_played_at: character.last_played.timestamp() as UnixTimestamp,
        emblem_hash: character.emblem_hash,
        emblem_url: character.emblem_path.clone(),
        emblem_background_url: character.emblem_background_path.clone(),
        minutes_played_session: character.minutes_played_session.parse::<u32>().unwrap_or_default(),
        minutes_played_lifetime: character.minutes_played_total.parse::<u32>().unwrap_or_default(),
        created_at: 0,
        updated_at: 0,
        deleted_at: 0,
    };

    let existing_character = database::character::get(record.character_id, database).await;
    if let Some(existing_character) = existing_character {
        // update existing record
        record.id = existing_character.id;
        record.created_at = existing_character.created_at;
        record.updated_at = unix_timestamp();
        record.deleted_at = existing_character.deleted_at;

        database::character::update(&record, database).await;
        record.id
    } else {
        // insert new character
        record.id = 0;
        record.created_at = unix_timestamp();
        record.updated_at = 0;
        record.deleted_at = 0;

        database::character::create(record, database).await
    }
}

pub async fn multiple(characters: &[DestinyCharacterComponent], database: &MySqlPool) {
    let mut character_futures = Vec::with_capacity(characters.len());
    for character in characters.iter() {
        character_futures.push(single(character, database));
    }

    futures::future::join_all(character_futures).await;
}
