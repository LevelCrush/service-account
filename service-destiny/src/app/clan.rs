use std::collections::HashMap;

use levelcrush::alias::UnixTimestamp;
use sqlx::SqlitePool;

use levelcrush::cache::{CacheDuration, CacheValue};
use levelcrush::tokio;
use levelcrush::alias::destiny::{GroupId, MembershipId, MembershipType};
use levelcrush::util::unix_timestamp;
use levelcrush::{futures, tracing};

use crate::app::state::AppState;
use crate::bungie::schemas::{DestinyGroupResponse, DestinySearchResultOfGroupMember};
use crate::database::activity_history::NetworkBreakdownResult;
use crate::database::clan::ClanInfoResult;
use crate::database::member::MemberResult;
use crate::jobs::task;
use crate::{database, sync};

use super::state::CacheItem;

const CACHE_KEY_CLAN: &str = "clan_info||";
pub const CACHE_KEY_CLAN_ROSTER: &str = "clan_roster||";
const CACHE_KEY_NETWORK_CLANS: &str = "network_clans_info||main";
const CACHE_KEY_NETWORK_CLANS_ROSTER: &str = "network_clans_roster||main";
const CACHE_KEY_CLAN_INFO_MEMBERSHIP: &str = "clan_info_membership||";
const CACHE_KEY_NETWORK_BREAKDOWN: &str = "activity_breakdown||network";

const UPDATE_CLAN_INTERVAL: i64 = 86400;
// 24 hours
const UPDATE_CLAN_NETWORK_INTERVAL: i64 = 3600; // 1 hour

pub async fn from_membership(
    membership_id: MembershipId,
    membership_type: MembershipType,
    state: &mut AppState,
) -> Option<ClanInfoResult> {
    let mut call_api = true;
    let cache_key = format!("{}{}", CACHE_KEY_CLAN_INFO_MEMBERSHIP, membership_id);
    let (should_update, mut clan_info) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::ClanInfo(result)) => Some(*result),
            _ => None,
        };
        (results.is_none(), results)
    };

    if should_update {
        clan_info = database::clan::from_membership(membership_id, &state.database).await;
    }

    if let Some(clan_info) = &clan_info {
        call_api = unix_timestamp() - clan_info.updated_at > UPDATE_CLAN_INTERVAL;
    }

    if call_api {
        let group_id = task::clan_info_by_membership(membership_id, membership_type, state).await;
        if let Ok(group_id) = group_id {
            task::clan_roster(group_id, state).await.ok();
            clan_info = database::clan::from_membership(membership_id, &state.database).await;
        } else {
            let err = group_id.err().unwrap();
            tracing::error!("Error fetching group id from membership {}: {}", membership_id, err);
        }
    }

    if should_update {
        if let Some(clan_info) = &clan_info {
            state
                .cache
                .write(
                    cache_key,
                    CacheValue::new(CacheItem::ClanInfo(Box::new(clan_info.clone()))),
                )
                .await;
        }
    }

    clan_info
}

/// takes the input from a group response from destiny and syncs to the database
pub async fn clan_info_sync(response: &DestinyGroupResponse, state: &SqlitePool) {
    // start syncing to the database
    sync::clan::info(&response.detail, state).await;
}

/// takes the response from the clan_roster_api function call  and then syncs that data to our database
pub async fn clan_roster_sync(
    group_id: GroupId,
    roster_response: &DestinySearchResultOfGroupMember,
    database: &SqlitePool,
) -> HashMap<MembershipId, MembershipType> {
    // at the time of this writing we are just going to pass what we need right to the sync function
    // there may be more we want to do with the roster in the future to return any more data
    sync::clan::roster(group_id, &roster_response.results, database).await
}

/// get clan information about all network clans from the cache/db
pub async fn network(state: &mut AppState) -> Vec<ClanInfoResult> {
    let (should_update, mut network_clans) = {
        let results = match state.cache.access(CACHE_KEY_NETWORK_CLANS).await {
            Some(CacheItem::ClanInfoArray(data)) => Some(data),
            _ => None,
        };

        (results.is_none(), results.unwrap_or_default())
    };

    // if we need to pull from db then do so
    if should_update {
        network_clans = database::clan::get_network_info(&state.database).await;

        state
            .cache
            .write(
                CACHE_KEY_NETWORK_CLANS,
                CacheValue::new(CacheItem::ClanInfoArray(network_clans.clone())),
            )
            .await;
    }

    network_clans
}

