use crate::bungie::definitions::{
    DestinyActivityDefinitionMap, DestinyActivityTypeDefinitionMap, DestinyClassDefinitionMap,
    DestinyRecordDefinitionMap, DestinySeasonDefinitionMap,
};
use crate::database;
use crate::database::activity::ActivityRecord;
use crate::database::activity_types::ActivityTypeRecord;
use crate::database::class::ClassRecord;
use crate::database::seasons::SeasonRecord;
use crate::database::triumph::TriumphRecord;
use levelcrush::tracing;
use levelcrush::util::unix_timestamp;
use sqlx::SqlitePool;

const RECORDS_PER_CHUNK: usize = 100;

pub async fn records(definitions: &DestinyRecordDefinitionMap, pool: &SqlitePool) {
    tracing::info!("Working on {} total definitions for triumphs", definitions.len());

    let hashes = definitions
        .iter()
        .map(|(_, definition)| definition.hash)
        .collect::<Vec<u32>>();

    tracing::info!("Mapping triumphs to existing data if possible");
    let existing_definitions = database::triumph::read(&hashes, pool).await;
    let records = definitions
        .iter()
        .map(|(hash, definition)| {
            let record_id = match existing_definitions.get(&definition.hash) {
                Some(data) => data.id,
                _ => 0,
            };

            // if there is a title here extract it
            // at time of writing (and knowledge) there is no actual difference between male and female versions of titles
            let title = match definition.title_info.titles_by_gender.get("Male") {
                Some(data) => data.clone(),
                _ => String::new(),
            };

            TriumphRecord {
                id: record_id,
                hash: definition.hash,
                name: definition.display_properties.name.clone(),
                description: definition.display_properties.description.clone(),
                title,
                is_title: definition.title_info.has_title as i8,
                gilded: definition.for_title_gilding as i8,
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            }
        })
        .collect::<Vec<TriumphRecord>>();

    tracing::info!("Writing triumphs to database");
    for chunk in records.chunks(RECORDS_PER_CHUNK) {
        database::triumph::write(chunk, pool).await;
    }
}

pub async fn seasons(definitions: &DestinySeasonDefinitionMap, pool: &SqlitePool) {
    tracing::info!("Working on {} total definitions for seasons", definitions.len());

    let hashes = definitions
        .iter()
        .map(|(_, definition)| definition.hash)
        .collect::<Vec<u32>>();

    tracing::info!("Mapping seasons to existing data if possible");
    let existing_definitions = database::seasons::read(&hashes, pool).await;
    let records = definitions
        .iter()
        .map(|(hash, definition)| {
            let record_id = match existing_definitions.get(&definition.hash) {
                Some(data) => data.id,
                _ => 0,
            };

            SeasonRecord {
                id: record_id,
                hash: definition.hash,
                name: definition.display_properties.name.clone(),
                pass_hash: definition.season_pass_hash,
                number: definition.season_number,
                starts_at: definition.start_date.timestamp() as u64,
                ends_at: definition.end_date.timestamp() as u64,
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            }
        })
        .collect::<Vec<SeasonRecord>>();

    tracing::info!("Writing seasons to database");
    for chunk in records.chunks(RECORDS_PER_CHUNK) {
        database::seasons::write(chunk, pool).await;
    }
}

