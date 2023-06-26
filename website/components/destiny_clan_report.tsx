import { useEffect, useRef, useState } from 'react';
import {
  DestinyMemberReport,
  DestinyMemberReportResponse,
  DestinyMemberResponse,
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
  ProgressBar,
} from '@tremor/react';
import Hyperlink from '@website/components/elements/hyperlink';
import { MemberResponse, ReportOutput } from '@levelcrush/service-destiny';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSpinner } from '@fortawesome/free-solid-svg-icons';
import useDeepCompareEffect from 'use-deep-compare-effect';

export interface ClanReportProps {
  clan: string;
  roster: MemberResponse[];
  season: 'lifetime' | number;
  modes?: string[];
}

interface MemberListProps extends CardProps {
  members: MemberResponse[];
  reportStatuses: { [membership_id: string]: boolean };
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
    console.log(
      'Is Done',
      props.reportStatuses[member.display_name],
      member.display_name
    );
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
              className=" whitespace-nowrap text-ellipsis overflow-hidden w-[10rem] inline-block mr-2"
              title={member.display_name}
            >
              {props.reportStatuses[member.display_name] ? (
                <></>
              ) : (
                <FontAwesomeIcon className="mr-4" icon={faSpinner} spin />
              )}
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

type FetchReportResult = {
  member: string;
  data: DestinyMemberReportResponse | null;
};

type ReportMap = { [member: string]: DestinyMemberReport };

//https://stackoverflow.com/questions/46240647/how-to-force-a-functional-react-component-to-render/53837442#53837442
//create your forceUpdate hook
function useForceUpdate() {
  const [value, setValue] = useState(0); // integer state
  return () => setValue((value) => value + 1); // update state to force render
  // A function that increment 👆🏻 the previous state like here
  // is better than directly setting `setValue(value + 1)`
}

export const DestinyClanReportComponent = (props: ClanReportProps) => {
  const forceUpdate = useForceUpdate();

  const modes = (props.modes || []).join(',');
  const [reportStatuses, setReportStatuses] = useState(
    {} as MemberListProps['reportStatuses']
  );
  const [reportMapData, setReportMapData] = useState({} as ReportMap);
  const fetchTimers = useRef({} as { [id: string]: number });

  const getReportType = (memberReport: ReportOutput | null) => {
    switch (typeof memberReport) {
      case 'bigint':
      case 'number':
        return 'loading';
      case 'object':
        return 'report';
      default:
        return 'unknown';
    }
  };

  const doForceUpdate = () => {
    setTimeout(() => forceUpdate(), 250);
  };

  /**
   * Fetch the report of a user and constantly check in if the report is still being generated
   * @param bungie_name
   * @param report_type
   */
  const fetchReport = async (bungie_name: string) => {
    const modeString = (props.modes || []).join(',');
    const reportType =
      props.season === 'lifetime'
        ? 'lifetime'
        : 'season/' + encodeURIComponent(props.season);

    const apiResponse = await fetch(
      ENV.hosts.destiny +
        '/member/' +
        encodeURIComponent(bungie_name) +
        '/report/' +
        reportType +
        (modeString.length > 0
          ? '?modes=' + encodeURIComponent(modeString)
          : '')
    );

    if (apiResponse.ok) {
      const data = (await apiResponse.json()) as DestinyMemberReportResponse;
      return { member: bungie_name, data: data } as FetchReportResult;
    } else {
      return { member: bungie_name, data: null } as FetchReportResult;
    }
  };

  const createFetchTimer = (bungie_name: string) => {
    const timer = window.setTimeout(async () => {
      const fetchResult = await fetchReport(bungie_name);
      if (fetchResult.data) {
        const data = fetchResult.data.response;
        const reportType = getReportType(data);
        switch (reportType) {
          case 'loading':
            createFetchTimer(fetchResult.member);
            break;
          case 'report':
            reportMapData[fetchResult.member] = data as DestinyMemberReport;
            reportStatuses[fetchResult.member] = true;
            setReportMapData(reportMapData);
            setReportStatuses(reportStatuses);
            doForceUpdate();
            break;
          case 'unknown':
            console.log('Unknown result:', data);
            break;
        }
      }
    }, 10 * 1000);
    fetchTimers.current[bungie_name] = timer;
  };

  const processInitialReports = (
    reports: PromiseFulfilledResult<FetchReportResult>[]
  ) => {
    const needTimers = [] as string[];
    const reportsDone = {} as { [member: string]: DestinyMemberReport };

    for (const reportPromise of reports) {
      const report = reportPromise.value;
      if (report.data === null) {
        continue;
      }

      const data = report.data.response;
      const reportType = getReportType(data);

      switch (reportType) {
        case 'loading':
          needTimers.push(report.member);
          break;
        case 'report':
          reportsDone[report.member] = data as DestinyMemberReport;
          break;
        case 'unknown':
          console.log('Unknown case', data);
          break;
      }
    }

    return {
      needTimers,
      reportsDone,
    };
  };

  const startInitialReportFetch = async () => {
    const promises = [] as Promise<FetchReportResult>[];
    for (const member of props.roster) {
      promises.push(fetchReport(member.display_name));
    }
    console.log('Executing all request');
    const results = await Promise.allSettled(promises);
    const success_results = results.filter((result) => {
      return result.status === 'fulfilled';
    });

    console.log(
      'Total Responses: ',
      results.length,
      'Total Success',
      success_results.length
    );
    console.log('Processing only the successful responses');
    const reportResults = processInitialReports(
      success_results as PromiseFulfilledResult<FetchReportResult>[]
    );

    console.log('Merging completed reports', reportResults.reportsDone);
    for (const member in reportResults.reportsDone) {
      reportMapData[member] = reportResults.reportsDone[member];
      reportStatuses[member] = true;
    }

    console.log(
      'Setting up fetch timers for the rest',
      reportResults.needTimers
    );
    for (const member of reportResults.needTimers) {
      createFetchTimer(member);
    }

    setReportMapData(reportMapData);
    setReportStatuses(reportStatuses);

    doForceUpdate();
  };

  useEffect(() => {
    console.log('Starting initial report fetch');
    startInitialReportFetch();

    return () => {
      for (const id in fetchTimers.current) {
        if (fetchTimers.current[id]) {
          window.clearTimeout(fetchTimers.current[id]);
          fetchTimers.current[id] = 0;
        }
      }
    };
  }, []);

  // what badges to display
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
          reportStatuses={reportStatuses}
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