pub async fn network_roster(state: &mut AppState) -> Vec<MemberResult> {
    let (should_update, mut roster) = {
        let results = match state.cache.access(CACHE_KEY_NETWORK_CLANS_ROSTER).await {
            Some(CacheItem::MemberArray(data)) => Some(data),
            _ => None,
        };

        (results.is_none(), results.unwrap_or_default())
    };

    if should_update {
        roster = database::clan::get_network_roster(&state.database).await;

        state
            .cache
            .write(
                CACHE_KEY_NETWORK_CLANS_ROSTER,
                CacheValue::new(CacheItem::MemberArray(roster.clone())),
            )
            .await;
    }

    roster
}

pub async fn get_roster(group_id: GroupId, state: &mut AppState) -> Vec<MemberResult> {
    let cache_key = format!("{}{}", CACHE_KEY_CLAN_ROSTER, group_id);
    let (should_update, mut roster) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::MemberArray(data)) => Some(data),
            _ => None,
        };

        (results.is_none(), results.unwrap_or_default())
    };

    if should_update {
        roster = database::clan::get_roster(group_id, &state.database).await;

        state
            .cache
            .write(&cache_key, CacheValue::new(CacheItem::MemberArray(roster.clone())))
            .await;
    }

    roster
}

pub async fn get(group_id: GroupId, state: &mut AppState) -> Option<ClanInfoResult> {
    let cache_key = format!("{}{}", CACHE_KEY_CLAN, group_id);
    let mut call_api = true;
    let (mut should_update, mut clan_info) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::ClanInfo(data)) => Some(*data),
            _ => None,
        };

        (results.is_none(), results)
    };

    if clan_info.is_none() {
        clan_info = database::clan::get_info(group_id, &state.database).await;
    }

    if let Some(result) = &clan_info {
        let target_interval = if result.is_network == 1 {
            UPDATE_CLAN_NETWORK_INTERVAL
        } else {
            UPDATE_CLAN_INTERVAL
        };
        call_api = unix_timestamp() - result.updated_at > target_interval;
    }

    if call_api {
        // assume we cannot update until we at least have our clan info
        should_update = false;

        let arc_ci_state = state.clone();
        let arc_roster_state = state.clone();

        let clan_info_api_future =
            tokio::spawn(async move { lib_destiny::api::clan::info(group_id, &arc_ci_state.bungie).await });
        let clan_roster_api_future =
            tokio::spawn(async move { lib_destiny::api::clan::roster(group_id, &arc_roster_state.bungie).await });
        let (clan_info_result, clan_roster_result) = tokio::join!(clan_info_api_future, clan_roster_api_future);

        let mut future_handles = Vec::with_capacity(2);
        if let Ok(clan_info_result) = clan_info_result {
            let arc_ci_state = state.clone(); // reclone our handle
            future_handles.push(tokio::spawn(async move {
                if let Ok(Some(clan_info_result)) = &clan_info_result {
                    clan_info_sync(clan_info_result, &arc_ci_state.database).await;
                } else {
                    if let Err(err) = clan_info_result {
                        tracing::error!("Clan Info Err for {} : {}", group_id, err);
                    }
                }
            }));

            should_update = true;
        }

        if let Ok(clan_roster_result) = clan_roster_result {
            let arc_roster_state = state.clone(); // reclone our handle
            future_handles.push(tokio::spawn(async move {
                if let Ok(Some(clan_roster_result)) = &clan_roster_result {
                    clan_roster_sync(group_id, &clan_roster_result, &arc_roster_state.database).await;
                } else {
                    if let Err(err) = clan_roster_result {
                        tracing::error!("Clan Roster Fetch Error for {}: {}", group_id, err);
                    }
                }
            }));
        }

        futures::future::join_all(future_handles).await;

        // if we should update, requery the database for our result set
        if should_update {
            clan_info = database::clan::get_info(group_id, &state.database).await;
        }
    }

    if should_update {
        if let Some(clan_info) = &clan_info {
            state
                .cache
                .write(
                    cache_key,
                    CacheValue::new(CacheItem::ClanInfo(Box::new(clan_info.clone()))),
                )
                .await;
        }
    }

    clan_info
}