/// sync activity definitions to database
pub async fn activities(definitions: &DestinyActivityDefinitionMap, pool: &SqlitePool) {
    tracing::info!("Working on {} total definitions for activities", definitions.len());

    let hashes = definitions
        .iter()
        .map(|(_, definition)| definition.hash)
        .collect::<Vec<u32>>();

    let existing_definitions = database::activity::exists_bulk(&hashes, pool).await;
    let records = definitions
        .iter()
        .map(|(_, definition)| {
            // extract record id if possible
            let record_id = match existing_definitions.get(&definition.hash) {
                Some(id) => *id,
                _ => 0,
            };

            ActivityRecord {
                id: record_id,
                activity_type: definition.activity_type_hash,
                name: definition.display_properties.name.clone(),
                description: definition.display_properties.description.clone(),
                image_url: definition.pgcr_image.clone(),
                fireteam_min_size: definition.matchmaking.min_party,
                fireteam_max_size: definition.matchmaking.max_party,
                max_players: definition.matchmaking.max_players,
                requires_guardian_oath: definition.matchmaking.requires_guardian_oath,
                is_pvp: definition.is_pvp,
                matchmaking_enabled: definition.matchmaking.is_matchmade,
                hash: definition.hash,
                index: definition.index,
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            }
        })
        .collect::<Vec<ActivityRecord>>();

    tracing::info!(
        "Total Activites: {} | Total Chunks: {}",
        definitions.len(),
        records.len()
    );

    for chunk in records.chunks(RECORDS_PER_CHUNK) {
        // attempt to write out. If it fails, then that's ok. We don't care.
        database::activity::write(chunk, pool).await;
    }

    tracing::info!("Done writing activities to DB");
}

/// syncs activity type definitions the definitions to the database
pub async fn activity_types(definitions: &DestinyActivityTypeDefinitionMap, pool: &SqlitePool) {
    tracing::info!("Working on {} total activity type definitions", definitions.len());

    let hashes = definitions
        .iter()
        .map(|(_, definition)| definition.hash)
        .collect::<Vec<u32>>();

    tracing::info!("Generating db chunks for activity type insertion");

    let existing_definitions = database::activity_types::exists_bulk(&hashes, pool).await;
    let records = definitions
        .iter()
        .map(|(_, definition)| {
            // extract record id if possible
            let record_id = match existing_definitions.get(&definition.hash) {
                Some(id) => *id,
                _ => 0,
            };

            ActivityTypeRecord {
                id: record_id,
                hash: definition.hash,
                index: definition.index,
                name: definition.display_properties.name.clone(),
                description: definition.display_properties.description.clone(),
                icon_url: definition.display_properties.icon.clone(),
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            }
        })
        .collect::<Vec<ActivityTypeRecord>>();

    tracing::info!(
        "Total Activity Types: {} | Total Chunks: {}",
        definitions.len(),
        records.len()
    );

    for chunk in records.chunks(RECORDS_PER_CHUNK) {
        // attempt to write out. If it fails, then that's ok. We don't care.
        database::activity_types::write(chunk, pool).await;
    }

    tracing::info!("Done writing activity types to db");
}

/// syncs class definitions to the database
pub async fn classes(definitions: &DestinyClassDefinitionMap, pool: &SqlitePool) {
    tracing::info!("Working on {} total definitions", definitions.len());

    let mut hashes = Vec::new();
    for (_, definition) in definitions.iter() {
        hashes.push(definition.hash);
    }

    tracing::info!("Generating db chunks for class data insertion");

    let mut chunk_index = 0;
    let mut records = Vec::new();
    let existing_definitions = database::class::exists_bulk(&hashes, pool).await;
    for (_, definition) in definitions.iter() {
        // extract record id if possible
        let record_id = match existing_definitions.get(&definition.hash) {
            Some(id) => *id,
            _ => 0,
        };

        if records.get(chunk_index).is_none() {
            records.push(Vec::new());
        }

        if let Some(chunk) = records.get_mut(chunk_index) {
            chunk.push(ClassRecord {
                id: record_id,
                hash: definition.hash,
                index: definition.index,
                class_type: definition.class_type,
                name: definition.display_properties.name.clone(),
                created_at: unix_timestamp(),
                updated_at: 0,
                deleted_at: 0,
            });

            if chunk.len() >= RECORDS_PER_CHUNK {
                chunk_index += 1;
            }
        }
    }

    tracing::info!("Total Classes: {} | Total Chunks: {}", definitions.len(), records.len());

    for chunk in records.iter() {
        // attempt to write out. If it fails, then that's ok. We don't care.
        database::class::write(chunk, pool).await;
    }
    tracing::info!("Done writing classes to database");
}
