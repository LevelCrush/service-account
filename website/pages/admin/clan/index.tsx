import Head from 'next/head';
import { SiteHeader } from '@website/components/site_header';
import Container from '@website/components/elements/container';
import OffCanvas from '@website/components/offcanvas';
import LoginGuard from '@website/components/login_guard';
import {
  Title,
  Grid,
  Col,
  Divider,
  SearchSelectItem,
  SearchSelect,
  SelectItem,
  Select,
} from '@tremor/react';
import DestinyMemberReportComponent from '@website/components/destiny_member_report';
import { useRouter } from 'next/router';
import { GetServerSideProps } from 'next';
import { useCallback, useEffect, useLayoutEffect, useState } from 'react';
import { DestinyMemberInformation } from '@website/core/api_responses';
import ENV from '@website/core/env';
import {
  ClanInformation,
  DestinyActivityModeGroup,
  getDestinyModeGroups,
  getDestinySeasons,
  getNetworkClans,
  getNetworkRoster,
} from '@levelcrush/service-destiny';
import DestinyClanReportComponent from '@website/components/destiny_clan_report';

export interface ReportPageSeasonProps {
  clan: string;
  seasons: number[];
  target_season: string;
  target_mode: string;
  modes: DestinyActivityModeGroup[];
  clans: ClanInformation[];
  roster: DestinyMemberInformation[];
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
