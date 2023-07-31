use levelcrush::types::UnixTimestamp;
use sqlx::SqlitePool;

use levelcrush::cache::CacheValue;
use levelcrush::futures::future::join_all;
use levelcrush::tokio;
use levelcrush::tracing;
use levelcrush::types::destiny::MembershipId;
use levelcrush::util::unix_timestamp;

use crate::app::state::{AppState, CacheItem};
use crate::bungie::schemas::{DestinyCharacterComponent, DestinyProfileResponse};
use crate::database::activity_history::ActivityHistoryRecord;
use crate::database::member::{MemberRecord, MemberResult};
use crate::database::triumph::TriumphTitleResult;
use crate::{database, jobs, sync};

/// max seconds to allow before pulling fresh data
const CACHE_KEY_BUNGIE_NAME: &str = "member_bn||";
const CACHE_KEY_MEMBERSHIP_ID: &str = "member_membership_id||";
const UPDATE_PROFILE_INTERVAL: i64 = 86400;
const UPDATE_ACTIVITY_INTERVAL: i64 = 86400;
// one day
const CACHE_KEY_ACTIVITIES: &str = "member_activities||";
const CACHE_KEY_MEMBER_SEARCH: &str = "search_members||";
const CACHE_KEY_MEMBER_SEARCH_COUNT: &str = "search_members_count||";
const CACHE_KEY_TITLE: &str = "member_titles||";

/// determine by the provided timestmap if the profile needs to be refreshed.
fn profile_needs_refresh(timestamp: i64) -> bool {
    let timestamp_now = unix_timestamp();
    let time_since_update = timestamp_now - timestamp;
    time_since_update >= UPDATE_PROFILE_INTERVAL
}

/// exclusively handles syncing a direct profile response from the bungie api
pub async fn profile_api_sync(
    profile_response: Option<DestinyProfileResponse>,
    database: &SqlitePool,
) -> Option<MemberRecord> {
    let mut db_record = None;
    let mut profile_component = None;
    let mut characters_component = None;
    let mut records_component = None;
    let mut membership_id = 0;
    if let Some(profile_response) = profile_response {
        profile_component = profile_response.profile;
        characters_component = profile_response.characters;
        records_component = profile_response.records;
    }

    // sync profile data
    if let Some(profile_component) = profile_component {
        // sync the profile first
        if let Some(data) = &profile_component.data {
            sync::member::profile(data, database).await;
            // get membership id from our response
            membership_id = data.user_info.membership_id.parse::<i64>().unwrap_or_default();
        }

        // now fetch the database version
        // note: if syncing fails, then of course our result will be None here.
        db_record = database::member::get_record(membership_id, database).await;
    }

    // sync characters if possible (this is just character data, not character activities)
    if let Some(characters_component) = characters_component {
        if let Some(data) = characters_component.data {
            let characters = data.into_values().collect::<Vec<DestinyCharacterComponent>>();

            // send off, we don't care about fetching the characters after. We just want to sync the profile in this case
            sync::character::multiple(&characters, database).await;
        }
    }

    if let Some(records_component) = records_component {
        if let Some(data) = records_component.data {
            sync::member::triumphs(membership_id, &data.records, database).await;
        }
    }

    db_record
}

async fn cache_member_write(cache_key: &String, record: Box<MemberResult>, state: &mut AppState) {
    tracing::info!("Saving in cache as {}", cache_key);
    state
        .cache
        .write(cache_key, CacheValue::new(CacheItem::Member(record)))
        .await;
}

