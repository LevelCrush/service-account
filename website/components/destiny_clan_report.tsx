import { useEffect, useRef, useState } from 'react';
import {
  DestinyMemberReport,
  DestinyMemberReportResponse,
  DestinyMemberStats,
} from '@website/core/api_responses';

import { H3, H5 } from '@website/components/elements/headings';
import ENV from '@website/core/env';
import {
  Divider,
  Grid,
  TabPanel,
  Title,
  Card,
  CardProps,
  List,
  ListItem,
  Col,
  DonutChart,
  Text,
  Legend,
  Metric,
  LineChart,
  TabGroup,
  TabList,
  Tab,
  TabPanels,
  BarChart,
} from '@tremor/react';
import Hyperlink from '@website/components/elements/hyperlink';
import { MemberResponse, ReportOutput } from '@levelcrush/service-destiny';

export interface ClanReportProps {
  clan: string;
  roster: MemberResponse[];
  season: 'lifetime' | number;
  modes?: string[];
}

interface MemberListProps extends CardProps {
  members: MemberResponse[];
  report_status: Map<string, boolean>;
  season: string;
  mode: string;
  listType: string;
  badgeRoleColors: { [role: string]: string };
  badgeClanColors: { [clan: string]: string };
}

function generate_member_url(
  bungie_name: string,
  season: string,
  mode: string
) {
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

const MemberListCard = (props: MemberListProps) => {
  const badges = {} as { [member: string]: { name: string; style: string }[] };
  for (const member of props.members) {
    badges[member.membership_id] = [];
    if (member.clan) {
      switch (member.clan.role) {
        case 5:
          badges[member.membership_id].push({
            name: 'Leader',
            style: props.badgeRoleColors['Leader'] || '',
          });
          break;
        case 3: {
          badges[member.membership_id].push({
            name: 'Admin',
            style: props.badgeRoleColors['Admin'] || '',
          });
        }
      }

      badges[member.membership_id].push({
        name: member.clan.name,
        style:
          props.badgeClanColors[member.clan.name] || 'bg-yellow-400 text-black',
      });
    }
  }

  return (
    <Card className=" ">
      <Title>{props.listType}</Title>
      <List className="h-[23.25rem] overflow-y-auto">
        {props.members.map((member, memberIndex) => (
          <ListItem key={props.listType + '_member_list_' + memberIndex}>
            <Hyperlink
              href={generate_member_url(
                member.display_name,
                props.season,
                props.mode
              )}
              target="_blank"
              className="whitespace-nowrap text-ellipsis overflow-hidden w-[10rem] inline-block mr-2"
              title={member.display_name}
            >
              {member.display_name}
            </Hyperlink>
            {typeof badges[member.membership_id] !== 'undefined' ? (
              badges[member.membership_id].map((badge, badgeIndex) => (
                <span
                  key={
                    'member_' + member.membership_id + '_badge_' + badgeIndex
                  }
                  className={
                    'mb-4 mr-2 lg:my-0 shrink-0 grow-0 basis-auto px-1 py-[.25rem] text-xs align-middle inline-block h-auto w-auto w-min-[6rem] w-max-[10rem] self-start border-1 ' +
                    badge.style
                  }
                >
                  {badge.name}
                </span>
              ))
            ) : (
              <></>
            )}
          </ListItem>
        ))}
      </List>
    </Card>
  );
};

export const DestinyClanReportComponent = (props: ClanReportProps) => {
  const modes = (props.modes || []).join(',');
  const reportStatuses = new Map<string, boolean>();

  // what badges to display
  const badges = {} as { [name: string]: string };
  const badgeClanColors = {
    'Level Crush': 'bg-[#50AFE0] text-black',
    'Level Stomp': 'bg-[#44A8BD] text-black',
    'Righteous Indiggnation':
      'bg-gradient-to-r from-[#F988B6] to-[#7A4359] text-[#FAF2A2] border-[#F988B6] border-[1px]',
  } as { [clan: string]: string };

  const badgeColors = {
    Leader: 'bg-red-600 text-white',
    Admin: 'bg-yellow-400 text-black',
  };

  return (
    <div className=" clan-report relative top-0">
      <H3 className="text-yellow-400 text-ellipsis max-w-full whitespace-nowrap overflow-hidden">
        {props.clan}
      </H3>
      <H5>
        <span className="mr-2">{'[' + props.season + ']'}</span>
        <br />
        <span className="text-sm">{modes}</span>
      </H5>
      <Divider />
      <Grid numItemsLg={4} className="mt-4 gap-4">
        <Col numColSpanLg={3}>
          <Card>
            <Text>Placeholder</Text>
          </Card>
        </Col>
        <MemberListCard
          members={props.roster}
          report_status={reportStatuses}
          season={props.season.toString()}
          mode={modes || 'all'}
          badgeClanColors={badgeClanColors}
          badgeRoleColors={badgeColors}
          listType={'Clan Roster'}
        />
      </Grid>
    </div>
  );
};

export default DestinyClanReportComponent;
