use crate::bungie::enums::{BungieClassType, BungieMembershipType, DestinyActivityModeType};
use levelcrush::chrono;
use levelcrush::macros::ExternalAPIResponse;
use std::collections::HashMap;

///This contract supplies basic information commonly used to display a minimal amount of information about a user.
///
/// **Source**: [Bungie Documentation](https://bungie-net.github.io/#/components/schemas/User.UserInfoCard)
#[ExternalAPIResponse]
pub struct UserInfoCard {
    #[serde(rename = "applicableMembershipTypes", default)]
    pub applicable_membership_types: Vec<BungieMembershipType>,

    #[serde(rename = "crossSaveOverride", default)]
    pub cross_save_override: BungieMembershipType,

    #[serde(rename = "membershipId")]
    pub membership_id: String, // technically membershipID should be a i64, but the api serves it as a string

    #[serde(rename = "membershipType")]
    pub membership_type: BungieMembershipType,

    #[serde(rename = "displayName", default)]
    pub display_name: String,

    #[serde(rename = "supplementalDisplayName", default)]
    pub supplemental_display_name: String,

    #[serde(rename = "LastSeenDisplayName", default)]
    pub last_seen_display_name: String,

    #[serde(rename = "LastSeenDisplayNameType", default)]
    pub last_seen_platform: i32,

    #[serde(rename = "bungieGlobalDisplayName", default)]
    pub global_display_name: String,

    #[serde(rename = "bungieGlobalDisplayNameCode", default)]
    pub global_display_name_code: i32,
}

/// Contains more relevant information about a membership profile
///
/// **Source** [Bungie Documentation](https://bungie-net.github.io/#/components/schemas/Destiny.Entities.Profiles.DestinyProfileComponent)
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct DestinyProfileComponent {
    #[serde(rename = "dateLastPlayed")]
    pub date_last_played: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "userInfo")]
    pub user_info: UserInfoCard,

    #[serde(rename = "characterIds")]
    pub characters: Vec<String>,

    #[serde(rename = "currentGuardianRank")]
    pub guardian_rank_current: u8,

    #[serde(rename = "lifetimeHighestGuardianRank")]
    pub guardian_rank_lifetime: u8,
}

/// Information related to characters tied to a membership
///
/// **Note** not all bungie provided fields are included on our side since we only care about a subset. See source linked below for a full rundown of what is possible
///
/// **Source** [Bungie Documentation](https://bungie-net.github.io/#/components/schemas/Destiny.Entities.Characters.DestinyCharacterComponent)
#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct DestinyCharacterComponent {
    #[serde(rename = "membershipId")]
    pub membership_id: String,

    #[serde(rename = "membershipType")]
    pub membership_type: BungieMembershipType,

    /// this is technically an i64, but the api returns it as a string
    #[serde(rename = "characterId")]
    pub character_id: String,

    #[serde(rename = "dateLastPlayed")]
    pub last_played: chrono::DateTime<chrono::Utc>,

    /// this is a field that is technically a i64, but api returns it as a String instead
    #[serde(rename = "minutesPlayedThisSession")]
    pub minutes_played_session: String,

    #[serde(rename = "minutesPlayedTotal")]
    pub minutes_played_total: String,

    pub light: i32,

    #[serde(rename = "classHash")]
    pub class_hash: u32,

    #[serde(rename = "classType")]
    pub class_type: BungieClassType,

    #[serde(rename = "emblemPath", default)]
    pub emblem_path: String,

    #[serde(rename = "emblemBackgroundPath", default)]
    pub emblem_background_path: String,

    #[serde(rename = "emblemHash", default)]
    pub emblem_hash: u32,
}

pub type DestinyCharacterComponentDictionary = HashMap<String, DestinyCharacterComponent>;

///https://bungie-net.github.io/#/components/schemas/Destiny.Components.Records.DestinyRecordComponent
#[ExternalAPIResponse]
pub struct DestinyRecordComponent {
    pub state: i32,

    #[serde(rename = "completedCount", default)]
    pub completed_count: i32,
}

pub type DestinyRecordComponentMap = HashMap<String, DestinyRecordComponent>;

