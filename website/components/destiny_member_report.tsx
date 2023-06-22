import { useEffect, useState } from 'react';
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

export interface MemberReportProps {
  bungie_name: string;
  season: 'lifetime' | number;
  modes?: string[];
  onReportLoaded?: (data: DestinyMemberReport) => void;
  onLoadingData?: () => void;
}

interface TitleCardProps extends CardProps {
  titles: DestinyMemberReport['titles'];
}

const TitleCard = (prop: TitleCardProps) => {
  const [totalTitles, setTotalTitles] = useState(prop.titles.length);
  const [titles, setTitles] = useState(prop.titles);

  useEffect(() => {
    setTotalTitles(prop.titles.length);

    const ntitles = [...prop.titles];
    if (ntitles.length < 10) {
      const diff = 10 - ntitles.length;
      for (let i = 0; i < diff; i++) {
        ntitles.push({
          title: 'Pending...',
          gilded_amount: 0,
          gilded_past: false,
          gilded_season: false,
        });
      }
    }
    setTitles(ntitles);
  }, [prop.titles]);

  return (
    <Card {...prop}>
      <Title>{'Titles (' + totalTitles + ')'}</Title>
      <List className="overflow-y-scroll h-[23.25rem]">
        {titles.map((data, dataIndex) => (
          <ListItem key={'member_title_' + dataIndex}>
            <span
              className={
                data.gilded_season
                  ? 'text-yellow-400'
                  : data.gilded_past
                  ? 'text-yellow-700'
                  : ''
              }
            >
              {data.title}
            </span>
            <span className={titles.length > 10 ? 'pr-4' : ''}>
              {'x' + data.gilded_amount + ' gilds'}
            </span>
          </ListItem>
        ))}
      </List>
    </Card>
  );
};

interface FireteamCardProps extends CardProps {
  members: DestinyMemberReport['frequent_clan_members'];
  fireteamType: string;
}

const FireteamCard = (props: FireteamCardProps) => {
  return (
    <Card>
      <Title>{props.fireteamType}</Title>
      <List className="h-[23.25rem] overflow-y-auto">
        {props.members.map((member, memberIndex) => (
          <ListItem
            key={props.fireteamType + '_fireteam_member_' + memberIndex}
          >
            <Hyperlink
              href={'/admin/report/' + encodeURIComponent(member.display_name)}
              className="whitespace-nowrap text-ellipsis overflow-hidden max-w-[10rem] inline-block"
              title={member.display_name}
            >
              {member.display_name}
            </Hyperlink>
            <span className="pl-4">{member.activities + ' activities'}</span>
          </ListItem>
        ))}
      </List>
    </Card>
  );
};

interface StatData {
  name: string;
  amount: number;
}

interface StatBlockProp {
  data: StatData[];
  title: string;
}
const StatBlockCard = (props: StatBlockProp) => (
  <Card>
    <Title>{props.title}</Title>
    <Grid numItemsMd={6} className="gap-4 mt-4">
      <Col numColSpanMd={4}>
        <DonutChart
          showAnimation={true}
          data={props.data}
          category="amount"
          index="name"
          valueFormatter={(input) =>
            Intl.NumberFormat('us').format(input).toString()
          }
        ></DonutChart>
      </Col>
      <Col numColSpanMd={2}>
        <Legend
          className="flex-col gap-4 items-center md:items-stretch"
          categories={props.data.map((v) => v.name)}
        ></Legend>
      </Col>
    </Grid>
  </Card>
);

function stat_type_name(stat_type: string) {
  switch (stat_type) {
    case 'pve':
      return 'PvE';
    case 'pvp':
      return 'PvP';
    case 'gambit':
      return 'Gambit';
    case 'private_matches':
      return 'Private Matches';
    case 'reckoning':
      return 'Reckoning';
    default:
      return 'unknown';
  }
}

