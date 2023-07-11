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
import {
  MemberReport,
  MemberResponse,
  NetworkActivityClanBreakdown,
  ReportOutput,
} from '@levelcrush/service-destiny';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSpinner } from '@fortawesome/free-solid-svg-icons';
import useDeepCompareEffect from 'use-deep-compare-effect';
import { APIResponse } from '@levelcrush';

export interface ClanReportProps {
  clan: string;
  roster: MemberResponse[];
  season: 'lifetime' | number;
  modes?: string[];
}

interface MemberRosterListProps extends CardProps {
  members: MemberResponse[];
  reportStatuses: { [membership_id: string]: boolean };
  season: string;
  mode: string;
  listType: string;
  badgeRoleColors: { [role: string]: string };
  badgeClanColors: { [clan: string]: string };
}

interface MemberListProps extends CardProps {
  members: DestinyMemberReport[];
  listType: string;
  season: string;
  mode: string;
  metric: keyof DestinyMemberReport;
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

const MemberListCard = (props: MemberListProps) => (
  <Card className=" ">
    <Title>{props.listType}</Title>
    <List className="h-[23.25rem] overflow-y-auto">
      {props.members.map((data, memberIndex) => (
        <ListItem key={props.listType + '_member_list_' + memberIndex}>
          <Hyperlink
            href={generate_member_url(
              data.display_name_global,
              props.season,
              props.mode
            )}
            target="_blank"
            className=" whitespace-nowrap text-ellipsis overflow-hidden w-[10rem] inline-block mr-2"
            title={data.member.display_name}
          >
            {(memberIndex + 1).toString().padStart(2, '0') +
              '. ' +
              data.member.display_name}
          </Hyperlink>
          <span className="mr-4">{(data as any)[props.metric]}</span>
        </ListItem>
      ))}
    </List>
  </Card>
);

const MemberRosterCard = (props: MemberRosterListProps) => {
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

function getActivityAmounts(memberReports: DestinyMemberReport[]) {
  //
}

/**
 * Looks through the member reports and finds the most activity by activity attempts
 * @param memberReports
 * @returns
 */
function getMostActiveMembers(memberReports: DestinyMemberReport[]) {
  const member_activities = {} as { [membership_id: string]: number };
  const member_reports = {} as { [membership_id: string]: DestinyMemberReport };
  for (const member of memberReports) {
    member_activities[member.membership_id] = member.activity_attempts;
    member_reports[member.membership_id] = member;
  }

  const membership_ids = Object.keys(member_activities);
  membership_ids.sort((a, b) => member_activities[b] - member_activities[a]);

  const sorted_members = [] as DestinyMemberReport[];
  for (const membership_id of membership_ids) {
    sorted_members.push(member_reports[membership_id]);
  }
  return sorted_members;
}

/**
 * Looks through the member reports and finds the most activity by activity attempts DONE WITH CLAN MEMBERS
 * @param memberReports
 * @returns
 */
function getMostClanActiveMembers(memberReports: DestinyMemberReport[]) {
  const member_activities = {} as { [membership_id: string]: number };
  const member_reports = {} as {
    [membership_id: string]: DestinyMemberReport;
  };
  for (const member of memberReports) {
    member_activities[member.membership_id] =
      member.activity_attempts_with_clan;
    member_reports[member.membership_id] = member;
  }

  const membership_ids = Object.keys(member_activities);
  membership_ids.sort((a, b) => member_activities[b] - member_activities[a]);

  const sorted_members = [] as DestinyMemberReport[];
  for (const membership_id of membership_ids) {
    sorted_members.push(member_reports[membership_id]);
  }
  return sorted_members;
}

/**
 * Looks through the member reports and generates activity period data over time, and by day
 * @param memberReports
 * @returns
 */
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
  let [reportStatuses, setReportStatuses] = useState(
    {} as MemberRosterListProps['reportStatuses']
  );
  let [reportMapData, setReportMapData] = useState({} as ReportMap);
  let [activityBreakdown, setActivityBreakdown] = useState(
    null as null | { [group_id: string]: NetworkActivityClanBreakdown }
  );

  // things required by our component to render that are state dependent
  let [reportArrayData, setReportArrayData] = useState([] as MemberReport[]);
  let [activityTimeBuckets, setActivityTimeBuckets] = useState(
    {} as {
      [clan: string]: {
        [bucket: string]: InstanceData[];
      };
    }
  );
  const ignoreBuckets = ['0', '1', '2', '3', '4', '5', '6'];
  let [dailyActivityClanMap, setDailyActivityClanMap] = useState(
    {} as {
      [clan: string]: { name: string; day: string; activities: number }[];
    }
  );
  let [weekdaysTimePeriods, setWeekdayTimePeriods] = useState([
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
  }[]);

  let [clans, setClans] = useState([] as string[]);
  let [dailyActivityIndexMap, setDailyActivityIndexMap] = useState(
    {} as { [day: string]: number }
  );

  let [dailyActivities, setDailyActivities] = useState(
    [] as {
      day: string;
      name: string;
      [clan: string]: number | string;
    }[]
  );

  // start updating other parts of our state based off this map data
  useDeepCompareEffect(() => {
    // create time buckets
    console.log('Map data changed. Updating other parts of state');
    setReportArrayData(Object.values(reportMapData));
    setActivityTimeBuckets(
      reportMapData ? createActivityPeriods(reportArrayData) : {}
    );
  }, [reportMapData]);

  useDeepCompareEffect(() => {
    console.log('Generating week day time periods');
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
    setWeekdayTimePeriods(weekdaysTimePeriods);
    setClans(clans);
  }, [reportArrayData, activityTimeBuckets]);

  useDeepCompareEffect(() => {
    console.log('Generating activity maps');
    dailyActivityClanMap = {};
    dailyActivityIndexMap = {};
    dailyActivities = [];
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
    setDailyActivityClanMap(dailyActivityClanMap);

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

    setDailyActivityIndexMap(dailyActivityIndexMap);
    setDailyActivities(dailyActivities);

    console.log('Daily activities', dailyActivities);
  }, [weekdaysTimePeriods, clans]);

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
    setTimeout(() => forceUpdate(), 1000);
  };