/// searches for a profile by the membership id and membership type. Checks DB first, then goes to bungie api if needed
pub async fn profile(membership_id: i64, state: &mut AppState) -> Option<MemberResult> {
    let mut call_api = true;

    // generate key
    let cache_key = format!("{}{}", CACHE_KEY_MEMBERSHIP_ID, membership_id);

    // check cache for this data first, and if nothing is found then get it from the database
    let (mut should_cache, mut member_record) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::Member(record)) => Some(*record),
            _ => None,
        };
        (results.is_none(), results)
    };

    if member_record.is_none() {
        member_record = database::member::get(membership_id, &state.database).await;
    }

    // now check if this profile needs to be refereshed
    if let Some(record) = &member_record {
        call_api = profile_needs_refresh(record.updated_at);
    }

    // not found, query the destiny api now
    if call_api {
        let user_card = lib_destiny::api::member::memberships_by_id(membership_id, &state.bungie).await;
        if let Ok(Some(user_card)) = user_card {
            let profile_response =
                lib_destiny::api::member::profile(membership_id, user_card.membership_type as i64, &state.bungie).await;

            if let Ok(profile_response) = profile_response {
                profile_api_sync(profile_response, &state.database).await;

                // refetch
                member_record = database::member::get(membership_id, &state.database).await;
                should_cache = true;
            } else {
                let err = profile_response.err().unwrap();
                tracing::error!("Profile Response: {} ", err);
            }
        } else if let Err(user_card) = user_card {
            tracing::error!("Could not get membership {}; Error: {}", membership_id, user_card);
        }
    }

    if let Some(member) = &member_record {
        if should_cache {
            cache_member_write(&cache_key, Box::new(member.clone()), state).await;
        }
    }

    member_record
}

/// searches for a member by bungie name
pub async fn by_bungie_name(bungie_name: &str, state: &mut AppState) -> Option<MemberResult> {
    let mut call_api = true;

    // generate key
    let cache_key = format!("{}{}", CACHE_KEY_BUNGIE_NAME, bungie_name.to_lowercase());

    // check cache for this data first, and if nothing is found then get it from the database
    tracing::info!("Looking for {} in Cache", bungie_name);
    let (mut should_cache, mut member_record) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::Member(record)) => Some(*record),
            _ => None,
        };
        (results.is_none(), results)
    };
    if member_record.is_none() {
        tracing::info!("Looking for {} in DB", bungie_name);

        member_record = database::member::get_by_bungie_name(bungie_name, &state.database).await;
    }

    // now check if this profile needs to be refreshed
    if let Some(record) = &member_record {
        call_api = profile_needs_refresh(record.updated_at);
    }

    // we need to query the api or refresh the profile data
    if call_api {
        tracing::info!("Reaching out to bungie server for: {}", bungie_name);
        let api_result = lib_destiny::api::member::search(bungie_name, &state.bungie).await;
        if let Ok(Some(api_result)) = &api_result {
            tracing::info!("Found user info!: {}", bungie_name);
            let membership_id = api_result.membership_id.parse::<i64>().unwrap_or_default();
            let membership_type = api_result.membership_type as i64;

            tracing::info!("Getting profile information for {}", bungie_name);
            let profile_response =
                lib_destiny::api::member::profile(membership_id, membership_type, &state.bungie).await;

            if let Ok(profile_response) = profile_response {
                tracing::info!("Syncing profile information for {}", bungie_name);
                profile_api_sync(profile_response, &state.database).await;

                // re fetch record
                member_record = database::member::get_by_bungie_name(bungie_name, &state.database).await;

                should_cache = true;
            } else {
                let err = profile_response.err().unwrap();
                tracing::error!("Profile Response Err: {}", err);
            }
        } else if let Err(err) = api_result {
            tracing::error!("Unable to search member: {}", err);
        }
    }

    if let Some(member) = &member_record {
        if should_cache {
            cache_member_write(&cache_key, Box::new(member.clone()), state).await;
        }
    }

    member_record
}

pub async fn search<T: Into<String>>(
    display_name: T,
    page: u32,
    limit: u32,
    state: &mut AppState,
) -> Vec<MemberResult> {
    let display_name = display_name.into();
    let cache_key = format!(
        "{}term||{}||page||{}||limit||{}",
        CACHE_KEY_MEMBER_SEARCH, display_name, page, limit
    );

    let (should_update, mut records) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::MemberArray(data)) => Some(data),
            _ => None,
        };
        (results.is_none(), results.unwrap_or_default())
    };

    if should_update {
        records = database::member::search(display_name, page, limit, &state.database).await;

        state
            .cache
            .write(cache_key, CacheValue::new(CacheItem::MemberArray(records.clone())))
            .await;
    }

    records
}