pub async fn get_by_slug(slug: &str, state: &mut AppState) -> Option<ClanInfoResult> {
    let cache_key = format!("{}slug||{}", CACHE_KEY_CLAN, slug);
    let mut call_api = false; // by default we don't want to call our api
    let mut group_id = 0;
    let (mut should_update, mut clan_info) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::ClanInfo(data)) => Some(*data),
            _ => None,
        };

        (results.is_none(), results)
    };

    if clan_info.is_none() {
        clan_info = database::clan::get_info_by_slug(slug, &state.database).await;
    }

    if let Some(result) = &clan_info {
        group_id = result.group_id;
        let target_interval = if result.is_network == 1 {
            UPDATE_CLAN_NETWORK_INTERVAL
        } else {
            UPDATE_CLAN_INTERVAL
        };
        call_api = unix_timestamp() - result.updated_at > target_interval;
    }

    if call_api {
        // assume we cannot update until we at least have our clan info
        should_update = false;

        let arc_ci_state = state.clone();
        let arc_roster_state = state.clone();

        let clan_info_api_future =
            tokio::spawn(async move { lib_destiny::api::clan::info(group_id, &arc_ci_state.bungie).await });
        let clan_roster_api_future =
            tokio::spawn(async move { lib_destiny::api::clan::roster(group_id, &arc_roster_state.bungie).await });
        let (clan_info_result, clan_roster_result) = tokio::join!(clan_info_api_future, clan_roster_api_future);

        let mut future_handles = Vec::with_capacity(2);
        if let Ok(clan_info_result) = clan_info_result {
            let arc_ci_state = state.clone(); // reclone our handle
            future_handles.push(tokio::spawn(async move {
                if let Ok(Some(clan_info_result)) = &clan_info_result {
                    clan_info_sync(clan_info_result, &arc_ci_state.database).await;
                } else {
                    if let Err(err) = clan_info_result {
                        tracing::error!("Clan Info Err for {} : {}", group_id, err);
                    }
                }
            }));

            should_update = true;
        }

        if let Ok(clan_roster_result) = clan_roster_result {
            let arc_roster_state = state.clone(); // reclone our handle
            future_handles.push(tokio::spawn(async move {
                if let Ok(Some(clan_roster_result)) = &clan_roster_result {
                    clan_roster_sync(group_id, &clan_roster_result, &arc_roster_state.database).await;
                } else {
                    if let Err(err) = clan_roster_result {
                        tracing::error!("Clan Roster Fetch Error for {}: {}", group_id, err);
                    }
                }
            }));
        }

        futures::future::join_all(future_handles).await;

        // if we should update, requery the database for our result set
        if should_update {
            clan_info = database::clan::get_info(group_id, &state.database).await;
        }
    }

    if should_update {
        if let Some(clan_info) = &clan_info {
            state
                .cache
                .write(
                    cache_key,
                    CacheValue::new(CacheItem::ClanInfo(Box::new(clan_info.clone()))),
                )
                .await;
        }
    }

    clan_info
}

/// gets network activity breakdown for clans in the network
///
/// Note: Feeding a value of 0 to timestamp_end will make it pull the current unix timestamp
pub async fn network_breakdown(
    modes: &[i64],
    timestamp_start: UnixTimestamp,
    timestamp_end: UnixTimestamp,
    state: &mut AppState,
) -> HashMap<i64, NetworkBreakdownResult> {
    let mode_str = modes.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",");

    let (timestamp_end_key, timestamp_end) = if timestamp_end == 0 {
        ("lifetime".to_string(), unix_timestamp())
    } else {
        (timestamp_end.to_string(), timestamp_end)
    };

    let cache_key = format!(
        "{}||{}||{}||{}",
        CACHE_KEY_NETWORK_BREAKDOWN, mode_str, timestamp_start, timestamp_end_key
    );

    let (mut should_update, mut result) = {
        let cache = match state.cache.access(&cache_key).await {
            Some(CacheItem::NetworkBreakdown(data)) => Some(data),
            _ => None,
        };
        (cache.is_none(), cache.unwrap_or_default())
    };

    let mut did_lock = false;
    let mut retries = 0;
    if should_update {
        tracing::warn!("Not in cache for network activity breakdown, locking");
        retries = state.locks.lock(&cache_key).await;
        did_lock = true;
    }

    // we are trying to update, but when we went to lock we encountered some retries.
    // check cache again
    if should_update && retries > 0 {
        tracing::warn!("Attempts to get activity breakdown: {}", retries);
        (should_update, result) = {
            let cache = match state.cache.access(&cache_key).await {
                Some(CacheItem::NetworkBreakdown(data)) => Some(data),
                _ => None,
            };
            (cache.is_none(), cache.unwrap_or_default())
        };
    }

    if should_update {
        tracing::warn!("Calling DB for network activity breakdown");
        let db_result =
            database::activity_history::network_breakdown(modes, timestamp_start, timestamp_end, &state.database).await;

        state
            .cache
            .write(
                &cache_key,
                CacheValue::exact(CacheItem::NetworkBreakdown(db_result.clone()), CacheDuration::OneHour),
            )
            .await;

        result = db_result;
    }

    if did_lock {
        tracing::warn!("Unlocking: {}", cache_key);
        state.locks.unlock(&cache_key).await;
    }

    result
}
