use crate::app::state::AppState;
use crate::env::AppVariable;
use crate::jobs::task;
use crate::persistant::PersistantCache;
use crate::{database, env};
use levelcrush::alias::destiny::InstanceId;
use levelcrush::anyhow;
use levelcrush::task_pool::TaskPool;
use levelcrush::tokio::sync::RwLock;
use levelcrush::tracing;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

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

    tracing::warn!("Loading from persistant cache");
    let instance_cache = PersistantCache::<Vec<i64>>::load("network_crawl_instance_ids.cache.json").await?;
    if !instance_cache.data.is_empty() {
        tracing::info!(
            "Did not finish last instance crawl. Will resume crawling {} total instances",
            instance_cache.data.len()
        );
    }

    let workers_allowed = env::get(AppVariable::CrawlWorkers).parse::<usize>().unwrap_or(1);
    tracing::warn!("Max Workers Per Pool: {workers_allowed}");

    let task_pool = TaskPool::new(workers_allowed);
    let instance_task_pool = TaskPool::new(workers_allowed);

    let instance_ids = Arc::new(RwLock::new(instance_cache.data.clone()));

    // shadow the old instance cache and move it into a ARC
    let instance_cache = Arc::new(RwLock::new(instance_cache));

    for (membership_id, membership_platform) in roster_members.into_iter() {
        let state_clone = app_state.clone();
        let instance_id_holder = instance_ids.clone();
        task_pool.queue(Box::new(move || {
            Box::pin(async move {
                let characters = task::profile(membership_id, membership_platform, &state_clone).await;
                if let Ok(characters) = characters {
                    for character in characters.into_iter() {
                        tracing::info!("Scanning activities for {membership_id} and {character}");
                        let instances = task::activities(membership_id, membership_platform,character,&state_clone).await;
                        if let Ok(instances) = instances {
                            let mut instance_id_writer = instance_id_holder.write().await;
                            instance_id_writer.extend(instances);
                            drop(instance_id_writer);
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
        levelcrush::tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // read from our instance id vector and clone the values into a hash set
    let instance_id_reader = instance_ids.read().await;
    let unique_instance_ids = instance_id_reader
        .clone()
        .into_iter()
        .collect::<HashSet<i64>>()
        .into_iter()
        .collect::<Vec<i64>>();

    drop(instance_id_reader); // drop reade

    // now we are going to save now that we only have unique instance ids
    tracing::info!("Saving instance ids to persistant cache");
    let instance_cache_handle = instance_cache.clone();
    let mut cache_writer = instance_cache_handle.write().await;
    cache_writer.data_mut().clear();
    cache_writer.data_mut().extend(unique_instance_ids.clone());
    let cache_save_result = cache_writer.save().await;
    if let Err(cache_save_err) = cache_save_result {
        tracing::error!("Error saving cache: {cache_save_err}");
    }
    drop(cache_writer);

    for chunk in unique_instance_ids.chunks(1000) {
        let instances = chunk.to_vec();
        let instance_cache_handle = instance_cache.clone();
        for instance_id in instances.into_iter() {
            let state_clone = app_state.clone();
            instance_task_pool
                .queue(Box::new(move || {
                    Box::pin(async move {
                        tracing::info!("Getting carnage report for: {}", instance_id);
                        let response = crate::api::instance::carnage_report(instance_id, &state_clone.bungie).await;
                        if let Ok(response) = response {
                            if let Some(response) = response {
                                crate::app::instance::carnage_report_sync(&response, &state_clone).await;
                            }
                        } else if let Err(response_err) = response {
                            tracing::error!("Error fetching carnage report for {instance_id}:\r\n{response_err}");
                        }
                    })
                }))
                .await;
        }

        while !instance_task_pool.is_empty().await {
            instance_task_pool.step().await;
            levelcrush::tokio::time::sleep(Duration::from_millis(100)).await;
        }

        tracing::info!("Updating persistant cache");
        let mut cache_writer = instance_cache_handle.write().await;
        for instance_id in chunk {
            let index_result = cache_writer.data_mut().iter().position(|&r| r == *instance_id);
            if let Some(index) = index_result {
                cache_writer.data_mut().remove(index);
            }
        }

        tracing::info!("Saving persistant cache");
        let cache_save_result = cache_writer.save().await;
        if let Err(cache_save_err) = cache_save_result {
            tracing::error!("Error saving cache: {cache_save_err}");
        }
        drop(cache_writer);
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
