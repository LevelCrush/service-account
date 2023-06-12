use crate::app::state::AppState;
use crate::bungie::enums::DestinyRouteParam;
use crate::bungie::schemas::{
    DestinyActivityHistoryResults, DestinyHistoricalStatsAccountResult, DestinyHistoricalStatsPeriodGroup,
};
use levelcrush::tracing;
use levelcrush::types::{destiny::CharacterId, destiny::MembershipId, destiny::MembershipType, UnixTimestamp};

const RESULTS_PER_PAGE: i64 = 250;

/// queries the bungie api to get overall member stats
pub async fn member_stats_api(
    membership_id: MembershipId,
    membership_type: MembershipType,
    state: &AppState,
) -> Option<DestinyHistoricalStatsAccountResult> {
    let membership_id = membership_id.to_string();
    let membership_type = membership_type.to_string();
    let request = state
        .bungie
        .get("/Destiny2/{membershipType}/Account/{membershipId}/Stats/")
        .param(DestinyRouteParam::PlatformMembershipID, membership_id.as_str())
        .param(DestinyRouteParam::PlatformMembershipType, membership_type.as_str())
        .send::<DestinyHistoricalStatsAccountResult>()
        .await;

    request.response
}

/// queries bungie api for character activity history by page
pub async fn activities_api_page(
    membership_id: MembershipId,
    membership_type: MembershipType,
    character_id: CharacterId,
    page: u32,
    state: &AppState,
) -> Option<DestinyActivityHistoryResults> {
    let request = state
        .bungie
        .get("/Destiny2/{membershipType}/Account/{membershipId}/Character/{characterId}/Stats/Activities")
        .param(DestinyRouteParam::PlatformMembershipID, membership_id.to_string())
        .param(DestinyRouteParam::PlatformMembershipType, membership_type.to_string())
        .param(DestinyRouteParam::Character, character_id.to_string())
        .query("page", page.to_string())
        .query("count", RESULTS_PER_PAGE.to_string())
        .send::<DestinyActivityHistoryResults>()
        .await;

    request.response
}

/// handles fetching all character stats + activities from the bungie api.
/// uses a start timestamp to determine what data to collect
///
/// Note: this will ping as many pages as possible (depending on how far back we need to go through) until it can get no more response
pub async fn character_api(
    membership_id: MembershipId,
    membership_type: MembershipType,
    character_id: CharacterId,
    start_timestamp: UnixTimestamp,
    state: &AppState,
) -> Vec<DestinyHistoricalStatsPeriodGroup> {
    // only query the information if we have activities to query
    let mut activities = Vec::new();

    let mut page = 0;
    tracing::info!(
        "Now working on page {} for {} ({}), start time = {}",
        page,
        membership_id,
        membership_type,
        start_timestamp
    );
    while let Some(response) = activities_api_page(membership_id, membership_type, character_id, page, state).await {
        // normally we would want to borrow the data, but this time we will consume it
        let mut found_activities = 0;
        for activity in response.activities {
            let timestamp = activity.period.timestamp() as u64;
            if timestamp > start_timestamp {
                // the date of this activity is greater then our start time that we want. Include it!
                activities.push(activity);
                found_activities += 1;
            }
        }

        tracing::info!("Found: {} total activities on page: {}", found_activities, page);

        // onto the next page
        if found_activities == 0 {
            tracing::info!("Breaking page loop");
            // break out right now, no need to keep going if we have zero activities found
            break;
        } else {
            page += 1;
            tracing::info!(
                "Now working on page {} for {} ({})",
                page,
                membership_id,
                membership_type
            );
        }
    }

    activities
}
