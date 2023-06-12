use std::collections::HashMap;

use levelcrush::chrono;
use levelcrush_macros::ExternalAPIResponse;

#[ExternalAPIResponse]
pub struct DestinyDisplayProperties {
    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub icon: String,

    #[serde(rename = "hasIcon")]
    pub has_icon: bool,
}

#[ExternalAPIResponse]
pub struct DestinyMatchmakingProperties {
    #[serde(rename = "isMatchmade")]
    pub is_matchmade: bool,

    #[serde(rename = "minParty")]
    pub min_party: u32,

    #[serde(rename = "maxParty")]
    pub max_party: u32,

    #[serde(rename = "maxPlayers")]
    pub max_players: u32,

    #[serde(rename = "requiresGuardianOath")]
    pub requires_guardian_oath: bool,
}

#[ExternalAPIResponse]
pub struct DestinyActivityDefinition {
    #[serde(default)]
    pub matchmaking: DestinyMatchmakingProperties,

    #[serde(rename = "displayProperties")]
    pub display_properties: DestinyDisplayProperties,

    #[serde(rename = "originalDisplayProperties")]
    pub original_display_properties: DestinyDisplayProperties,

    #[serde(rename = "selectionScreenDisplayProperties", default)]
    pub selection_screen_display_properties: DestinyDisplayProperties,

    #[serde(rename = "activityTypeHash")]
    pub activity_type_hash: u32,

    #[serde(rename = "destinationHash")]
    pub destination_hash: u32,

    #[serde(rename = "placeHash")]
    pub place_hash: u32,

    #[serde(rename = "pgcrImage", default)]
    pub pgcr_image: String,

    #[serde(rename = "isPvP")]
    pub is_pvp: bool,

    #[serde(rename = "isPlaylist")]
    pub is_playlist: bool,

    pub hash: u32,
    pub index: u32,
    pub redacted: bool,
    pub blacklisted: bool,
}

pub type DestinyActivityDefinitionMap = HashMap<String, DestinyActivityDefinition>;

#[ExternalAPIResponse]
pub struct DestinyClassDefinition {
    #[serde(rename = "classType")]
    pub class_type: u8,

    #[serde(rename = "displayProperties")]
    pub display_properties: DestinyDisplayProperties,

    pub hash: u32,
    pub index: u32,
    pub redacted: bool,
    pub blacklisted: bool,
}

pub type DestinyClassDefinitionMap = HashMap<String, DestinyClassDefinition>;

#[ExternalAPIResponse]
pub struct DestinyActivityTypeDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: DestinyDisplayProperties,

    pub hash: u32,
    pub index: u32,
    pub redacted: bool,
    pub blacklisted: bool,
}

pub type DestinyActivityTypeDefinitionMap = HashMap<String, DestinyActivityTypeDefinition>;

#[ExternalAPIResponse]
pub struct DestinySeasonDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: DestinyDisplayProperties,

    #[serde(rename = "seasonNumber")]
    pub season_number: i32,

    #[serde(rename = "seasonPassHash", default)]
    pub season_pass_hash: u32,

    #[serde(rename = "startDate", default)]
    pub start_date: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "endDate", default)]
    pub end_date: chrono::DateTime<chrono::Utc>,

    pub hash: u32,
    pub index: u32,
    pub redacted: bool,
    pub blacklisted: bool,
}

pub type DestinySeasonDefinitionMap = HashMap<String, DestinySeasonDefinition>;

#[ExternalAPIResponse]
pub struct DestinyRecordTitleBlock {
    #[serde(rename = "hasTitle")]
    pub has_title: bool,

    #[serde(rename = "titlesByGender", default)]
    pub titles_by_gender: HashMap<String, String>,
}

/// Subset of fields that we are extracting from a record definition. Full definition found below at the link
///
/// https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.Records.DestinyRecordDefinition
#[ExternalAPIResponse]
pub struct DestinyRecordDefinition {
    #[serde(rename = "displayProperties")]
    pub display_properties: DestinyDisplayProperties,
    pub scope: i32,

    #[serde(rename = "titleInfo", default)]
    pub title_info: DestinyRecordTitleBlock,

    #[serde(rename = "forTitleGilding", default)]
    pub for_title_gilding: bool,

    pub hash: u32,
    pub index: u32,
    pub redacted: bool,
    pub blacklisted: bool,
}

pub type DestinyRecordDefinitionMap = HashMap<String, DestinyRecordDefinition>;

/// Supported Definitions from the destiny api
pub enum DestinyDefinition {
    Activity,
    ActivityType,
    Class,
    Seasons,
    Records,
}

impl From<DestinyDefinition> for &'static str {
    fn from(value: DestinyDefinition) -> Self {
        match value {
            DestinyDefinition::Activity => "DestinyActivityDefinition",
            DestinyDefinition::ActivityType => "DestinyActivityTypeDefinition",
            DestinyDefinition::Class => "DestinyClassDefinition",
            DestinyDefinition::Seasons => "DestinySeasonDefinition",
            DestinyDefinition::Records => "DestinyRecordDefinition",
        }
    }
}