  const fetchReport = async (clan: string) => {
    const modeString = (props.modes || []).join(',');
    const reportType =
      props.season === 'lifetime'
        ? 'lifetime'
        : 'season/' + encodeURIComponent(props.season);

    const clanPath =
      clan === 'network' ? 'network' : 'clan/' + encodeURIComponent(clan);
    const apiResponse = await fetch(
      ENV.hosts.destiny +
        '/' +
        clanPath +
        '/report/' +
        reportType +
        (modeString.length > 0
          ? '?modes=' + encodeURIComponent(modeString)
          : '')
    );
    if (apiResponse.ok) {
      const data = (await apiResponse.json()) as APIResponse<{
        [membership_id: string]: ReportOutput;
      }>;
      return data;
    } else {
      return null;
    }
  };

  const fetchActivityBreakdown = async () => {
    const clan = props.clan;
    const modeString = (props.modes || []).join(',');
    const reportType =
      props.season === 'lifetime'
        ? 'lifetime'
        : 'season/' + encodeURIComponent(props.season);

    const clanPath =
      clan === 'network' ? 'network' : 'clan/' + encodeURIComponent(clan);
    const apiResponse = await fetch(
      ENV.hosts.destiny +
        '/' +
        clanPath +
        '/report/activities/' +
        reportType +
        (modeString.length > 0
          ? '?modes=' + encodeURIComponent(modeString)
          : '')
    );

    if (apiResponse.ok) {
      const data = (await apiResponse.json()) as APIResponse<{
        [group_id: string]: NetworkActivityClanBreakdown;
      }>;
      setActivityBreakdown(data.response);
    } else {
      setActivityBreakdown(null);
    }
  };

  const fetchClanReport = async () => {
    const result = await fetchReport(props.clan);
    const needTimers = [] as string[];
    const reportsDone = {} as { [member: string]: DestinyMemberReport };
    if (result && result.response) {
      for (const member in result.response) {
        const data = result.response[member];
        const reportType = getReportType(data);

        switch (reportType) {
          case 'loading':
            needTimers.push(member);
            break;
          case 'report':
            reportsDone[member] = data as DestinyMemberReport;
            break;
          case 'unknown':
            console.log('Unknown case', data);
            break;
        }
      }
    } else {
      console.error('Unable to fetch clan report', props.clan);
    }

    console.log('Merging completed reports', reportsDone);
    reportMapData = {};
    reportStatuses = {};
    for (const member in reportsDone) {
      reportMapData[member] = reportsDone[member];
      reportStatuses[member] = true;
    }

    if (needTimers.length > 0) {
      console.warn(
        'Not all member reports are done. Attempting to refetch shortly',
        needTimers.length
      );
      setTimeout(() => {
        fetchClanReport();
      }, 5 * 1000);
    }

    setReportMapData(reportMapData);
    setReportStatuses(reportStatuses);

    //doForceUpdate();
  };

  useEffect(() => {
    console.log('Starting initial report fetch');
    fetchClanReport();

    console.log('Fetching activity breakdown');
    fetchActivityBreakdown();

    //    doForceUpdate();

    return () => {
      // nothing to clenaup here
    };
  }, [props.season, props.clan, props.modes]);

  const activityColors = [
    'yellow',
    'blue',
    'emerald',
  ] as CardProps['decorationColor'][];

  // create time buckets
  /* const reportArrayData = Object.values(reportMapData);
  const activityTimeBuckets = reportMapData
    ? createActivityPeriods(reportArrayData)
    : {};

  // activity overtime
  const ignoreBuckets = ['0', '1', '2', '3', '4', '5', '6'];
  
  const dailyActivityClanMap2 = {} as {
    [clan: string]: { name: string; day: string; activities: number }[];
  };

  /*
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
  }); */

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

  // bucket members
  const mostActiveMembers = getMostActiveMembers(reportArrayData);
  const mostActiveMembersWithClan = getMostClanActiveMembers(reportArrayData);

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
        <MemberRosterCard
          members={props.roster}
          reportStatuses={reportStatuses}
          season={props.season.toString()}
          mode={modes || 'all'}
          badgeClanColors={badgeClanColors}
          badgeRoleColors={badgeColors}
          listType={'Clan Roster'}
        />
      </Grid>
      <Divider />
      <Grid className="gap-4 mt-4" numItemsLg={4}>
        <MemberListCard
          members={mostActiveMembers}
          listType={'Most Activities'}
          season={props.season.toString()}
          mode={modes || 'all'}
          metric={'activity_attempts'}
        />
        <MemberListCard
          members={mostActiveMembersWithClan}
          listType={'Most Activities With Clan'}
          season={props.season.toString()}
          mode={modes || 'all'}
          metric={'activity_attempts_with_clan'}
        />
        <Col numColSpanLg={1}></Col>
      </Grid>
    </div>
  );
};

export default DestinyClanReportComponent;
