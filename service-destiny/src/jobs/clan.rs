use std::collections::{HashMap, HashSet};

use crate::app::state::AppState;
use crate::database;
use crate::jobs::task;
use levelcrush::tracing;
use levelcrush::types::destiny::InstanceId;

pub async fn info(args: &[String]) {
    tracing::info!("Running job: Clan Info");
    tracing::info!("Setting up app state");
    let app_state = AppState::new().await;

    // parse arguments as group ids
    let group_ids: Vec<i64> = args
        .iter()
        .map(|group_id| group_id.parse::<i64>().unwrap_or_default())
        .collect();

    for group_id in group_ids.iter() {
        task::clan_info(*group_id, &app_state).await;
    }

    tracing::info!("Done syncing");
}

pub async fn roster(args: &[String]) {
    tracing::info!("Running job: Clan Roster");
    tracing::info!("Setting up app state");
    let app_state = AppState::new().await;

    // parse arguments as group ids
    let group_ids: Vec<i64> = args
        .iter()
        .map(|group_id| group_id.parse::<i64>().unwrap_or_default())
        .collect();

    for group_id in group_ids.iter() {
        task::clan_roster(*group_id, &app_state).await;
    }
}

async fn crawl(group_ids: &[i64], app_state: &AppState) {
    for group_id in group_ids.iter() {
        task::clan_info(*group_id, app_state).await;
        let members = task::clan_roster(*group_id, app_state).await;
        for (membership_id, membership_type) in members.iter() {
            task::profile(*membership_id, *membership_type, app_state).await;
        }
    }
}
/// crawl any clan in the system (or get any clan)
/// since we don't know if these are network clans or not, just crawl info, roster and profile data to sync profile + character informations
pub async fn crawl_direct(args: &[String]) {
    tracing::info!("Syncing direct clan data");
    tracing::info!("Setting up app state");

    let app_state = AppState::new().await;
    // parse arguments as group ids
    let group_ids: Vec<i64> = args
        .iter()
        .map(|group_id| group_id.parse::<i64>().unwrap_or_default())
        .collect();

    crawl(&group_ids, &app_state).await;
}

pub async fn crawl_non_network() {
    tracing::info!("Syncing all non network clan data");
    tracing::info!("Setting up app state");

    let app_state = AppState::new().await;
    let groups = database::clan::get_non_network(&app_state.database).await;
    crawl(&groups, &app_state).await;
}

/// crawl clans marked with is_network = 1
/// since we care about network more, we will crawl activities/stats/etc automatically
pub async fn crawl_network() {
    tracing::info!("crawling clan network");
    tracing::info!("Setting up app state");

    let app_state = AppState::new().await;
    let groups = database::clan::get_network(&app_state.database).await;

    let mut roster_members = HashMap::new();
    for group in groups.iter() {
        let group_id = *group;

        // crawl clan info
        task::clan_info(group_id, &app_state).await;

        // merger rosters
        let group_roster = task::clan_roster(group_id, &app_state).await;
        roster_members.extend(group_roster.iter());
    }

    // start crawling and then store all unique instance ids
    let instance_ids = {
        let mut tmp_set = HashSet::new();
        for (membership_id, membership_platform) in roster_members.iter() {
            let characters = task::profile(*membership_id, *membership_platform, &app_state).await;
            for character in characters.iter() {
                let character_instances =
                    task::activities(*membership_id, *membership_platform, *character, &app_state).await;

                tmp_set.extend(character_instances.iter());
            }
        }
        tmp_set.into_iter().collect::<Vec<InstanceId>>()
    };

    tracing::info!("Now crawling {} total instances", instance_ids.len());
    if !instance_ids.is_empty() {
        task::instance_data(&instance_ids, &app_state).await;
    }
}
