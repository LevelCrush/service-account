import { GetServerSideProps } from 'next';
import { DestinyNetworkRosterResponse } from '@core/api_responses';
import ENV from '@core/env';
import ReportPage, { ReportPageSeasonProps } from './season/[season]';

export const getServerSideProps: GetServerSideProps<
  ReportPageSeasonProps
> = async (context) => {
  const destiny_api = ENV.hosts.destiny;
  const response = await fetch(destiny_api + '/network/roster');

  const network_roster = response.ok
    ? ((await response.json()) as DestinyNetworkRosterResponse)
    : null;

  const roster =
    network_roster && network_roster.response !== null
      ? network_roster.response
      : [];

  let seasons = [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21];
  seasons.sort((a, b) => b - a);
  seasons = [0].concat(seasons);

  const modes = [
    {
      name: 'All',
      value: 'all',
    },
    {
      name: 'Raid',
      value: '4',
    },
  ] as ReportPageSeasonProps['modes'];

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