/// https://bungie-net.github.io/#/components/schemas/Destiny.Components.Records.DestinyProfileRecordsComponent
#[ExternalAPIResponse]
pub struct DestinyProfileRecordsComponent {
    pub score: i32,

    #[serde(rename = "activeScore")]
    pub active_score: i32,
    #[serde(rename = "legacyScore")]
    pub legacy_score: i32,
    #[serde(rename = "lifetimeScore")]
    pub lifetime_score: i32,

    pub records: DestinyRecordComponentMap,
}

/// Contains component data as well as profile privacy and if it is disabled (if available)
///
/// **Source**: [Bungie Documentation](https://bungie-net.github.io/#/components/schemas/SingleComponentResponseOfDestinyProfileComponent)
#[ExternalAPIResponse]
pub struct DestinyComponent<T> {
    pub disabled: Option<bool>,
    pub data: Option<T>,
    #[serde(default)]
    pub privacy: i32,
}

/// The response for GetDestinyProfile, with components for character and item-level data.
///
/// **Note**: This does not contain all available properties of a full Destiny Profile response. Only what we need
///
/// **Source**: [Bungie Official Documentation](https://bungie-net.github.io/#/components/schemas/Destiny.Responses.DestinyProfileResponse)
#[ExternalAPIResponse]
pub struct DestinyProfileResponse {
    /// Records the timestamp of when most components were last generated from the world server source. Unless the component type is specified in the documentation for secondaryComponentsMintedTimestamp, this value is sufficient to do data freshness.
    #[serde(rename = "responseMintedTimestamp", default)]
    pub timestamp_response: chrono::DateTime<chrono::Utc>,

    ///Some secondary components are not tracked in the primary response timestamp and have their timestamp tracked here. If your component is any of the following, this field is where you will find your timestamp value:
    ///
    /// PresentationNodes, Records, Collectibles, Metrics, StringVariables, Craftables, Transitory
    ///
    /// All other component types may use the primary timestamp property.
    #[serde(rename = "secondaryComponentsMintedTimestamp")]
    pub timestamp_secondary_components: chrono::DateTime<chrono::Utc>,

    /// This field is present when the "Profile" component is added into the request
    pub profile: Option<DestinyComponent<DestinyProfileComponent>>,

    /// This field is present when the "Character" component is added into the request
    pub characters: Option<DestinyComponent<DestinyCharacterComponentDictionary>>,

    #[serde(rename = "profileRecords")]
    pub records: Option<DestinyComponent<DestinyProfileRecordsComponent>>,
}

/// Information about the activity
///
/// **Source** [Bungie Documentation](https://bungie-net.github.io/#/components/schemas/Destiny.HistoricalStats.DestinyHistoricalStatsActivity)
#[ExternalAPIResponse]
pub struct DestinyHistoricalStatsActivity {
    /// The unique hash identifier of the DestinyActivityDefinition that was played. If I had this to do over, it'd be named activityHash. Too late now.
    ///
    /// **Note** Is the same as directorActivityHash
    #[serde(rename = "referenceId")]
    pub reference_id: u32,

    /// The unique hash identifier of the DestinyActivityDefinition that was played.
    #[serde(rename = "directorActivityHash")]
    pub director_activity_hash: u32,

    ///The unique identifier for this *specific* match that was played.
    ///
    /// This value can be used to get additional data about this activity such as who else was playing via the GetPostGameCarnageReport endpoint.
    #[serde(rename = "instanceId")]
    pub instance_id: String,

    #[serde(rename = "isPrivate")]
    pub is_private: bool,

    pub mode: DestinyActivityModeType,
    pub modes: Vec<DestinyActivityModeType>,

    #[serde(rename = "membershipType", default)]
    pub membership_type: i32,
}

#[ExternalAPIResponse]
pub struct DestinyHistoricalStatsValuePair {
    pub value: f64,

    #[serde(rename = "displayValue")]
    pub display_value: String,
}

#[ExternalAPIResponse]
pub struct DestinyHistoricalStatsValue {
    #[serde(rename = "statId", default)]
    pub stat_id: String,

