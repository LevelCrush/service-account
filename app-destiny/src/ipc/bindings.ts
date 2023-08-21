export interface APIResponseError {
  field: string;
  message: string;
}

export interface PaginationData {
  total_results: number;
  total_pages: number;
  page: number;
  limit: number;
  showing: number;
  term: string;
}

export interface PaginationQuery {
  page: number;
  limit: number;
}

export interface PaginationResponse<T> {
  data: T[];
  pagination: PaginationData;
}

export interface APIResponse<T> {
  success: boolean;
  response: T | null;
  errors: Array<APIResponseError>;
  requested_at: bigint;
  completed_at: bigint;
}

export interface ClanInformation {
  group_id: string;
  name: string;
  call_sign: string;
  is_network: boolean;
  member_count?: number;
  slug?: string;
  motto?: string;
  about?: string;
}

export interface ClanResponse {
  group_id: string;
  name: string;
  call_sign: string;
  is_network: boolean;
  member_count?: number;
  slug?: string;
  motto?: string;
  about?: string;
  roster: Array<MemberResponse>;
}

export interface MemberClanInformation {
  group_id: string;
  name: string;
  call_sign: string;
  is_network: boolean;
  member_count?: number;
  slug?: string;
  motto?: string;
  about?: string;
  timestamp_join_date: number;
  role: number;
}

export interface MemberResponse {
  display_name: string;
  display_name_platform: string;
  membership_id: string;
  membership_platform: number;
  timestamp_last_played: number;
  raid_report: string;
  clan?: MemberClanInformation;
}

export interface MemberReport {
  version: bigint;
  membership_id: string;
  snapshot_range: string;
  display_name_global: string;
  last_played_at: number;
  activity_timestamps: Record<number, number>;
  activity_attempts: number;
  activity_attempts_with_clan: number;
  activity_completions: number;
  stats_pve: MemberReportStats;
  stats_pvp: MemberReportStats;
  stats_gambit: MemberReportStats;
  stats_private_matches: MemberReportStats;
  stats_reckoning: MemberReportStats;
  top_activity_modes: Array<MemberReportActivityMode>;
  top_activities: Array<MemberReportActivity>;
  activity_map: Record<string, MemberReportActivity>;
  frequent_clan_members: Array<MemberReportFireteamMember>;
  frequent_non_clan_members: Array<MemberReportFireteamMember>;
  total_clan_members: number;
  total_non_clan_members: number;
  titles: Array<MemberTitle>;
  member: MemberResponse;
  search: MemberReportSearchQuery;
}

export interface MemberReportActivity {
  attempts: number;
  completions: number;
  name: string;
  description: string;
}

export interface MemberReportActivityMode {
  mode: string;
  count: number;
}

export interface MemberReportFireteamMember {
  display_name: string;
  activities: number;
}

export interface MemberReportSearchQuery {
  member: string;
  modes: string;
  season: string;
}

export interface MemberReportStats {
  kills: number;
  deaths: number;
  assists: number;
  victories: number;
  defeats: number;
  activities: number;
  activity_completions: number;
  activities_completed_start_to_finish: number;
}

export interface MemberTitle {
  title: string;
  gilded_past: boolean;
  gilded_amount: number;
  gilded_season: boolean;
}

export interface MemberTitleResponse {
  display_name: string;
  display_name_platform: string;
  membership_id: string;
  membership_platform: bigint;
  timestamp_last_played: number;
  raid_report: string;
  clan?: MemberClanInformation;
  titles: Array<MemberTitle>;
}

export interface NetworkActivityClanBreakdown {
  group_id: string;
  name: string;
  total_members: number;
  activity_attempts: number;
  activities_completed_with_clan: number;
  activities_completed: number;
  percent_with_clan: number;
  avg_clan_member_amount: number;
}

export type ReportOutput = bigint | MemberReport;

export interface SettingModeRecord {
  id: bigint;
  leaderboard: bigint;
  dashboard: bigint;
  name: string;
  value: string;
  description: string;
  order: bigint;
  created_at: bigint;
  updated_at: bigint;
  deleted_at: bigint;
}

export interface LeaderboardEntry {
  display_name: string;
  amount: number;
  standing: number;
  percent_ranking: number;
}

export interface Leaderboard {
  name: string;
  entries: Array<LeaderboardEntry>;
  description: string;
}

export interface DestinySeason {
  name: string;
  number: number;
  starts_at: number;
  ends_at: number;
}
