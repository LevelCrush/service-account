use crate::{
    app::{self, state::AppState},
    database, sync,
};
use levelcrush::tracing;
use levelcrush::types::destiny::{CharacterId, InstanceId, MembershipId, MembershipType};
use std::collections::HashMap;

pub struct ProfileSearchResults {
    pub membership_id: i64,
    pub membership_type: i32,
    pub bungie_name: String,
    pub characters: Vec<i64>,
}

/// handles fetching profile information and then syncing it
pub async fn profile(membership_id: i64, membership_type: i32, state: &AppState) -> Vec<i64> {
    tracing::info!("Getting profile information");
    let profile_response = if membership_type <= 0 {
        tracing::info!("Missing membership type for: {} doing additional query", membership_id);
        let profile = app::member::memberships_by_id_api(membership_id, state).await;
        if let Some(profile) = profile {
            let target_membership = profile.membership_id.parse::<i64>().unwrap_or_default();
            tracing::info!(
                "Found membership type for: {} at {}:{}",
                membership_id,
                target_membership,
                profile.membership_type as i32
            );
            app::member::profile_api(target_membership, profile.membership_type as i32, state).await
        } else {
            None
        }
    } else {
        app::member::profile_api(membership_id, membership_type, state).await
    };

    // extract characters from our profile response **assuming** we have a profile component included
    tracing::info!("Getting characters ids from {}", membership_id);
    let characters = match &profile_response {
        Some(data) => match &data.profile {
            Some(profile) => match &profile.data {
                Some(data) => data
                    .characters
                    .iter()
                    .map(|character_id| character_id.parse::<i64>().unwrap_or_default())
                    .collect::<Vec<i64>>(),
                _ => Vec::new(),
            },
            _ => Vec::new(),
        },
        _ => Vec::new(),
    };

    tracing::info!("Syncing profile information");
    app::member::profile_api_sync(profile_response, &state.database).await;

    characters
}

/// search for a profile and sync to the database and return relevant information
pub async fn profile_search(bungie_name: &str, state: &AppState) -> Option<ProfileSearchResults> {
    tracing::info!("Searching For: {}", bungie_name);
    let api_result = app::member::search_api(bungie_name, state).await;
    if let Some(api_result) = api_result {
        tracing::info!("Found user info!");
        let membership_id = api_result.membership_id.parse::<i64>().unwrap_or_default();
        let membership_type = api_result.membership_type as i32;

        // get profile information and sync
        let characters = profile(membership_id, membership_type, state).await;

        // return profile results if we can
        Some(ProfileSearchResults {
            membership_id,
            membership_type,
            bungie_name: bungie_name.to_string(),
            characters,
        })
    } else {
        None
    }
}

/// fetch and sync character activites
pub async fn activities(
    membership_id: MembershipId,
    membership_type: MembershipType,
    character_id: CharacterId,
    state: &AppState,
) -> Vec<InstanceId> {
    // determine our starting point timestamp wise of when we allow new data in
    let start_timestamp = database::activity_history::last_activity_timestamp(character_id, &state.database).await;

    tracing::info!("Fetching character history: {}", character_id);
    let activity_history =
        app::activity::character_api(membership_id, membership_type, character_id, start_timestamp, state).await;

    tracing::info!("Syncing character history: {}", character_id);
    let instance_ids = sync::activity::history(
        membership_id,
        membership_type,
        character_id,
        &activity_history,
        &state.database,
    )
    .await;

    tracing::info!("Now syncing character stats for activities: {}", character_id);
    sync::activity::stats(
        membership_id,
        membership_type,
        character_id,
        &activity_history,
        &state.database,
    )
    .await;

    instance_ids
}

/// exclusively request clan information
/// returns a Hash map  that represents (key = membership_id: i64, value = membership_type: i32)
pub async fn clan_roster(group_id: i64, state: &AppState) -> HashMap<i64, i32> {
    tracing::info!("Requesting group information for: {}", group_id);
    let api_response = app::clan::clan_roster_api(group_id, state).await;
    if let Some(api_response) = api_response {
        tracing::info!("Starting to sync for group: {}", group_id);
        let results = app::clan::clan_roster_sync(group_id, &api_response, &state.database).await;
        tracing::info!("Done attempting sync for group: {}", group_id);

        results
    } else {
        HashMap::new()
    }
}

/// query bungie for a particular group id and then sync
pub async fn clan_info(group_id: i64, state: &AppState) {
    tracing::info!("Starting to query bungie api for group: {}", group_id);
    let api_response = app::clan::clan_info_api(group_id, state).await;
    if let Some(api_response) = api_response {
        tracing::info!("Starting to sync for group: {}", group_id);
        app::clan::clan_info_sync(&api_response, &state.database).await;
        tracing::info!("Done attempting sync for group: {}", group_id);
    }
}

/// query bungie database and then sync
pub async fn clan_info_by_membership(membership_id: i64, membership_type: i32, state: &AppState) -> i64 {
    tracing::info!(
        "Starting to query bungie api for group by membership: {}",
        membership_id
    );
    let mut group = 0;
    let api_response = app::clan::from_membership_api(membership_id, membership_type, state).await;
    if let Some(api_response) = api_response {
        group = match api_response.results.first() {
            Some(group_membership) => group_membership.group.group_id.parse::<i64>().unwrap_or_default(),
            _ => 0,
        };

        if group > 0 {
            // technically we can reuse the group data from the response. However our task are already setup to sync and pull a certain response
            // for now just do this since we are already setup to do it like so
            clan_info(group, state).await;
        }
    }

    group
}

/// Primary Task: Take provided input of instance ids and get post carnage report data to get all instance members,instance data we need
/// Secondary Task: Update profile information for each of the instance members
pub async fn instance_data(instance_ids: &[i64], state: &AppState) {
    for instance_id in instance_ids.iter() {
        tracing::info!("Getting carnage report for: {}", *instance_id);
        let response = app::instance::carnage_report_api(*instance_id, state).await;
        if let Some(response) = response {
            app::instance::carnage_report_sync(&response, state).await;
        }
    }

    // for each member run the profile sync task
}
