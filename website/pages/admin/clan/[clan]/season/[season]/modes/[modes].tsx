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

export const getServerSideProps: GetServerSideProps<
  ReportPageSeasonProps
> = async (context) => {
  // fetch our network roster, seasons, destiny game mode groupings
  const [clans, roster, seasons, modes] = await Promise.all([
    getNetworkClans(ENV.hosts.destiny),
    getNetworkRoster(ENV.hosts.destiny),
    getDestinySeasons(ENV.hosts.destiny),
    getDestinyModeGroups(ENV.hosts.destiny, 'dashboard'),
  ]);

  clans.unshift({
    group_id: '0',
    name: 'Network',
    slug: 'network',
    is_network: true,
    call_sign: '[Network]',
  });

  return {
    props: {
      seasons: seasons,
      clan: (context.query.clan as string) || 'network',
      target_season: (context.query.season as string) || 'lifetime',
      target_mode: (context.query.modes as string) || 'all',
      modes: modes,
      clans,
      roster,
    },
  };
};

function generate_url(clan_slug: string, season: string, mode: string) {
  return (
    '/admin/clan/' +
    encodeURIComponent(clan_slug) +
    (season === 'lifetime'
      ? '/lifetime'
      : '/season/' + encodeURIComponent(season)) +
    '/modes/' +
    encodeURIComponent(mode)
  );
}

export const ClanReportPage = (props: ReportPageSeasonProps) => {
  const [targetClan, setClan] = useState(props.clan);
  const [targetSnapshot, setSnapshot] = useState(props.target_season);
  const [targetMode, setMode] = useState(props.target_mode);
  const router = useRouter();

  useEffect(() => {
    const url = generate_url(targetClan, targetSnapshot, targetMode);
    router.push(url);
  }, [targetClan, targetSnapshot, targetMode]);

  return (
    <OffCanvas>
      <Head>
        <title>
          {props.clan +
            ' ' +
            (props.target_season === 'lifetime'
              ? 'Lifetime'
              : 'Season ' + props.target_season) +
            ' Report | Level Crush'}
        </title>
      </Head>
      <SiteHeader forceStickyStyle={true} />
      <main>
        <Container className="top-[4.5rem] relative">
          <LoginGuard admin={true}>
            <Grid numItemsLg={12} className="gap-6">
              <Col numColSpan={3}>
                <Title>Clan</Title>
                <SearchSelect
                  className="mt-2"
                  defaultValue={props.clan}
                  onValueChange={(v) => setClan(v)}
                >
                  {props.clans.map((clan, clanIndex) => (
                    <SearchSelectItem
                      key={'clan_report_roster_select_item_' + clanIndex}
                      value={clan.slug || ''}
                    >
                      {clan.name}
                    </SearchSelectItem>
                  ))}
                </SearchSelect>
              </Col>
              <Col numColSpanLg={2} numColSpanMd={6}>
                <Title>Snapshot</Title>
                <Select
                  defaultValue={targetSnapshot}
                  className="mt-2"
                  onValueChange={(v) => setSnapshot(v)}
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
                  defaultValue={targetMode}
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
            <DestinyClanReportComponent
              clan={props.clan}
              roster={props.roster}
              season={
                targetSnapshot === 'lifetime'
                  ? 'lifetime'
                  : parseInt(targetSnapshot)
              }
              modes={targetMode === 'all' ? [] : targetMode.split(',')}
            />
          </LoginGuard>
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ClanReportPage;
