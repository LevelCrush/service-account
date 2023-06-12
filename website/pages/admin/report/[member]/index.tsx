import Head from 'next/head';
import { SiteHeader } from '../../../../components/site_header';
import Container from '../../../../components/elements/container';
import OffCanvas from '../../../../components/offcanvas';
import LoginGuard from '../../../../components/login_guard';

import {
  Text,
  Title,
  Grid,
  Col,
  Divider,
  SearchSelectItem,
  SearchSelect,
  SelectItem,
  Select,
} from '@tremor/react';

import DestinyMemberReportComponent from '../../../../components/destiny_member_report';
import { useRouter } from 'next/router';
import { GetServerSideProps } from 'next';
import { useState } from 'react';
import {
  DestinyMemberInformation,
  DestinyNetworkRosterResponse,
} from '../../../../core/api_responses';
import ENV from '../../../../core/env';

export interface ReportPageProps {
  member: string;
  seasons: number[];
  modes: { value: string; name: string }[];
  roster: DestinyMemberInformation[];
}

export const getServerSideProps: GetServerSideProps<ReportPageProps> = async (
  context
) => {
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
  ] as ReportPageProps['modes'];

  return {
    props: {
      seasons: seasons,
      member: context.query.member as string,
      modes: modes,
      roster: roster,
    },
  };
};

export const ReportPage = (props: ReportPageProps) => {
  const [targetSeason, setSeason] = useState('lifetime' as 'lifetime' | number);
  const [targetMode, setMode] = useState('');

  return (
    <OffCanvas>
      <Head>
        <title>Report | Level Crush</title>
      </Head>
      <SiteHeader forceStickyStyle={true} />
      <main>
        <Container className="top-[4.5rem] relative">
          <LoginGuard admin={true}>
            <Grid numItemsLg={12} className="gap-6">
              <Col numColSpan={3}>
                <Title>Member</Title>
                <SearchSelect
                  className="mt-2"
                  defaultValue={props.member}
                  onValueChange={(v) =>
                    (window.location.href =
                      '/admin/report/' + encodeURIComponent(v))
                  }
                >
                  {props.roster.map((member, memberIndex) => (
                    <SearchSelectItem
                      key={'member_report_roster_select_item_' + memberIndex}
                      value={member.display_name}
                    >
                      {member.display_name}
                    </SearchSelectItem>
                  ))}
                </SearchSelect>
              </Col>
              <Col numColSpanLg={2} numColSpanMd={6}>
                <Title>Snapshot</Title>
                <Select
                  defaultValue="lifetime"
                  className="mt-2"
                  onValueChange={(v) =>
                    setSeason(v === 'lifetime' ? 'lifetime' : parseInt(v))
                  }
                >
                  {props.seasons.map((season) => {
                    const v = season === 0 ? 'lifetime' : season + '';
                    const text = season === 0 ? 'Lifetime' : 'Season ' + season;
                    return (
                      <SelectItem
                        value={v}
                        key={'search_select_overview_season_x' + season}
                      >
                        {text}
                      </SelectItem>
                    );
                  })}
                </Select>
              </Col>
              <Col numColSpanLg={2} numColSpanMd={6}>
                <Title>Modes</Title>
                <Select
                  className="mt-2"
                  defaultValue="all"
                  onValueChange={(v) => setMode(v)}
                >
                  {props.modes.map((mode, modeIndex) => (
                    <SelectItem
                      key={'member_report_mode_select_item_' + modeIndex}
                      value={mode.value}
                    >
                      {mode.name}
                    </SelectItem>
                  ))}
                </Select>
              </Col>
            </Grid>
            <Divider />
            <DestinyMemberReportComponent
              bungie_name={props.member}
              season={targetSeason}
              modes={targetMode === 'all' ? [] : targetMode.split(',')}
            />
          </LoginGuard>
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ReportPage;
