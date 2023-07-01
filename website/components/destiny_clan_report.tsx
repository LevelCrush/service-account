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
  AreaChart,
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
              {props.reportStatuses[member.membership_id] ? (
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

interface InstanceData {
  instance_id: number;
  occurred_at: number;
}

function createActivityPeriods(memberReports: DestinyMemberReport[]) {
  const clan_keys = [] as string[];
  const member_clan_keys = {} as { [member: string]: string };
  const buckets = {} as {
    [clan: string]: { [bucket: string]: InstanceData[] };
  };
  const alreadyScannedInstances = {} as {
    [clan: string]: { [instance_id: string]: boolean };
  };
  // scan and determine how many clans we need to worry about
  for (const memberReport of memberReports) {
    const clan = memberReport.member.clan;
    const clan_key = clan ? clan.name : 'N/A';
    member_clan_keys[memberReport.member.display_name] = clan_key;
    clan_keys.push(clan_key);

    if (!clan) {
      console.error(
        'No clan found!',
        memberReport.display_name_global,
        memberReport.membership_id,
        BigInt(memberReport.membership_id),
        BigInt(memberReport.membership_id).toString(),
        memberReport.membership_id.toString(),
        memberReport
      );
    }

    if (typeof buckets[clan_key] === 'undefined') {
      buckets[clan_key] = {};
    }
  }

  // now go through and build the activity periods
  for (const memberReport of memberReports) {
    const clan = memberReport.member.clan;
    const clan_key = clan ? clan.name : 'N/A';
    const instance_timestamps = memberReport.activity_timestamps;

    // bucket by mm-dd-yyyy
    for (const instance_id in instance_timestamps) {
      const timestamp = instance_timestamps[instance_id];

      if (typeof alreadyScannedInstances[clan_key] === 'undefined') {
        alreadyScannedInstances[clan_key] = {};
      }

      if (
        typeof alreadyScannedInstances[clan_key][instance_id] === 'undefined'
      ) {
        alreadyScannedInstances[clan_key][instance_id] = true;

        // generate bucket keys
        const datetime = new Date(timestamp * 1000);
        const dateMonthDay = datetime.getDate();
        const dateDay = datetime.getDay();
        const dateMonth = datetime.getMonth();
        const dateYear = datetime.getFullYear();

        const instance_record = {
          instance_id: parseInt(instance_id),
          occurred_at: timestamp,
        } as InstanceData;

        if (typeof buckets[clan_key][dateDay] === 'undefined') {
          buckets[clan_key][dateDay] = [];
        }

        const bucketKey =
          dateYear +
          '-' +
          (dateMonth + 1).toString().padStart(2, '0') +
          '-' +
          (dateMonthDay + '').padStart(2, '0');

        if (typeof buckets[clan_key][bucketKey] === 'undefined') {
          buckets[clan_key][bucketKey] = [];
        }

        buckets[clan_key][dateDay].push(instance_record);
        buckets[clan_key][bucketKey].push(instance_record);
      }
    }
  }

  return buckets;
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

            // bad, but for now it works to get the job done
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
      promises.push(fetchReport(member.membership_id));
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

    // bad, but for now it works to get the job done
    doForceUpdate();
  };

  useEffect(() => {
    setReportStatuses({});
    setReportMapData({});
    console.log('Starting initial report fetch');
    startInitialReportFetch();

    doForceUpdate();

    return () => {
      for (const id in fetchTimers.current) {
        if (fetchTimers.current[id]) {
          window.clearTimeout(fetchTimers.current[id]);
          fetchTimers.current[id] = 0;
        }
      }
    };
  }, [props.season, props.clan, props.modes]);

  const activityColors = [
    'yellow',
    'blue',
    'emerald',
  ] as CardProps['decorationColor'][];

  // create time buckets
  const activityTimeBuckets = reportMapData
    ? createActivityPeriods(Object.values(reportMapData))
    : {};

  // activity overtime
  const ignoreBuckets = ['0', '1', '2', '3', '4', '5', '6'];
  const dailyActivityClanMap = {} as {
    [clan: string]: { name: string; day: string; activities: number }[];
  };

  const weekdaysTimePeriods = [
    {
      name: 'Monday',
      day: '1',
    },
    {
      name: 'Tuesday',
      day: '2',
    },
    {
      name: 'Wednesday',
      day: '3',
    },
    {
      name: 'Thursday',
      day: '4',
    },
    {
      name: 'Friday',
      day: '5',
    },
    {
      name: 'Saturday',
      day: '6',
    },
    {
      name: 'Sunday',
      day: '0',
    },
  ] as {
    name: string;
    day: string;
    [clan: string]: string | number;
  }[];

  const clans = [] as string[];
  for (const clan in activityTimeBuckets) {
    clans.push(clan);
    for (let i = 0; i < weekdaysTimePeriods.length; i++) {
      const dayKey = weekdaysTimePeriods[i].day;
      weekdaysTimePeriods[i][clan] =
        typeof activityTimeBuckets[clan][dayKey] !== 'undefined'
          ? activityTimeBuckets[clan][dayKey].length
          : 0;
    }
  }

  for (const clan in activityTimeBuckets) {
    dailyActivityClanMap[clan] = [];
    for (const day in activityTimeBuckets[clan]) {
      if (!ignoreBuckets.includes(day)) {
        const date = new Date(day);
        const name =
          (date.getMonth() + 1).toString().padStart(2, '0') +
          '-' +
          (date.getDate() + '').padStart(2, '0') +
          '-' +
          date.getFullYear();
        dailyActivityClanMap[clan].push({
          name: name,
          day: day,
          activities: activityTimeBuckets[clan][day].length,
        });
      }
    }
  }

  const dailyActivityIndexMap = {} as {
    [day: string]: number;
  };

  const dailyActivities = [] as {
    day: string;
    name: string;
    [clan: string]: number | string;
  }[];

  for (const clan in dailyActivityClanMap) {
    for (let i = 0; i < dailyActivityClanMap[clan].length; i++) {
      const dailyActivity = dailyActivityClanMap[clan][i];
      //if(typeof dailyActivityIndexMap[dailyActivity.]
      const day = dailyActivity.day;

      // create a new daily activities entry and store the idnex
      if (typeof dailyActivityIndexMap[day] === 'undefined') {
        // new
        const index = dailyActivities.length;
        dailyActivityIndexMap[day] = index;

        const details = {
          day: day,
          name: dailyActivity.name,
        } as { day: string; name: string; [clan: string]: number | string };

        for (const clanName of clans) {
          details[clanName] = 0;
        }

        dailyActivities.push(details);
      }

      const index = dailyActivityIndexMap[day];
      dailyActivities[index][clan] = dailyActivity.activities;
    }
  }

  dailyActivities.sort((a, b) => {
    const timestamp_a = new Date(a.day).getTime() * 1000;
    const timestamp_b = new Date(b.day).getTime() * 1000;
    return timestamp_a - timestamp_b;
  });

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
          <Title>Activity Periods</Title>
          <TabGroup>
            <TabList>
              <Tab>By Day</Tab>
              <Tab>Overtime</Tab>
            </TabList>
            <TabPanels>
              <TabPanel>
                <BarChart
                  className="h-[23.25rem]"
                  data={weekdaysTimePeriods}
                  categories={Object.keys(activityTimeBuckets)}
                  index={'name'}
                  showAnimation={true}
                  showLegend={true}
                ></BarChart>
              </TabPanel>
              <TabPanel>
                <AreaChart
                  className="h-[23.25rem]"
                  startEndOnly={true}
                  data={dailyActivities}
                  categories={Object.keys(activityTimeBuckets)}
                  index={'name'}
                  showAnimation={true}
                  showLegend={true}
                ></AreaChart>
              </TabPanel>
            </TabPanels>
          </TabGroup>
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