function combineStats(
  memberReport: DestinyMemberReport,
  stat: keyof DestinyMemberStats
) {
  const combine = [] as StatData[];
  for (const key in memberReport) {
    if (key.includes('stats_')) {
      const stat_split = key.split('_');
      const stat_type = key.replace(stat_split[0] + '_', '');
      const name = stat_type_name(stat_type);
      combine.push({
        name: name,
        amount: (memberReport as any)[key][stat],
      });
    }
  }
  return combine;
}

interface InstanceData {
  instance_id: number;
  occurred_at: number;
}

function createActivityPeriods(memberReport: DestinyMemberReport) {
  const instance_timestamps = memberReport.activity_timestamps;

  const buckets = {} as {
    [bucket_id: string]: InstanceData[];
  };

  // bucket by mm-dd-yyyy
  for (const instance_id in instance_timestamps) {
    const timestamp = instance_timestamps[instance_id];

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

    if (typeof buckets[dateDay] === 'undefined') {
      buckets[dateDay] = [];
    }

    const bucketKey =
      dateYear +
      '-' +
      (dateMonth + 1).toString().padStart(2, '0') +
      '-' +
      (dateMonthDay + '').padStart(2, '0');

    if (typeof buckets[bucketKey] === 'undefined') {
      buckets[bucketKey] = [];
    }

    buckets[dateDay].push(instance_record);
    buckets[bucketKey].push(instance_record);
  }

  return buckets;
}

