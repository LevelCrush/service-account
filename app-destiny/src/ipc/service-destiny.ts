import { invoke } from '@tauri-apps/api';
import {
  APIResponse,
  ClanInformation,
  ClanResponse,
  DestinySeason,
  MemberResponse,
  NetworkActivityClanBreakdown,
  ReportOutput,
  SettingModeRecord,
} from '@ipc/bindings';

export type DestinyModeTypeSearch = 'all' | 'leaderboards' | 'dashboard';

/**
 * Intended to represent combinations of the the enumeration "DestinyActivityModeType".
 *
 * [Bungie Documentation](https://bungie-net.github.io/#/components/schemas/Destiny.HistoricalStats.Definitions.DestinyActivityModeType)
 */
export interface DestinyActivityModeGroup {
  name: string;
  value: string;
}

/**
 * Query the database and get the active destiny seasons
 * @returns
 */
export const getDestinySeasons = async () => {
  const season_data = (await invoke('settings_active_seasons')) as APIResponse<
    DestinySeason[]
  >;

  let seasons = [] as number[];
  seasons = seasons.concat(
    [0],
    season_data.response !== null
      ? season_data.response.map((v) => v.number)
      : []
  );

  return seasons;
};

/**
 * Query the database and fetch the premade destiny mode groups
 * @param mode_type
 * @returns
 */
export const getDestinyModeGroups = async (
  mode_type: DestinyModeTypeSearch
) => {
  const target_type = mode_type ? mode_type : 'all';

  // translate into the tauri command we need to invoke
  let invoke_cmd = 'settings_modes';
  switch (target_type) {
    case 'dashboard':
      invoke_cmd = 'settings_dashboard_modes';
      break;
    case 'leaderboards':
      invoke_cmd = 'settings_leaderboard_modes';
      break;
    case 'all':
    default:
      invoke_cmd = 'settings_modes';
      break;
  }

  const data = (await invoke(invoke_cmd)) as APIResponse<SettingModeRecord[]>;
  const settings = data.response !== null ? data.response : [];
  const modes = settings.map((val) => {
    return {
      name: val.name,
      value: val.value,
    } as DestinyActivityModeGroup;
  });

  return modes;
};

/**
 * Query the database and get our network roster
 * @returns
 */
export const getNetworkRoster = async () => {
  const network_roster = (await invoke('network_roster')) as APIResponse<
    MemberResponse[]
  >;

  return network_roster && network_roster.response !== null
    ? network_roster.response
    : [];
};

/**
 * Queries the service for networked clans
 * @returns
 */
export const getNetworkClans = async () => {
  const network_clans = (await invoke('network_clans')) as APIResponse<
    ClanInformation[]
  >;
  return network_clans.response !== null ? network_clans.response : [];
};

/**
 * Query the database for member information
 * @param bungie_name
 * @returns
 */
export const getMemberInfo = async (bungie_name: string) => {
  const data = (await invoke('member_info', {
    bungieName: bungie_name,
  })) as APIResponse<MemberResponse>;
  return data;
};

/**
 * Fetches a report for a member
 * @param bungie_name
 * @param season
 * @param modes
 * @returns
 */
export const getMemberReport = async (
  bungie_name: string,
  season: string,
  modes: string
) => {
  console.log('Asking for member report', bungie_name, season, modes);

  const invoke_cmd =
    season === 'lifetime' ? 'member_lifetime_report' : 'member_season_report';

  const data = (await invoke(invoke_cmd, {
    bungieName: bungie_name,
    season: season,
    reportQueries: {
      modes: modes ? modes : undefined,
    },
  })) as APIResponse<ReportOutput>;

  console.log('Data returned from member report', bungie_name, data);

  return data;
};

/**
 * Get network report
 * @param season
 * @param modes
 */
export const getNetworkReport = async (season: string, modes: string) => {
  const invoke_cmd =
    season === 'lifetime' ? 'network_lifetime_report' : 'network_season_report';

  const data = await invoke(invoke_cmd, {
    season: season,
    reportQueries: {
      modes: modes,
    },
  });
  return data as APIResponse<{ [key: string]: ReportOutput }>;
};

/**
 * Gets the network breakdown
 * @param season
 * @param modes
 * @returns
 */
export const getNetworkBreakdown = async (season: string, modes: string) => {
  const invoke_cmd =
    season === 'lifetime'
      ? 'network_breakdown_lifetime'
      : 'network_breakdown_season';

  const data = await invoke(invoke_cmd, {
    season: season,
    reportQueries: {
      modes: modes,
    },
  });
  return data as APIResponse<{ [key: string]: NetworkActivityClanBreakdown }>;
};

export const getClanInfo = async (group_id: string) => {
  console.log('Invoking clan info', group_id);
  const data = (await invoke('clan_info', {
    groupId: group_id,
  })) as APIResponse<ClanInformation>;
  console.log('Done getting data', data);
  return data;
};

export const getClanRoster = async (group_id: string) => {
  console.log('Invoking clan roster', group_id);
  const data = (await invoke('clan_roster', {
    groupId: group_id,
  })) as APIResponse<ClanResponse>;
  console.log('Done getting clan roster', data);
  return data;
};
