use crate::app::report::member::MemberReport;
use crate::database::clan::ClanInfoResult;
use crate::database::leaderboard::LeaderboardEntryResult;
use crate::database::triumph::TriumphTitleResult;
use crate::{app, database::member::MemberResult};
use levelcrush::bigdecimal::ToPrimitive;
use levelcrush::server::{APIResponse, PaginationResponse};
use levelcrush::types::destiny::GroupId;
use levelcrush::types::{destiny::MembershipId, destiny::MembershipType, UnixTimestamp};
use ts_rs::TS;

// clan responses
#[derive(serde::Serialize, TS, Default, Debug, Clone)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct ClanInformation {
    pub group_id: GroupId,
    pub name: String,
    pub call_sign: String,
    pub is_network: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub motto: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
}

impl ClanInformation {
    pub fn from_db(record: ClanInfoResult) -> ClanInformation {
        ClanInformation {
            group_id: record.group_id,
            name: record.name,
            call_sign: record.call_sign,
            is_network: record.is_network == 1,
            member_count: Some(record.member_count as u32),
            slug: Some(record.slug),
            motto: Some(record.motto),
            about: Some(record.about),
        }
    }
}

#[derive(serde::Serialize, TS, Clone, Default, Debug)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberClanInformation {
    #[serde(flatten)]
    pub info: ClanInformation,

    #[ts(type = "number")]
    pub timestamp_join_date: u64,
    pub role: i8,
}

#[derive(serde::Serialize, TS, Clone, Debug, Default)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberResponse {
    /// bungie global display name
    pub display_name: String,

    /// display name tied to the platform that originated from the accoun
    pub display_name_platform: String,

    /// this is the membership id that is primarily used by the bungie account
    /// because of cross save, this is not the "true" bungie account id
    #[ts(type = "number")]
    pub membership_id: MembershipId,

    /// the platform type of the membership. Included because other calls to the bungie api require it
    pub membership_platform: MembershipType,

    /// the timestamp of the last time this user played
    #[ts(type = "number")]
    pub timestamp_last_played: UnixTimestamp,

    /// link to the users raid report
    pub raid_report: String,

    /// minimal information about the members clan/status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clan: Option<MemberClanInformation>,
}

impl MemberResponse {
    pub fn from_db(result: MemberResult) -> MemberResponse {
        let clan = if result.clan_group_id > 0 {
            Some(MemberClanInformation {
                info: ClanInformation {
                    group_id: result.clan_group_id,
                    name: result.clan_name,
                    call_sign: result.clan_call_sign,
                    is_network: result.clan_is_network == 1,
                    motto: None,
                    about: None,
                    slug: None,
                    member_count: None,
                },
                timestamp_join_date: result.clan_joined_at.to_u64().unwrap_or_default(),
                role: result.clan_group_role,
            })
        } else {
            None
        };

        MemberResponse {
            display_name: result.display_name_global,
            display_name_platform: result.display_name,
            membership_id: result.membership_id,
            membership_platform: result.platform,
            raid_report: app::member::generate_raid_report_url(result.membership_id, result.platform),
            clan,
            timestamp_last_played: result.last_played_at,
        }
    }
}

#[derive(serde::Serialize, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct ClanResponse {
    #[serde(flatten)]
    pub data: ClanInformation,
    pub roster: Vec<MemberResponse>,
}

#[derive(serde::Serialize, Default, Clone, Debug, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberTitle {
    pub title: String,
    pub gilded_past: bool,
    pub gilded_amount: i8,
    pub gilded_season: bool,
}

impl MemberTitle {
    pub fn from_db(record: TriumphTitleResult) -> MemberTitle {
        MemberTitle {
            title: record.title,
            gilded_past: record.has_gilded == 1,
            gilded_amount: record.total_gilds as i8,
            gilded_season: record.can_equip_gilded == 1,
        }
    }
}

#[derive(serde::Serialize, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberTitleResponse {
    #[serde(flatten)]
    pub member: MemberResponse,
    pub titles: Vec<MemberTitle>,
}

#[derive(serde::Serialize, TS)]
#[serde(untagged)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub enum ReportOutput {
    TaskRunning(UnixTimestamp),
    Report(Box<MemberReport>),
}

#[derive(serde::Serialize, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct LeaderboardEntry {
    pub display_name: String,
    pub amount: i32,
    pub standing: i32,
    pub percent_ranking: f64,
    pub percent_distance: f64,
}

impl LeaderboardEntry {
    pub fn from_db(record: LeaderboardEntryResult) -> LeaderboardEntry {
        LeaderboardEntry {
            display_name: record.display_name,
            amount: record.amount.to_i32().unwrap_or_default(),
            standing: record.standing as i32,
            percent_ranking: record.percent_ranking,
            percent_distance: record.percent_ranking,
        }
    }
}

#[derive(serde::Serialize, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct Leaderboard {
    pub name: String,
    pub entries: Vec<LeaderboardEntry>,
}

// type aliases
pub type APIMemberResponse = APIResponse<MemberResponse>;
pub type APIMemberSearchResponse = APIResponse<PaginationResponse<MemberResponse>>;
pub type APIClanInfoMultiResponse = APIResponse<Vec<ClanInformation>>;
pub type APIClanInfoResponse = APIResponse<ClanInformation>;
pub type APIClanRosterResponse = APIResponse<ClanResponse>;
pub type APINetworkRosterResponse = APIResponse<Vec<MemberResponse>>;
pub type APIMemberTitleResponse = APIResponse<MemberTitleResponse>;
