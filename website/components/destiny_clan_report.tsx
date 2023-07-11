import { useEffect, useState } from 'react';
import { DestinyMemberReport } from '@website/core/api_responses';

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
  TabGroup,
  TabList,
  Tab,
  TabPanels,
  BarChart,
  AreaChart,
  DonutChart,
  Text,
  Legend,
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
      {props.members.map((data, memberIndex) => {
        const metric = data[props.metric];

        return (
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
            <span className="mr-4">
              {props.metric !== 'last_played_at'
                ? metric.toString()
                : new Date((metric as number) * 1000).toLocaleString('en-US')}
            </span>
          </ListItem>
        );
      })}
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

interface InstanceData {
  instance_id: number;
  occurred_at: number;
}

function getActivityAmounts(memberReports: DestinyMemberReport[]) {
  //
}

function getZeroActivityMembers(memberReports: DestinyMemberReport[]) {
  const member_reports = [] as DestinyMemberReport[];
  for (const member of memberReports) {
    if (member.activity_attempts === 0) {
      member_reports.push(member);
    }
  }

  member_reports.sort((a, b) =>
    a.display_name_global.localeCompare(b.display_name_global)
  );

  return member_reports;
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

type ReportStatusMap = MemberRosterListProps['reportStatuses'];
type ActivityBreakdown = { [group_id: string]: NetworkActivityClanBreakdown };
type ActivityTimeBucketMap = {
  [clan: string]: {
    [bucket: string]: InstanceData[];
  };
};
type ActivityClanMap = {
  [clan: string]: { name: string; day: string; activities: number }[];
};

type ActivityPeriod = {
  name: string;
  day: string;
  [clan: string]: string | number;
};
type ActivityIndexMap = { [day: string]: number };
type SumActivityStats = {
  activities: number;
  activities_completed: number;
  activities_completed_with_clan: number;
  percent_with_clan: number;
};

export const DestinyClanReportComponent = (props: ClanReportProps) => {
  const modes = (props.modes || []).join(',');

  const [sumStats, setSumStats] = useState({
    activities: 0,
    activities_completed: 0,
    activities_completed_with_clan: 0,
  } as SumActivityStats);
  const [reportStatuses, setReportStatuses] = useState({} as ReportStatusMap);
  const [reportMapData, setReportMapData] = useState({} as ReportMap);
  const [activityBreakdown, setActivityBreakdown] = useState(
    null as null | ActivityBreakdown
  );

  // things required by our component to render that are state dependent
  const [reportArrayData, setReportArrayData] = useState([] as MemberReport[]);
  const [activityTimeBuckets, setActivityTimeBuckets] = useState(
    {} as ActivityTimeBucketMap
  );

  const ignoreBuckets = ['0', '1', '2', '3', '4', '5', '6'];
  const [dailyActivityClanMap, setDailyActivityClanMap] = useState(
    {} as ActivityClanMap
  );
  const [weekdaysTimePeriods, setWeekdayTimePeriods] = useState([
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
  ] as ActivityPeriod[]);

  const [loadingActivityData, setLoadingActivityData] = useState(true);
  const [loadingBreakdownData, setLoadingBreakdownData] = useState(true);

  const [clans, setClans] = useState([] as string[]);
  const [dailyActivityIndexMap, setDailyActivityIndexMap] = useState(
    {} as ActivityIndexMap
  );

  const [dailyActivities, setDailyActivities] = useState(
    [] as ActivityPeriod[]
  );

  // start updating other parts of our state based off this map data
  useEffect(() => {
    // create time buckets
    console.log('Map data changed. Updating other parts of state');
    setReportArrayData(Object.values(reportMapData));
  }, [reportMapData]);

  useEffect(() => {
    console.log('Updated array version of the map data');
    setActivityTimeBuckets(
      reportArrayData ? createActivityPeriods(reportArrayData) : {}
    );
  }, [reportArrayData]);

  useEffect(() => {
    console.log('Generating week day time periods');
    const newWeekdaysTimePeriods = [
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
    ] as ActivityPeriod[];

    const clans = [] as string[];
    for (const clan in activityTimeBuckets) {
      clans.push(clan);
      for (let i = 0; i < newWeekdaysTimePeriods.length; i++) {
        const dayKey = newWeekdaysTimePeriods[i].day;
        newWeekdaysTimePeriods[i][clan] =
          typeof activityTimeBuckets[clan][dayKey] !== 'undefined'
            ? activityTimeBuckets[clan][dayKey].length
            : 0;
      }
    }
    setWeekdayTimePeriods(newWeekdaysTimePeriods);
    setClans(clans);
  }, [reportArrayData, activityTimeBuckets]);

  useEffect(() => {
    console.log('Generating activity maps');
    const newDailyActivityClanMap = {} as ActivityClanMap;
    const newDailyActivityIndexMap = {} as ActivityIndexMap;
    const newDailyActivities = [] as ActivityPeriod[];
    for (const clan in activityTimeBuckets) {
      newDailyActivityClanMap[clan] = [];
      for (const day in activityTimeBuckets[clan]) {
        if (!ignoreBuckets.includes(day)) {
          const date = new Date(day);
          const name =
            (date.getMonth() + 1).toString().padStart(2, '0') +
            '-' +
            (date.getDate() + '').padStart(2, '0') +
            '-' +
            date.getFullYear();
          newDailyActivityClanMap[clan].push({
            name: name,
            day: day,
            activities: activityTimeBuckets[clan][day].length,
          });
        }
      }
    }

    for (const clan in newDailyActivityClanMap) {
      for (let i = 0; i < newDailyActivityClanMap[clan].length; i++) {
        const dailyActivity = newDailyActivityClanMap[clan][i];
        //if(typeof dailyActivityIndexMap[dailyActivity.]
        const day = dailyActivity.day;

        // create a new daily activities entry and store the idnex
        if (typeof newDailyActivityIndexMap[day] === 'undefined') {
          // new
          const index = newDailyActivities.length;
          newDailyActivityIndexMap[day] = index;

          const details = {
            day: day,
            name: dailyActivity.name,
          } as { day: string; name: string; [clan: string]: number | string };

          for (const clanName of clans) {
            details[clanName] = 0;
          }

          newDailyActivities.push(details);
        }

        const index = newDailyActivityIndexMap[day];
        newDailyActivities[index][clan] = dailyActivity.activities;
      }
    }

    newDailyActivities.sort((a, b) => {
      const timestamp_a = new Date(a.day).getTime() * 1000;
      const timestamp_b = new Date(b.day).getTime() * 1000;
      return timestamp_a - timestamp_b;
    });

    setDailyActivityClanMap(newDailyActivityClanMap);
    setDailyActivityIndexMap(newDailyActivityIndexMap);
    setDailyActivities(newDailyActivities);

    console.log('Daily activities', newDailyActivities);
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
    setLoadingBreakdownData(true);
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
        '/report/activity/' +
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

    setLoadingBreakdownData(false);
  };

  const fetchClanReport = async () => {
    setLoadingActivityData(true);
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

    const newReportMapData = {} as ReportMap;
    const newReportStatuses = {} as ReportStatusMap;
    for (const member in reportsDone) {
      newReportMapData[member] = reportsDone[member];
      newReportStatuses[member] = true;
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

    if (needTimers.length === 0) {
      setLoadingActivityData(false);
    }

    setReportMapData(newReportMapData);
    setReportStatuses(newReportStatuses);
  };

  useEffect(() => {
    //

    if (activityBreakdown !== null) {
      let sumActivities = 0;
      let sumActivitiesCompletions = 0;
      let sumActivitiesCompletedWithClan = 0;
      let avgPercentWithClan = 0;
      for (const clan in activityBreakdown) {
        const breakdown = activityBreakdown[clan];
        sumActivities += breakdown.activity_attempts;
        sumActivitiesCompletions += breakdown.activities_completed;
        sumActivitiesCompletedWithClan +=
          breakdown.activities_completed_with_clan;
        avgPercentWithClan += breakdown.percent_with_clan;
      }

      avgPercentWithClan =
        avgPercentWithClan / Object.keys(activityBreakdown).length;

      setSumStats({
        activities: sumActivities,
        activities_completed: sumActivitiesCompletions,
        activities_completed_with_clan: sumActivitiesCompletedWithClan,
        percent_with_clan: avgPercentWithClan,
      });
    }
  }, [activityBreakdown]);

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
  const noActivityMembers = getZeroActivityMembers(reportArrayData);

  const categories = Object.keys(activityTimeBuckets);
  categories.sort();

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
      <Grid
        className={(loadingActivityData ? 'animate-pulse' : '') + ' mt-4 gap-4'}
        numItemsLg={4}
      >
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
                  categories={categories}
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
                  categories={categories}
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
      <Grid
        className={(loadingActivityData ? 'animate-pulse' : '') + ' mt-4 gap-4'}
        numItemsLg={3}
      >
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
        <MemberListCard
          members={noActivityMembers}
          listType={'Zero Activities'}
          season={props.season.toString()}
          mode={modes || 'all'}
          metric={'last_played_at'}
        />
      </Grid>
      <Grid
        className={
          (loadingBreakdownData ? 'animate-pulse' : '') + ' mt-4 gap-4'
        }
        numItemsLg={3}
      >
        <Card>
          <Title>
            {props.clan === 'network'
              ? 'Activity % completed with network'
              : 'Activity % completed with clan'}
          </Title>
          <Grid numItemsMd={7} className="gap-4 mt-4">
            <Col numColSpanMd={4}>
              <DonutChart
                index="name"
                category="metric"
                showAnimation={true}
                label={Math.ceil(sumStats.percent_with_clan) + '%'}
                data={
                  activityBreakdown !== null
                    ? Object.keys(activityBreakdown).map(
                        (clanId, clanIndex) => {
                          const data = activityBreakdown[clanId];
                          return {
                            name: data.name,
                            metric: data.percent_with_clan,
                          };
                        }
                      )
                    : []
                }
                valueFormatter={(input) => input + '%'}
              ></DonutChart>
            </Col>
            <Col numColSpanMd={3}>
              <Legend
                className="flex-col gap-4 items-center md:items-stretch"
                categories={
                  activityBreakdown !== null
                    ? Object.keys(activityBreakdown).map(
                        (v) => activityBreakdown[v].name
                      )
                    : []
                }
              ></Legend>
            </Col>
          </Grid>
        </Card>
      </Grid>
    </div>
  );
};

export default DestinyClanReportComponent;
