import { DestinyNetworkRosterResponse } from '@core/api_responses';
import ENV from '@core/env';

/**
 * Calls the the level crush dest iny service and fetches the network roster
 * @returns
 */
export async function getNetworkRoster() {
  const destiny_api = ENV.hosts.destiny;
  const response = await fetch(destiny_api + '/network/roster');

  const network_roster = response.ok
    ? ((await response.json()) as DestinyNetworkRosterResponse)
    : null;

  return network_roster && network_roster.response !== null
    ? network_roster.response
    : [];
}

export default getNetworkRoster;
