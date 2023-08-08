import Head from 'next/head';
import { SiteHeader } from '@website/components/site_header';
import Container from '@website/components/elements/container';
import OffCanvas from '@website/components/offcanvas';
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
import { useEffect, useState } from 'react';
import ENV from '@website/core/env';
import {
  DestinyActivityModeGroup,
  getDestinyModeGroups,
  getDestinySeasons,
  getNetworkRoster,
} from '@ipc/service-destiny';
import { MemberResponse } from '@ipc/bindings';

function generate_url(bungie_name: string, season: string, mode: string) {
  return (
    '/admin/member/' +
    encodeURIComponent(bungie_name) +
    (season === 'lifetime'
      ? '/lifetime'
      : '/season/' + encodeURIComponent(season)) +
    '/modes/' +
    encodeURIComponent(mode)
  );
}

export const ReportPage = () => {
  const router = useRouter();
  const [targetUser, setUser] = useState(router.query.member as string);
  const [targetSnapshot, setSnapshot] = useState(
    (router.query.season as string) || 'lifetime'
  );
  const [targetMode, setMode] = useState((router.query.modes as string) || '');

  const [roster, setRoster] = useState([] as MemberResponse[]);
  const [seasons, setSeasons] = useState([] as number[]);
  const [modes, setModes] = useState([] as DestinyActivityModeGroup[]);

  useEffect(() => {
    Promise.all([
      getNetworkRoster(),
      getDestinySeasons(),
      getDestinyModeGroups('dashboard'),
    ]).then(([roster, seasons, modes]) => {
      setRoster(roster);
      setSeasons(seasons);
      setModes(modes);
    });
  }, []);

  /*
  useEffect(() => {
    router.push(generate_url(targetUser, targetSnapshot, targetMode));
  }, [targetMode, targetSnapshot, targetUser]);
  */

  return (
    <OffCanvas>
      <Head>
        <title>
          {targetUser +
            ' ' +
            (targetSnapshot === 'lifetime'
              ? 'Lifetime'
              : 'Season ' + targetSnapshot) +
            ' Report | Level Crush'}
        </title>
      </Head>
      <SiteHeader forceStickyStyle={true} />
      <main>
        <Container className="top-[4.5rem] relative">
          <Grid numItemsLg={12} className="gap-6">
            <Col numColSpan={3}>
              <Title>Member</Title>
              <SearchSelect
                className="mt-2"
                defaultValue={targetUser}
                onValueChange={(v) => setUser(v)}
              >
                {roster.map((member, memberIndex) => (
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
                defaultValue={targetSnapshot}
                className="mt-2"
                onValueChange={(v) => setSnapshot(v)}
              >
                {seasons.map((season) => {
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
                {modes.map((mode, modeIndex) => (
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
            bungie_name={targetUser}
            season={
              targetSnapshot === 'lifetime'
                ? 'lifetime'
                : parseInt(targetSnapshot)
            }
            modes={targetMode === 'all' ? [] : targetMode.split(',')}
          />
        </Container>
      </main>
    </OffCanvas>
  );
};

export default ReportPage;
