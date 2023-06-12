use crate::{
    app::state::AppState,
    bungie::{enums::DestinyRouteParam, schemas::DestinyPostGameCarnageReportData},
    database::{self, instance::InstanceMemberRecord},
    sync,
};
use levelcrush::{
    cache::CacheValue,
    types::destiny::{CharacterId, InstanceId, MembershipId, MembershipType},
};
use std::collections::HashMap;

use super::state::CacheItem;

const CACHE_KEY_INSTANCE_MEMBERS: &str = "instance_members||";

/// get the carnage report from the bungie api
pub async fn carnage_report_api(instance_id: InstanceId, state: &AppState) -> Option<DestinyPostGameCarnageReportData> {
    let request = state
        .bungie
        .get("/Destiny2/Stats/PostGameCarnageReport/{activityId}")
        .param(DestinyRouteParam::Activity, instance_id.to_string())
        .send::<DestinyPostGameCarnageReportData>()
        .await;

    request.response
}

/// syncs the carnage report info to the database
/// returns a HashMap<(membership_id:i64, membership_type: i32), Vec<i64>> tied to instance
pub async fn carnage_report_sync(
    report_data: &DestinyPostGameCarnageReportData,
    state: &AppState,
) -> HashMap<(MembershipId, MembershipType), Vec<CharacterId>> {
    // for now we just pass this through directly
    // we may want to do more in the future (maybe)
    sync::activity::instance(report_data, &state.database).await
}

pub async fn multi_get_members(instance_ids: &[InstanceId], state: &mut AppState) -> Vec<InstanceMemberRecord> {
    let instance_ids_str = instance_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let cache_key = format!("{}{}", CACHE_KEY_INSTANCE_MEMBERS, instance_ids_str);
    let (should_update, mut members) = {
        let results = match state.cache.access(&cache_key).await {
            Some(CacheItem::InstanceMemberArray(data)) => Some(data),
            _ => None,
        };
        (results.is_none(), results.unwrap())
    };

    if should_update {
        //. we will only ever get from the database
        members = database::instance::multi_get_members(instance_ids, &state.database).await;

        state
            .cache
            .write(
                cache_key,
                CacheValue::new(CacheItem::InstanceMemberArray(members.clone())),
            )
            .await;
    }

    members
}
