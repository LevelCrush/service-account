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

    tracing::info!("Mapping triumphs to existing data if possible");
    let records = definitions
        .iter()
        .map(|(hash, definition)| {
            // extract title
            // bungie only seems to populate the male and female as the same (makes sense)
            let title = match definition.title_info.titles_by_gender.get("Male") {
                Some(data) => data.clone(),
                _ => String::new(),
            };

            TriumphRecord {
                id: 0,
                hash: definition.hash as i64,
                name: definition.display_properties.name.clone(),
                description: definition.display_properties.description.clone(),
                title,
                is_title: definition.title_info.has_title as i64,
                gilded: definition.for_title_gilding as i64,
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

    tracing::info!("Mapping seasons to existing data if possible");
    let records = definitions
        .iter()
        .map(|(hash, definition)| SeasonRecord {
            id: 0,
            hash: definition.hash as i64,
            name: definition.display_properties.name.clone(),
            pass_hash: definition.season_pass_hash as i64,
            number: definition.season_number as i64,
            starts_at: definition.start_date.timestamp(),
            ends_at: definition.end_date.timestamp(),
            created_at: unix_timestamp(),
            updated_at: 0,
            deleted_at: 0,
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

    let records = definitions
        .iter()
        .map(|(_, definition)| ActivityRecord {
            id: 0,
            activity_type: definition.activity_type_hash as i64,
            name: definition.display_properties.name.clone(),
            description: definition.display_properties.description.clone(),
            image_url: definition.pgcr_image.clone(),
            fireteam_min_size: definition.matchmaking.min_party as i64,
            fireteam_max_size: definition.matchmaking.max_party as i64,
            max_players: definition.matchmaking.max_players as i64,
            requires_guardian_oath: definition.matchmaking.requires_guardian_oath,
            is_pvp: definition.is_pvp,
            matchmaking_enabled: definition.matchmaking.is_matchmade,
            hash: definition.hash as i64,
            index: definition.index as i64,
            created_at: unix_timestamp(),
            updated_at: 0,
            deleted_at: 0,
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
    tracing::info!("Generating db chunks for activity type insertion");

    let records = definitions
        .iter()
        .map(|(_, definition)| ActivityTypeRecord {
            id: 0,
            hash: definition.hash as i64,
            index: definition.index as i64,
            name: definition.display_properties.name.clone(),
            description: definition.display_properties.description.clone(),
            icon_url: definition.display_properties.icon.clone(),
            created_at: unix_timestamp(),
            updated_at: 0,
            deleted_at: 0,
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

    tracing::info!("Generating db chunks for class data insertion");

    let mut chunk_index = 0;
    let mut records = Vec::new();

    for (_, definition) in definitions.iter() {
        if records.get(chunk_index).is_none() {
            records.push(Vec::new());
        }

        if let Some(chunk) = records.get_mut(chunk_index) {
            chunk.push(ClassRecord {
                id: 0,
                hash: definition.hash as i64,
                index: definition.index as i64,
                class_type: definition.class_type as i64,
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
