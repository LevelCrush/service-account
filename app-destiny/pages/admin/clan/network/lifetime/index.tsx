import { GetServerSideProps } from 'next';
import ClanReportPage, {
  ReportPageSeasonProps,
} from '@website/pages/admin/clan/[clan]/season/[season]/modes/[modes]';
import {
  getDestinyModeGroups,
  getDestinySeasons,
  getNetworkClans,
  getNetworkRoster,
} from '@ipc/service-destiny';
import ENV from '@website/core/env';

export const getServerSideProps: GetServerSideProps<
  ReportPageSeasonProps
> = async (context) => {
  // fetch our network roster, seasons, destiny game mode groupings
  const [clans, roster, seasons, modes] = await Promise.all([
    getNetworkClans(),
    getNetworkRoster(),
    getDestinySeasons(),
    getDestinyModeGroups('dashboard'),
  ]);

  return {
    props: {
      seasons: seasons,
      clan: 'network',
      target_season: 'lifetime',
      target_mode: 'all',
      modes: modes,
      clans,
      roster,
    },
  };
};

export default ClanReportPage;
