use crate::env::Env;
use crate::{app::state::AppState, jobs::task};
use levelcrush::anyhow;
use levelcrush::tracing;
use std::collections::HashSet;

/// always fetch the bungie api and return fresh data when being called as a job
pub async fn profile(args: &[String], env: &Env) -> anyhow::Result<()> {
    tracing::warn!("Setting up application state");
    let state = AppState::new(env).await;

    for bungie_name in args.iter() {
        // search and sync if we can the profiles by searching for bungie name
        task::profile_search(bungie_name, &state).await?;
    }

    Ok(())
}

pub async fn crawl_profile(args: &[String], env: &Env) -> anyhow::Result<()> {
    let state = AppState::new(env).await;

    for bungie_name in args.iter() {
        // search and sync if we can the profiles by searching for bungie name
        let profile_results = task::profile_search(bungie_name, &state).await?;
        if let Some(profile_results) = profile_results {
            // querying clan information
            task::clan_info_by_membership(profile_results.membership_id, profile_results.membership_type, &state)
                .await?;
        }
    }

    Ok(())
}

pub async fn crawl_profile_deep(args: &[String], env: &Env) -> anyhow::Result<()> {
    let state = AppState::new(env).await;

    for bungie_name in args.iter() {
        // search and sync if we can the profiles by searching for bungie name
        let profile_results = task::profile_search(bungie_name, &state).await?;
        if let Some(profile_results) = profile_results {
            // querying clan information
            task::clan_info_by_membership(profile_results.membership_id, profile_results.membership_type, &state)
                .await?;
            let mut tmp_set = HashSet::new();
            // now go through and do any character actions
            for character in profile_results.characters.iter() {
                let character_instances = task::activities(
                    profile_results.membership_id,
                    profile_results.membership_type,
                    *character,
                    &state,
                )
                .await?;
                tmp_set.extend(character_instances.iter());
            }

            let instance_ids = tmp_set.into_iter().collect::<Vec<i64>>();

            tracing::info!("Now crawling {} total instances", instance_ids.len());
            if !instance_ids.is_empty() {
                task::instance_data(&instance_ids, &state).await?;
            }
        }
    }
    Ok(())
}