pub async fn search_count<T: Into<String>>(display_name: T, state: &mut AppState) -> u32 {
    let display_name = display_name.into();
    let cache_key = format!("{}{}", CACHE_KEY_MEMBER_SEARCH_COUNT, display_name);

    let (should_update, mut count) = {
        let result = match state.cache.access(&cache_key).await {
            Some(CacheItem::MemberSearchCount(reported_count)) => Some(reported_count),
            _ => None,
        };
        (result.is_none(), result.unwrap_or_default())
    };

    if should_update {
        count = database::member::search_count(display_name, &state.database).await;

        // write into the cache overwriting the current value there even it is 0
        // 0 is valid
        state
            .cache
            .write(cache_key, CacheValue::new(CacheItem::MemberSearchCount(count)))
            .await;
    }

    count
}

pub async fn titles(membership_id: MembershipId, state: &mut AppState) -> Vec<TriumphTitleResult> {
    let cache_key = format!("{}{}", CACHE_KEY_TITLE, membership_id);

    let (should_update, mut titles) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::MemberTitles(data)) => Some(data),
            _ => None,
        };
        (results.is_none(), results.unwrap_or_default())
    };

    if should_update {
        titles = database::triumph::member_titles(membership_id, &state.database).await;

        state
            .cache
            .write(cache_key, CacheValue::new(CacheItem::MemberTitles(titles.clone())))
            .await;
    }

    titles
}

pub async fn activities(
    membership_id: MembershipId,
    timestamp_start: UnixTimestamp,
    timestamp_end: UnixTimestamp,
    modes: &[i64],
    state: &mut AppState,
) -> Vec<ActivityHistoryRecord> {
    let mut call_api = false;
    let mode_str = modes
        .iter()
        .map(|mode| mode.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let cache_key = format!(
        "{}{}||{}||{}||{}",
        CACHE_KEY_ACTIVITIES, membership_id, timestamp_start, timestamp_end, mode_str
    );

    let (should_update, mut member_activities) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::ActivityHistoryArray(data)) => Some(data),
            _ => None,
        };
        (results.is_none(), results.unwrap_or_default())
    };

    if should_update {
        member_activities =
            database::activity_history::member(membership_id, timestamp_start, timestamp_end, modes, &state.database)
                .await;

        // most recent activities are at top. Check the most recent entry and if its been more then > X amount of time pull new data
        let recent = member_activities.first();
        if let Some(record) = recent {
            call_api = unix_timestamp() - record.occurred_at > UPDATE_ACTIVITY_INTERVAL;
        }
    }

    if call_api {
        let member = profile(membership_id, state).await;
        if let Some(member) = member {
            // this is wildly inn efficient. Do not send up to production like this
            let characters = jobs::task::profile(membership_id, member.platform, state).await;
            if let Ok(characters) = characters {
                let mut future_handles = Vec::with_capacity(3);
                for character in characters.into_iter() {
                    let state_clone = state.clone();
                    let platform = member.platform;
                    future_handles.push(tokio::spawn(async move {
                        jobs::task::activities(membership_id, platform, character, &state_clone)
                            .await
                            .ok();
                    }));
                }
                join_all(future_handles).await;
            } else {
                let err = characters.err().unwrap();
                tracing::error!("Could not fetch activities for member {}: {}", membership_id, err);
            }
        }

        member_activities =
            database::activity_history::member(membership_id, timestamp_start, timestamp_end, modes, &state.database)
                .await;
    }

    if should_update {
        state
            .cache
            .write(
                cache_key,
                CacheValue::new(CacheItem::ActivityHistoryArray(member_activities.clone())),
            )
            .await;
    }

    member_activities
}