function renderOverall(memberReport: DestinyMemberReport, modes: string[]) {
  const overallKills = combineStats(memberReport, 'kills');
  const overallDeaths = combineStats(memberReport, 'deaths');
  const overallAssists = combineStats(memberReport, 'assists');
  const overallVictories = combineStats(memberReport, 'victories');
  const overallDefeats = combineStats(memberReport, 'defeats');
  const overallActivities = combineStats(memberReport, 'activities');
  const overallCompletions = combineStats(memberReport, 'activity_completions');
  const overallFullCompletes = combineStats(
    memberReport,
    'activities_completed_start_to_finish'
  );

  const activityColors = [
    'yellow',
    'blue',
    'emerald',
  ] as CardProps['decorationColor'][];

  const topActivityModeCount = 3;

  const activityTimeBuckets = createActivityPeriods(memberReport);

  const weekdaysTimePeriods = [
    {
      name: 'Monday',
      Activities:
        typeof activityTimeBuckets['1'] !== 'undefined'
          ? activityTimeBuckets['1'].length
          : 0,
    },
    {
      name: 'Tuesday',
      Activities:
        typeof activityTimeBuckets['2'] !== 'undefined'
          ? activityTimeBuckets['2'].length
          : 0,
    },
    {
      name: 'Wednesday',
      Activities:
        typeof activityTimeBuckets['3'] !== 'undefined'
          ? activityTimeBuckets['3'].length
          : 0,
    },
    {
      name: 'Thursday',
      Activities:
        typeof activityTimeBuckets['4'] !== 'undefined'
          ? activityTimeBuckets['4'].length
          : 0,
    },
    {
      name: 'Friday',
      Activities:
        typeof activityTimeBuckets['5'] !== 'undefined'
          ? activityTimeBuckets['5'].length
          : 0,
    },
    {
      name: 'Saturday',
      Activities:
        typeof activityTimeBuckets['6'] !== 'undefined'
          ? activityTimeBuckets['6'].length
          : 0,
    },
    {
      name: 'Sunday',
      Activities:
        typeof activityTimeBuckets['0'] !== 'undefined'
          ? activityTimeBuckets['0'].length
          : 0,
    },
  ];

  const ignoreBuckets = ['0', '1', '2', '3', '4', '5', '6'];
  const dailyActivities = [];
  for (const bucket in activityTimeBuckets) {
    if (!ignoreBuckets.includes(bucket)) {
      const date = new Date(bucket);
      const name =
        (date.getMonth() + 1).toString().padStart(2, '0') +
        '-' +
        (date.getDate() + '').padStart(2, '0') +
        '-' +
        date.getFullYear();

      dailyActivities.push({
        name: name,
        bucket: bucket,
        Activities: activityTimeBuckets[bucket].length,
      });
    }
  }
  dailyActivities.sort((a, b) => {
    const timestamp_a = new Date(a.bucket).getTime() * 1000;
    const timestamp_b = new Date(b.bucket).getTime() * 1000;
    return timestamp_a - timestamp_b;
  });

  return (
    <div className="report-view " data-view="overall">
      <Grid numItemsLg={6} className="gap-4">
        <Col numColSpanLg={4} className="space-y-4">
          <Title>Activity Periods</Title>
          <TabGroup>
            <TabList>
              <Tab>By Day</Tab>
              <Tab>Overtime</Tab>
            </TabList>
            <TabPanels>
              <TabPanel>
                <BarChart
                  data={weekdaysTimePeriods}
                  categories={['Activities']}
                  index={'name'}
                  showAnimation={true}
                  showLegend={false}
                ></BarChart>
              </TabPanel>
              <TabPanel>
                <LineChart
                  startEndOnly={true}
                  data={dailyActivities}
                  categories={['Activities']}
                  index={'name'}
                  showAnimation={true}
                  showLegend={false}
                ></LineChart>
              </TabPanel>
            </TabPanels>
          </TabGroup>
        </Col>
        <Col numColSpanLg={2} className="space-y-4">
          {modes.length > 0 ? (
            <>
              <Title>{'Top ' + topActivityModeCount + ' Activities'}</Title>
              {memberReport.top_activities
                .slice(0, topActivityModeCount)
                .map((activity, activityIndex) => (
                  <Card
                    key={'overall_report_activity_' + activityIndex}
                    decoration="top"
                    decorationColor={
                      activityColors[activityIndex % activityColors.length]
                    }
                  >
                    <Text>{activity.name}</Text>
                    <Metric>{activity.attempts}</Metric>
                  </Card>
                ))}
            </>
          ) : (
            <>
              <Title>{'Top ' + topActivityModeCount + ' Modes'}</Title>
              {memberReport.top_activity_modes
                .slice(0, topActivityModeCount)
                .map((activityMode, activityModeIndex) => (
                  <Card
                    key={'overall_report_mode_activity_' + activityModeIndex}
                    decoration="top"
                    decorationColor={
                      activityColors[activityModeIndex % activityColors.length]
                    }
                  >
                    <Text>{activityMode.mode}</Text>
                    <Metric>{activityMode.count}</Metric>
                  </Card>
                ))}
            </>
          )}
        </Col>
      </Grid>
      <Grid numItemsLg={3} className="mt-4 gap-4">
        <StatBlockCard title="Kills" data={overallKills} />
        <StatBlockCard title="Deaths" data={overallDeaths} />
        <StatBlockCard title="Assists" data={overallAssists} />
        <StatBlockCard title="Activities" data={overallActivities} />
        {modes.includes('4') ? (
          <>
            <StatBlockCard title="Completions" data={overallCompletions} />
            <StatBlockCard
              title="Completions (Start to Finish)"
              data={overallFullCompletes}
            />
          </>
        ) : (
          <>
            <StatBlockCard title="Victories" data={overallVictories} />
            <StatBlockCard title="Defeats" data={overallDefeats} />
          </>
        )}
      </Grid>
      <Grid numItemsLg={3} className="mt-4 gap-4">
        <TitleCard titles={memberReport.titles} />
        <FireteamCard
          members={memberReport.frequent_clan_members}
          fireteamType="Network Fireteam"
        />
        <FireteamCard
          members={memberReport.frequent_non_clan_members}
          fireteamType="Not Network Fireteam"
        />
      </Grid>
    </div>
  );
}

