import { GetServerSideProps } from 'next';
import { ClanInformation, MemberResponse } from '@ipc/bindings';
import { DestinyActivityModeGroup } from '@ipc/service-destiny';

export interface ReportPageSeasonProps {
  clan: string;
  seasons: number[];
  target_season: string;
  target_mode: string;
  modes: DestinyActivityModeGroup[];
  clans: ClanInformation[];
  roster: MemberResponse[];
}

export const revalidate = 600;

export const getServerSideProps: GetServerSideProps = async (context) => {
  return {
    redirect: {
      destination: '/admin/clan/network/lifetime',
      permanent: false,
    },
    props: {},
  };
};

export const RedirectToReport = () => <></>;

export default RedirectToReport;
