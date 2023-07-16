import { APIResponse, ServiceDestiny, ServiceAccounts } from '@levelcrush';

export interface APIError {
  field: string;
  message: string;
}

export type AccountLinkedPlatformResult =
  ServiceAccounts.AccountLinkedPlatformsResult;

export type AccountLinkedPlatformsResponse =
  APIResponse<AccountLinkedPlatformResult>;

export interface AccountLinkedPlatformResultMap {
  [bungie_name: string]: AccountLinkedPlatformResult | null;
}

export interface AccountLinkedPlatformMultiSearchResponse {
  success: boolean;
  response: AccountLinkedPlatformResultMap | null;
  error: APIError[];
}

export type DestinyClanInformation = ServiceDestiny.ClanInformation;
export type DestinyClanResponse = APIResponse<ServiceDestiny.ClanInformation>;
export type DestinyMemberInformation = ServiceDestiny.MemberResponse;
export type DestinyMemberResponse = APIResponse<ServiceDestiny.MemberResponse>;
export type DestinyNetworkRosterResponse = APIResponse<
  ServiceDestiny.MemberResponse[]
>;

export type DestinyClanRoster = ServiceDestiny.ClanResponse;
export type DestinyClanRosterResponse =
  APIResponse<ServiceDestiny.ClanResponse>;
export type DestinyMemberStats = ServiceDestiny.MemberReportStats;
export type DestinyMemberFireteamMember =
  ServiceDestiny.MemberReportFireteamMember;
export type DestinyMemberTitle = ServiceDestiny.MemberTitle;
export type DestinyMemberReport = ServiceDestiny.MemberReport;
export type DestinyMemberReportResponse =
  APIResponse<ServiceDestiny.ReportOutput>;

/**
 * Expected data from a platform if it is discord
 */
export interface AccountPlatformDiscord {
  display_name: string;
  discord_id: string;
}

/**
 * Expected data from a platform if it is twitch
 */
export interface AccountPlatformTwitch {
  profile_image_url: string;
  twitch_id: string;
  display_name: string;
  login: string;
  offline_image_url: string;
}

/**
 * Expected data from our AccountResponse if our platform is bungie related
 */
export interface AccountPlatformBungie {
  display_name: string;
  unique_name: string;
  memberships: string;
  raid_report: string;
  primary_membership_id: string;
  primary_platform: string;
  [membership_key: string]: string;
}

/**
 * The expected response from our server when querying for profile information
 */
export interface AccountResponse {
  success: boolean;
  response: {
    display_name: string;
    is_admin: boolean;
    platforms: {
      [platform: string]:
        | AccountPlatformDiscord
        | AccountPlatformBungie
        | AccountPlatformTwitch;
    };
    challenge: string;
  };
  errors: unknown[]; // this response will not return a route, so if there are any, it's unknown and we don't know how to handle it
}
