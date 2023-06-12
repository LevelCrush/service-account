import { APIResponse, ServiceDestiny } from 'levelcrush';

export interface APIError {
  field: string;
  message: string;
}

export interface AccountLinkedPlatformResult {
  account_token: string;
  discord: string;
  bungie: string;
  twitch: string;
}

export interface AccountLinkedPlatformsResponse {
  success: boolean;
  response: AccountLinkedPlatformResult | null;
  error: APIError[];
}

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
  APIResponse<ServiceDestiny.MemberReport>;