    /// from bungie doc: When a stat represents the best, most, longest, fastest or some other personal best, the actual activity ID where that personal best was established is available on this property.
    #[serde(rename = "activityId")]
    pub activity_id: Option<i64>,

    pub basic: DestinyHistoricalStatsValuePair,
    pub pga: Option<DestinyHistoricalStatsValuePair>,
    pub weighted: Option<DestinyHistoricalStatsValuePair>,
}

#[ExternalAPIResponse]
pub struct DestinyHistoricalStatsPeriodGroup {
    pub period: chrono::DateTime<chrono::Utc>,
    pub values: HashMap<String, DestinyHistoricalStatsValue>,

    #[serde(rename = "activityDetails")]
    pub details: DestinyHistoricalStatsActivity,
}

/// Activity history tied to a character
///
/// **Source** [Bungie Documentation](https://bungie-net.github.io/#Destiny2.GetActivityHistory)
#[ExternalAPIResponse]
pub struct DestinyActivityHistoryResults {
    #[serde(default)]
    pub activities: Vec<DestinyHistoricalStatsPeriodGroup>,
}

#[ExternalAPIResponse]
pub struct ClanBanner {
    #[serde(rename = "decalId")]
    pub decal_id: u32,

    #[serde(rename = "decalColorId")]
    pub decal_color_id: u32,

    #[serde(rename = "decalBackgroundColorId")]
    pub decal_background_color_id: u32,

    #[serde(rename = "gonfalonId")]
    pub gonfalon_id: u32,

    #[serde(rename = "gonfalonColorId")]
    pub gonfalon_color_id: u32,

    #[serde(rename = "gonfalonDetailId")]
    pub gonfalon_detail_id: u32,

    #[serde(rename = "gonfalonDetailColorId")]
    pub gonfalon_detail_color_id: u32,
}

#[ExternalAPIResponse]
pub struct GroupV2ClanInfoAndInvestment {
    #[serde(rename = "clanCallsign")]
    pub clan_callsign: String,

    #[serde(rename = "clanBannerData")]
    pub clan_banner: ClanBanner,
}

#[ExternalAPIResponse]
pub struct GroupV2 {
    #[serde(rename = "groupId")]
    pub group_id: String,

    pub name: String,

    #[serde(rename = "groupType")]
    pub group_type: i32,

    pub about: String,

    #[serde(rename = "memberCount")]
    pub member_count: i32,
    pub motto: String,
    pub theme: String,

    #[serde(rename = "bannerPath")]
    pub banner_path: String,

    #[serde(rename = "avatarPath")]
    pub avatar_path: String,

    #[serde(rename = "remoteGroupId")]
    pub remote_group_id: String,

    #[serde(rename = "clanInfo")]
    pub clan_info: GroupV2ClanInfoAndInvestment,
}

#[ExternalAPIResponse]
pub struct DestinyGroupResponse {
    pub detail: GroupV2,
}

#[ExternalAPIResponse]
pub struct GroupMember {
    #[serde(rename = "memberType")]
    pub member_type: i32,

    #[serde(rename = "isOnline")]
    pub is_online: bool,

    #[serde(rename = "groupId")]
    pub group_id: String,

    #[serde(rename = "destinyUserInfo")]
    pub user_info: UserInfoCard,

    #[serde(rename = "bungieNetUserInfo", default)]
    pub bungie_info: UserInfoCard,

    #[serde(rename = "joinDate")]
    pub join_date: chrono::DateTime<chrono::Utc>,
}

#[ExternalAPIResponse]
pub struct PagedQuery {
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: i32,

    #[serde(rename = "currentPage")]
    pub current_page: i32,

    #[serde(rename = "requestContinuationToken", default)]
    pub request_token: String,
}

#[ExternalAPIResponse]
pub struct DestinySearchResultOfGroupMember {
    pub results: Vec<GroupMember>,

    #[serde(rename = "totalResults")]
    pub total_results: i32,

    #[serde(rename = "hasMore")]
    pub has_more: bool,

    #[serde(default)]
    pub query: PagedQuery,

