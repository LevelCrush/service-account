use std::collections::{HashMap, HashSet};
use std::time::Duration;

use levelcrush::anyhow;
use levelcrush::task_manager::TaskPool;
use levelcrush::tracing;
use levelcrush::types::destiny::InstanceId;

use crate::app::state::AppState;
use crate::database;
use crate::jobs::task;

pub async fn info(args: &[String]) -> anyhow::Result<()> {
    tracing::info!("Running job: Clan Info");
    tracing::info!("Setting up app state");
    let app_state = AppState::new().await;

    // parse arguments as group ids
    let group_ids: Vec<i64> = args
        .iter()
        .map(|group_id| group_id.parse::<i64>().unwrap_or_default())
        .collect();

    for group_id in group_ids.iter() {
        task::clan_info(*group_id, &app_state).await?;
    }

    tracing::info!("Done syncing");
    Ok(())
}

pub async fn roster(args: &[String]) -> anyhow::Result<()> {
    tracing::info!("Running job: Clan Roster");
    tracing::info!("Setting up app state");
    let app_state = AppState::new().await;

    // parse arguments as group ids
    let group_ids: Vec<i64> = args
        .iter()
        .map(|group_id| group_id.parse::<i64>().unwrap_or_default())
        .collect();

    for group_id in group_ids.iter() {
        task::clan_roster(*group_id, &app_state).await?;
    }
    Ok(())
}

async fn crawl(group_ids: &[i64], app_state: &AppState) -> anyhow::Result<()> {
    for group_id in group_ids.iter() {
        task::clan_info(*group_id, app_state).await?;
        let members = task::clan_roster(*group_id, app_state).await?;
        for (membership_id, membership_type) in members.iter() {
            task::profile(*membership_id, *membership_type, app_state).await?;
        }
    }
    Ok(())
}

/// crawl any clan in the system (or get any clan)
/// since we don't know if these are network clans or not, just crawl info, roster and profile data to sync profile + character informations
pub async fn crawl_direct(args: &[String]) -> anyhow::Result<()> {
    tracing::info!("Syncing direct clan data");
    tracing::info!("Setting up app state");

    let app_state = AppState::new().await;
    // parse arguments as group ids
    let group_ids: Vec<i64> = args
        .iter()
        .map(|group_id| group_id.parse::<i64>().unwrap_or_default())
        .collect();

    crawl(&group_ids, &app_state).await?;
    Ok(())
}

pub async fn crawl_non_network() -> anyhow::Result<()> {
    tracing::info!("Syncing all non network clan data");
    tracing::info!("Setting up app state");

    let app_state = AppState::new().await;
    let groups = database::clan::get_non_network(&app_state.database).await;
    crawl(&groups, &app_state).await?;

    Ok(())
}

/// an improved version of the crawler that uses the task pool
pub async fn crawl_network2() -> anyhow::Result<()> {
    tracing::info!("crawling clan network");
    tracing::info!("Setting up app state");

    let app_state = AppState::new().await;

    tracing::info!("Getting in network clans");
    let groups = database::clan::get_network(&app_state.database).await;

    let mut roster_members = HashMap::new();
    for group in groups.iter() {
        let group_id = *group;

        // crawl clan info
        task::clan_info(group_id, &app_state).await?;

        // merger rosters
        let group_roster = task::clan_roster(group_id, &app_state).await?;
        roster_members.extend(group_roster.iter());
    }

    let task_pool = TaskPool::new(8);
    for (membership_id, membership_platform) in roster_members.into_iter() {
        let state_clone = app_state.clone();
        task_pool.queue(Box::new(move || {
            Box::pin(async move {
                let characters = task::profile(membership_id, membership_platform, &state_clone).await;
                if let Ok(characters) = characters {
                    for character in characters.into_iter() {
                        tracing::info!("Scanning activities for {membership_id} and {character}");
                        let instances = task::activities(membership_id, membership_platform,character,&state_clone).await;
                        if let Ok(instances) = instances {
                        tracing::info!("Now crawling {} total instances for {membership_id} and {character}", instances.len());
                            let instance_data_result = task::instance_data(&instances, &state_clone).await;
                            if let Err(instance_err) = instance_data_result {
                                tracing::error!("Error fetching instance data for {membership_id} and character {character}:\n{instance_err}");
                            }
                        } else if let Err(instances_err) = instances {
                            tracing::error!("Error fetching activities for Member {membership_id} and character {character}:\n{instances_err}");
                        }
                       
                    }
                } else if let Err(character_err) = characters {
                    tracing::error!("Error fetching characters from the api\nMembership:{membership_id}|{membership_platform}\nError:{character_err}");
                }
            })
        })).await;
    }

    while !task_pool.is_empty().await {
        task_pool.step().await;
        levelcrush::tokio::time::sleep(Duration::from_secs(1)).await;
    }   

    tracing::info!("Done network crawling");

    Ok(())
}

/// crawl clans marked with is_network = 1
/// since we care about network more, we will crawl activities/stats/etc automatically
pub async fn crawl_network() -> anyhow::Result<()> {
    tracing::info!("crawling clan network");
    tracing::info!("Setting up app state");

    let app_state = AppState::new().await;
    let groups = database::clan::get_network(&app_state.database).await;

    let mut roster_members = HashMap::new();
    for group in groups.iter() {
        let group_id = *group;

        // crawl clan info
        task::clan_info(group_id, &app_state).await?;

        // merger rosters
        let group_roster = task::clan_roster(group_id, &app_state).await?;
        roster_members.extend(group_roster.iter());
    }

    // start crawling and then store all unique instance ids
    let instance_ids = {
        let mut tmp_set = HashSet::new();
        for (membership_id, membership_platform) in roster_members.iter() {
            let characters = task::profile(*membership_id, *membership_platform, &app_state).await?;
            for character in characters.iter() {
                let character_instances =
                    task::activities(*membership_id, *membership_platform, *character, &app_state).await?;

                tmp_set.extend(character_instances.iter());
            }
        }
        tmp_set.into_iter().collect::<Vec<InstanceId>>()
    };

    tracing::info!("Now crawling {} total instances", instance_ids.len());
    if !instance_ids.is_empty() {
        task::instance_data(&instance_ids, &app_state).await?;
    }

    Ok(())
}
