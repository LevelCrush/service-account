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
import { useEffect, useLayoutEffect, useState } from 'react';
import { DestinyMemberInformation } from '@website/core/api_responses';
import ENV from '@website/core/env';
import {
  DestinyActivityModeGroup,
  getDestinyModeGroups,
  getDestinySeasons,
  getNetworkRoster,
} from '@levelcrush/service-destiny';

export interface ReportPageSeasonProps {
  member: string;
  seasons: number[];
  target_season: string;
  target_mode: string;
  modes: DestinyActivityModeGroup[];
  roster: DestinyMemberInformation[];
}

//export const revalidate = 600;

export const getServerSideProps: GetServerSideProps<
  ReportPageSeasonProps
> = async (context) => {
  // fetch our network roster, seasons, destiny game mode groupings
  const [roster, seasons, modes] = await Promise.all([
    getNetworkRoster(ENV.hosts.destiny),
    getDestinySeasons(ENV.hosts.destiny),
    getDestinyModeGroups(ENV.hosts.destiny, 'dashboard'),
  ]);

  return {
    props: {
      seasons: seasons,
      member: context.query.member as string,
      target_season: context.query.season as string,
      target_mode: context.query.modes as string,
      modes: modes,
      roster,
    },
  };
};

function generate_url(bungie_name: string, season: string, mode: string) {
  return (
    '/admin/report/' +
    encodeURIComponent(bungie_name) +
    (season === 'lifetime'
      ? '/lifetime'
      : '/season/' + encodeURIComponent(season)) +
    '/modes/' +
    encodeURIComponent(mode)
  );
}

export const ReportPage = (props: ReportPageSeasonProps) => {
  const [targetUser, setUser] = useState(props.member);
  const [targetSnapshot, setSnapshot] = useState(props.target_season);
  const [controlsDisabled, setControlsDisabled] = useState(true);
  const [overviewUrl, setOverviewUrl] = useState(
    generate_url(props.member, props.target_season, props.target_mode)
  );

  const [targetMode, setMode] = useState(props.target_mode);
  const router = useRouter();

  useEffect(() => {
    setControlsDisabled(true);
    setOverviewUrl(generate_url(targetUser, targetSnapshot, targetMode));
  }, [targetMode, targetSnapshot, targetUser]);

  useLayoutEffect(() => {
    console.log('New url requested');
    router.push(overviewUrl);
  }, [overviewUrl]);

  return (
    <OffCanvas>
      <Head>
        <title>
          {props.member +
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
                <Title>Member</Title>
                <SearchSelect
                  className="mt-2"
                  defaultValue={props.member}
                  onValueChange={(v) => setUser(v)}
                  disabled={controlsDisabled}
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
                  defaultValue={props.target_season}
                  className="mt-2"
                  onValueChange={(v) => setSnapshot(v)}
                  disabled={controlsDisabled}
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
                  defaultValue={props.target_mode}
                  onValueChange={(v) => setMode(v)}
                  disabled={controlsDisabled}
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
              season={
                props.target_season === 'lifetime'
                  ? 'lifetime'
                  : parseInt(props.target_season)
              }
              modes={targetMode === 'all' ? [] : targetMode.split(',')}
              onReportLoaded={(data) => setControlsDisabled(false)}
              onLoadingData={() => {
                console.log('Loading data!');
                setControlsDisabled(true);
              }}
            />
          </LoginGuard>
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ReportPage;
