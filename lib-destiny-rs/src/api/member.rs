use crate::aliases::*;
use crate::bungie::enums::{DestinyComponentType, DestinyRouteParam};
use crate::bungie::schemas::{DestinyCharacterComponent, DestinyProfileResponse, UserInfoCard, UserMembershipData};
use crate::bungie::{BungieClient, BungieRequestBodyType};
use levelcrush::anyhow;
use levelcrush::anyhow::anyhow;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct UserExactSearchResult {
    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "displayNameCode")]
    pub code: i16,
}

/// this is setup by comparing bungie net platform membership types
/// and comparing to raid report platforms
/// https://bungie-net.github.io/#/components/schemas/BungieMembershipType
pub const fn get_platform_name(membership_type: MembershipType) -> &'static str {
    match membership_type {
        0 => "none", // 0 is verified to be none
        1 => "xb",   // 1 is verified to be xbox
        2 => "ps",   // 2 is verified to be playstation
        _ => "pc", // other numbers either result in epic game store, steam, battle.net, or unknown numbers. Values like -1 are not possible and value 254 is reserved and not used
    }
}

pub fn generate_raid_report_url(membership_id: MembershipId, membership_type: MembershipType) -> String {
    let platform_name = get_platform_name(membership_type);
    format!("https://raid.report/{}/{}", platform_name, membership_id)
}

pub async fn memberships_by_id(
    membership_id: MembershipId,
    bungie: &BungieClient,
) -> anyhow::Result<Option<UserInfoCard>> {
    let request = bungie
        .get("/User/GetMembershipsById/{membershipId}/0")
        .param(DestinyRouteParam::PlatformMembershipID, membership_id.to_string())
        .send::<UserMembershipData>()
        .await?;

    if let Some(response) = request.response {
        if response.memberships.len() == 1 {
            Ok(Some(response.memberships.first().unwrap().clone()))
        } else {
            // if this user has more then one user card, they have multiple linked platforms, and **should** crossSaveOverride populated
            let mut target_card = None;
            let input_membership_str = membership_id.to_string();
            for user_card in response.memberships.into_iter() {
                if user_card.membership_id == input_membership_str {
                    target_card = Some(user_card);
                    break;
                } else if user_card.membership_type == user_card.cross_save_override {
                    target_card = Some(user_card);
                }
            }
            Ok(target_card)
        }
    } else {
        Ok(None)
    }
}

/// This will return a profile of information about the membership associated; given from the official bungie api
pub async fn profile(
    membership_id: MembershipId,
    membership_type: MembershipType,
    bungie: &BungieClient,
) -> anyhow::Result<Option<DestinyProfileResponse>> {
    let request = bungie
        .get("/Destiny2/{membershipType}/Profile/{membershipId}/")
        .param(DestinyRouteParam::PlatformMembershipType, membership_type.to_string())
        .param(DestinyRouteParam::PlatformMembershipID, membership_id.to_string())
        .component(DestinyComponentType::Profiles)
        .component(DestinyComponentType::Characters)
        .component(DestinyComponentType::Records)
        .send::<DestinyProfileResponse>()
        .await?;

    Ok(request.response)
}

/// explicitly searches the bungie api based off the bungie name provided
pub async fn search(bungie_name: &str, bungie: &BungieClient) -> anyhow::Result<Option<UserInfoCard>> {
    let mut target_code = "";
    let target_name = {
        // split the bungie name by #, we need to do this so we can separate out the discriminator
        let name_split = bungie_name.split('#').last();
        if let Some(input_discriminator) = name_split {
            target_code = input_discriminator;
            let replace_str = format!("#{}", target_code);
            bungie_name.replace(replace_str.as_str(), "")
        } else {
            bungie_name.to_string()
        }
    };

    let request = bungie
        .post("/Destiny2/SearchDestinyPlayerByBungieName/{membershipType}/")
        .param(DestinyRouteParam::PlatformMembershipType, "All")
        .body(
            Some(UserExactSearchResult {
                display_name: target_name,
                code: target_code.parse::<i16>().unwrap_or_default(),
            }),
            BungieRequestBodyType::JSON,
        )
        .send::<Vec<UserInfoCard>>()
        .await?;

    let mut active_card = None;
    if let Some(response) = request.response {
        active_card = if response.len() == 1 {
            Some(response.first().unwrap().clone())
        } else {
            // if this user has more then one user card, they have multiple linked platforms, and **should** crossSaveOverride populated
            let mut target_card = None;
            for user_card in response.into_iter() {
                if user_card.membership_type == user_card.cross_save_override {
                    target_card = Some(user_card);
                    break;
                } else {
                    target_card = Some(user_card);
                }
            }
            target_card
        };
    }

    Ok(active_card)
}