    #[serde(rename = "replacementContinuationToken", default)]
    pub replacement_token: String,
}

/// only contains a small subset of what we need
/// https://bungie-net.github.io/#/components/schemas/Destiny.HistoricalStats.DestinyHistoricalStatsByPeriod
#[ExternalAPIResponse]
pub struct DestinyHistoricalStatsByPeriod {
    #[serde(rename = "allTime")]
    pub all_time: HashMap<String, DestinyHistoricalStatsValue>,
}

#[ExternalAPIResponse]
pub struct DestinyHistoricalStatsWithMerged {
    /// this contains both pve and pvp stats, which is all we are concerned about for our needs
    pub merged: DestinyHistoricalStatsByPeriod,
}

/// we only use a subset of properties for our representation of the schema
/// https://bungie-net.github.io/#/components/schemas/Destiny.HistoricalStats.DestinyHistoricalStatsAccountResult
#[ExternalAPIResponse]
pub struct DestinyHistoricalStatsAccountResult {
    #[serde(rename = "mergedAllCharacters")]
    pub all_characters: DestinyHistoricalStatsWithMerged,
}

#[ExternalAPIResponse]
pub struct GroupMembership {
    #[serde(default)]
    pub member: GroupMember,

    #[serde(default)]
    pub group: GroupV2,
}

/// https://bungie-net.github.io/#/components/schemas/GroupsV2.GetGroupsForMemberResponse
#[ExternalAPIResponse]
pub struct GetGroupsForMemberResponse {
    #[serde(rename = "areAllMembershipsInactive")]
    pub all_inactive_members: HashMap<String, bool>,
    pub results: Vec<GroupMembership>,
}

#[ExternalAPIResponse]
pub struct DestinyHistoricalPlayer {
    #[serde(rename = "destinyUserInfo")]
    pub user_info: UserInfoCard,

    #[serde(rename = "characterClass", default)]
    pub character_class: String,

    #[serde(rename = "classHash")]
    pub class_hash: u32,

    #[serde(rename = "characterLevel")]
    pub character_level: i32,

    #[serde(rename = "lightLevel")]
    pub light_level: i32,

    #[serde(rename = "bungieNetUserInfo", default)]
    pub bungie_info: UserInfoCard,

    #[serde(rename = "clanName", default)]
    pub clan_name: String,

    #[serde(rename = "clanTag", default)]
    pub clan_tag: String,

    #[serde(rename = "emblemHash", default)]
    pub emblem_hash: u32,
}

/// https://bungie-net.github.io/#/components/schemas/Destiny.HistoricalStats.DestinyPostGameCarnageReportEntry
#[ExternalAPIResponse]
pub struct DestinyPostGameCarnageReportEntry {
    pub standing: i32,
    #[serde(rename = "characterId", default)]
    pub character_id: String,
    pub score: DestinyHistoricalStatsValue,
    pub values: HashMap<String, DestinyHistoricalStatsValue>,
    pub player: DestinyHistoricalPlayer,
}

#[ExternalAPIResponse]
pub struct DestinyPostGameCarnageReportTeamEntry {
    #[serde(rename = "teamId")]
    pub team_id: i32,

    pub standing: DestinyHistoricalStatsValue,
    pub score: DestinyHistoricalStatsValue,

    #[serde(rename = "teamName")]
    pub team_name: String,
}

#[ExternalAPIResponse]
pub struct DestinyPostGameCarnageReportData {
    pub period: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "startingPhaseIndex")]
    pub starting_phase_index: Option<i32>,

    #[serde(rename = "activityWasStartedFromBeginning")]
    pub started_from_beginning: Option<bool>,

    #[serde(rename = "activityDetails")]
    pub details: DestinyHistoricalStatsActivity,

    #[serde(default)]
    pub entries: Vec<DestinyPostGameCarnageReportEntry>,

    pub teams: Vec<DestinyPostGameCarnageReportTeamEntry>,
}

#[ExternalAPIResponse]
pub struct UserMembershipData {
    #[serde(rename = "destinyMemberships")]
    pub memberships: Vec<UserInfoCard>,
}
