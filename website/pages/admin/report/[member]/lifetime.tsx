import { GetServerSideProps } from 'next';
import ENV from '@website/core/env';
import {
  ReportPage,
  ReportPageSeasonProps,
} from '@website/pages/admin/report/[member]/season/[season]';
import {
  getDestinyModeGroups,
  getDestinySeasons,
  getNetworkRoster,
} from '@levelcrush/service-destiny';

export const getServerSideProps: GetServerSideProps<
  ReportPageSeasonProps
> = async (context) => {
  // fetch our network roster, seasons, destiny game mode groupings
  const [roster, seasons, modes] = await Promise.all([
    getNetworkRoster(ENV.hosts.destiny),
    getDestinySeasons(ENV.hosts.destiny),
    getDestinyModeGroups(ENV.hosts.destiny),
  ]);

  return {
    props: {
      seasons: seasons,
      member: context.query.member as string,
      target_season: 'lifetime',
      modes: modes,
      roster: roster,
    },
  };
};

export default ReportPage;
