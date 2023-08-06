use levelcrush::anyhow;
use levelcrush::alias::UnixTimestamp;

use crate::aliases::*;
use crate::bungie::enums::DestinyRouteParam;
use crate::bungie::schemas::{
    DestinyActivityHistoryResults, DestinyHistoricalStatsAccountResult, DestinyHistoricalStatsPeriodGroup,
};
use crate::bungie::BungieClient;

const RESULTS_PER_PAGE: i64 = 250;

/// queries the bungie api to get overall member stats
pub async fn member(
    membership_id: MembershipId,
    membership_type: MembershipType,
    bungie: &BungieClient,
) -> anyhow::Result<Option<DestinyHistoricalStatsAccountResult>> {
    let membership_id = membership_id.to_string();
    let membership_type = membership_type.to_string();
    let request = bungie
        .get("/Destiny2/{membershipType}/Account/{membershipId}/Stats/")
        .param(DestinyRouteParam::PlatformMembershipID, membership_id.as_str())
        .param(DestinyRouteParam::PlatformMembershipType, membership_type.as_str())
        .send::<DestinyHistoricalStatsAccountResult>()
        .await?;

    Ok(request.response)
}

/// queries bungie api for character activity history by page
pub async fn activities_page(
    membership_id: MembershipId,
    membership_type: MembershipType,
    character_id: CharacterId,
    page: u32,
    bungie: &BungieClient,
) -> anyhow::Result<Option<DestinyActivityHistoryResults>> {
    let request = bungie
        .get("/Destiny2/{membershipType}/Account/{membershipId}/Character/{characterId}/Stats/Activities")
        .param(DestinyRouteParam::PlatformMembershipID, membership_id.to_string())
        .param(DestinyRouteParam::PlatformMembershipType, membership_type.to_string())
        .param(DestinyRouteParam::Character, character_id.to_string())
        .query("page", page.to_string())
        .query("count", RESULTS_PER_PAGE.to_string())
        .send::<DestinyActivityHistoryResults>()
        .await?;

    Ok(request.response)
}

/// handles fetching all character stats + activities from the bungie api.
/// uses a start timestamp to determine what data to collect
///
/// Note: this will ping as many pages as possible (depending on how far back we need to go through) until it can get no more response
pub async fn character(
    membership_id: MembershipId,
    membership_type: MembershipType,
    character_id: CharacterId,
    start_timestamp: UnixTimestamp,
    bungie: &BungieClient,
) -> anyhow::Result<Vec<DestinyHistoricalStatsPeriodGroup>> {
    // only query the information if we have activities to query
    let mut activities = Vec::new();
    let mut page = 0;

    while let Some(response) = activities_page(membership_id, membership_type, character_id, page, bungie).await? {
        // normally we would want to borrow the data, but this time we will consume it
        let mut found_activities = 0;
        for activity in response.activities {
            let timestamp = activity.period.timestamp();
            if timestamp > start_timestamp {
                // the date of this activity is greater then our start time that we want. Include it!
                activities.push(activity);
                found_activities += 1;
            }
        }

        // onto the next page
        if found_activities == 0 {
            // break out right now, no need to keep going if we have zero activities found
            break;
        } else {
            page += 1;
        }
    }
    Ok(activities)
}