export const DestinyMemberReportComponent = (props: MemberReportProps) => {
  const [memberReport, setMemberReport] = useState(
    null as DestinyMemberReportResponse['response']
  );

  const [alreadyLoadedData, setAlreadyLoadedData] = useState(false);
  const [isLoadingData, setIsLoadingData] = useState(false);
  const [fetchTimerInterval, setFetchTimerInterval] = useState(0);

  const fetchReport = async (bungie_name: string, report_type: string) => {
    const modeString = (props.modes || []).join(',');
    const apiResponse = await fetch(
      ENV.hosts.destiny +
        '/member/' +
        encodeURIComponent(bungie_name) +
        '/report/' +
        report_type +
        (modeString.length > 0
          ? '?modes=' + encodeURIComponent(modeString)
          : '')
    );

    const data = (await apiResponse.json()) as DestinyMemberReportResponse;

    if (
      typeof data.response === 'number' ||
      typeof data.response === 'bigint'
    ) {
      setFetchTimerInterval(
        window.setTimeout(() => {
          fetchReport(bungie_name, report_type);
        }, 1 * 5000)
      ); // check on the report every 1 seconds
      if (props.onLoadingData) {
        props.onLoadingData();
      }
    } else {
      // stop timer
      if (fetchTimerInterval) {
        console.log('Data received. Stopping current timer');
        window.clearTimeout(fetchTimerInterval);
        setFetchTimerInterval(0);
      }
    }

    // set membger report response
    if (!alreadyLoadedData) {
      // if we have never loaded any data into our report, update our response

      setMemberReport(data.response);
    } else if (typeof data.response === 'object') {
      // only update our member report when we have the report in our responsea.response.search.modes == props.modes?.join(',')

      setIsLoadingData(false);
      setMemberReport(data.response);
    } else {
      setIsLoadingData(true);
    }

    if (!alreadyLoadedData && typeof data.response === 'object') {
      setAlreadyLoadedData(true);
    }
  };

  useEffect(() => {
    if (
      memberReport !== null &&
      typeof memberReport === 'number' &&
      typeof memberReport === 'bigint' &&
      props.onLoadingData
    ) {
      console.log('Loading data!');
      props.onLoadingData();
    } else if (memberReport !== null && typeof memberReport === 'object') {
      setAlreadyLoadedData(true);
      if (props.onReportLoaded) {
        props.onReportLoaded(memberReport as DestinyMemberReport);
      }
    }
  }, [memberReport]);

  // fetch the member report on load
  useEffect(() => {
    const report_type =
      props.season === 'lifetime' ? 'lifetime' : 'season/' + props.season;

    if (props.bungie_name && props.bungie_name.trim().length > 0) {
      // fetch the member report
      fetchReport(props.bungie_name, report_type).finally(() =>
        console.log('Member report fetched for: ', props.bungie_name)
      );
    }

    return () => {
      // cleanup
      if (fetchTimerInterval) {
        console.log('Stopping current timeout', fetchTimerInterval);
        window.clearTimeout(fetchTimerInterval);
        setFetchTimerInterval(0);
      }
    };
  }, [props.bungie_name, props.modes, props.season]);

  if (memberReport) {
    switch (typeof memberReport) {
      case 'object':
        // this conversion is fine to do because we know we are already working with an object type
        const data = memberReport as unknown as DestinyMemberReport;
        return (
          <div
            className={
              (isLoadingData ? 'animate-pulse' : '') +
              ' member-report relative top-0'
            }
          >
            <H3 className="text-yellow-400 text-ellipsis max-w-full whitespace-nowrap overflow-hidden">
              {data.display_name_global}
            </H3>
            <H5>
              <span className="mr-2">{'[' + data.snapshot_range + ']'}</span>
              <span>
                {data.member.clan ? '[' + data.member.clan.name + ']' : ''}
              </span>
              <br />
              <span className="text-sm">
                {data.search.modes ? 'Modes: [' + data.search.modes + ']' : ''}
              </span>
            </H5>
            <H5></H5>
            <p className="mt-4">
              <span className="mr-2">Last seen:</span>
              <span>
                {new Date(data.last_played_at * 1000).toLocaleString('default')}
              </span>
            </p>
            <Divider />
            {renderOverall(memberReport, props.modes || [])}
          </div>
        );
      default:
        return <>Loading</>;
    }
  } else {
    return <>Querying! Please wait. This can take some time</>;
  }
};

export default DestinyMemberReportComponent;
