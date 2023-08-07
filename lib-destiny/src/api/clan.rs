use crate::aliases::*;
use crate::bungie::enums::DestinyRouteParam;
use crate::bungie::schemas::{DestinyGroupResponse, DestinySearchResultOfGroupMember, GetGroupsForMemberResponse};
use crate::bungie::BungieClient;
use levelcrush::anyhow;

/// get clan info by querying the bungie api via membership id and type
pub async fn from_membership(
    membership_id: MembershipId,
    membership_type: MembershipType,
    bungie: &BungieClient,
) -> anyhow::Result<Option<GetGroupsForMemberResponse>> {
    let request = bungie
        .get("/GroupV2/User/{membershipType}/{membershipId}/0/1")
        .param(DestinyRouteParam::PlatformMembershipID, membership_id.to_string())
        .param(DestinyRouteParam::PlatformMembershipType, membership_type.to_string())
        .send::<GetGroupsForMemberResponse>()
        .await?;

    Ok(request.response)
}

/// queries the clan info via the bungie api
pub async fn info(group_id: GroupId, bungie: &BungieClient) -> anyhow::Result<Option<DestinyGroupResponse>> {
    let request = bungie
        .get("/GroupV2/{groupId}/")
        .param(DestinyRouteParam::GroupID, group_id.to_string())
        .send::<DestinyGroupResponse>()
        .await?;

    Ok(request.response)
}

/// queries the clan roster information from the bungie api
pub async fn roster(
    group_id: GroupId,
    bungie: &BungieClient,
) -> anyhow::Result<Option<DestinySearchResultOfGroupMember>> {
    let request = bungie
        .get("/GroupV2/{groupId}/Members")
        .param(DestinyRouteParam::GroupID, group_id.to_string())
        .send::<DestinySearchResultOfGroupMember>()
        .await?;

    Ok(request.response)
}
