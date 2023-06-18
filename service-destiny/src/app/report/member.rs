use crate::app::instance;
use crate::app::state::CacheItem;
use crate::bungie::enums::DestinyActivityModeType;
use crate::database::activity::ActivityInstanceResult;
use crate::database::activity_stats::StatFilter;
use crate::database::member::MemberResult;
use crate::jobs::task;
use crate::routes::responses::{MemberResponse, MemberTitle};
use crate::{
    app::{self, state::AppState},
    database,
};
use levelcrush::bigdecimal::ToPrimitive;
use levelcrush::cache::CacheDuration;
use levelcrush::cache::CacheValue;
use levelcrush::chrono::{self, Datelike, TimeZone, Utc};
use levelcrush::tracing;
use levelcrush::types::destiny::InstanceId;
use levelcrush::types::destiny::MembershipId;
use levelcrush::types::UnixTimestamp;
use levelcrush::util::unix_timestamp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use ts_rs::TS;

use levelcrush::tokio;

const CACHE_DURATION_REPORT: CacheDuration = CacheDuration::HalfDay;
const VERSION_MEMBER_REPORT_CURRENT: u8 = 0;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberReportStats {
    #[ts(type = "number")]
    pub kills: u64,
    #[ts(type = "number")]
    pub deaths: u64,
    #[ts(type = "number")]
    pub assists: u64,
    #[ts(type = "number")]
    pub victories: u64,
    #[ts(type = "number")]
    pub defeats: u64,
    #[ts(type = "number")]
    pub activities: u64,
    #[ts(type = "number")]
    pub activity_completions: u64,

    #[ts(type = "number")]
    pub activities_completed_start_to_finish: u64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MemberConstructionReport {
    pub version: u8,
    pub stats_pve: MemberReportStats,
    pub stats_pvp: MemberReportStats,
    pub stats_gambit: MemberReportStats,
    pub stats_private_matches: MemberReportStats,
    pub stats_reckoning: MemberReportStats,
    pub display_name_global: String,
    pub membership_id: i64,
    pub activity_attempts: u64,
    pub activity_attempts_with_clan: u64,
    pub activity_completions: u64,
    pub activity_modes: HashMap<i32, u64>,
    pub instance_timestamps: HashSet<(InstanceId, UnixTimestamp)>,
    pub instance_members_profiles: HashMap<MembershipId, MemberResult>,
    pub instance_members: HashMap<MembershipId, u64>,
    pub clan_members: HashMap<String, u64>,
    pub non_clan_members: HashMap<String, u64>,
    pub last_played_at: UnixTimestamp,
    pub activity_definitions: Vec<ActivityInstanceResult>,
}

#[derive(serde::Serialize, Clone, Debug, Default, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberReportActivityMode {
    pub mode: String,

    #[ts(type = "number")]
    pub count: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberReportActivity {
    #[ts(type = "number")]
    pub attempts: u64,

    #[ts(type = "number")]
    pub completions: u64,

    pub name: String,
    pub description: String,
}

#[derive(serde::Serialize, Clone, Debug, Default, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberReportFireteamMember {
    pub display_name: String,

    #[ts(type = "number")]
    pub activities: u64,
}

#[derive(serde::Serialize, Clone, Debug, Default, TS)]
#[ts(export, export_to = "../lib-levelcrush-ts/src/service-destiny/")]
pub struct MemberReport {
    pub version: u8,

    #[ts(type = "number")]
    pub membership_id: i64,

    pub snapshot_range: String,

    pub display_name_global: String,

    #[ts(type = "number")]
    pub last_played_at: UnixTimestamp,

    #[ts(type = "Record<number,number>")]
    pub activity_timestamps: HashMap<InstanceId, UnixTimestamp>,

    #[ts(type = "number")]
    pub activity_attempts: u64,

    #[ts(type = "number")]
    pub activity_attempts_with_clan: u64,

    #[ts(type = "number")]
    pub activity_completions: u64,

    pub stats_pve: MemberReportStats,
    pub stats_pvp: MemberReportStats,
    pub stats_gambit: MemberReportStats,
    pub stats_private_matches: MemberReportStats,
    pub stats_reckoning: MemberReportStats,
    pub top_activity_modes: Vec<MemberReportActivityMode>,
    pub top_activities: Vec<MemberReportActivity>,
    pub activity_map: HashMap<String, MemberReportActivity>,
    pub frequent_clan_members: Vec<MemberReportFireteamMember>,
    pub frequent_non_clan_members: Vec<MemberReportFireteamMember>,

    #[ts(type = "number")]
    pub total_clan_members: u64,

    #[ts(type = "number")]
    pub total_non_clan_members: u64,
    pub titles: Vec<MemberTitle>,
    pub member: MemberResponse, // added later on,
}

const CACHE_KEY_LIFETIME: &str = "member_report||lifetime||";
const CACHE_KEY_SEASON: &str = "member_report||season||";

async fn coalesce_display_name(member: &mut MemberReportFireteamMember, state: &mut AppState) {
    let membership_id_parse = member.display_name.parse::<i64>();
    if let Ok(membership_id) = membership_id_parse {
        task::profile(membership_id, 0, &state).await;
        let result = database::member::get(membership_id, &state.database).await;
        if let Some(result) = result {
            member.display_name = if result.display_name_global == "#0000" {
                result.display_name
            } else {
                result.display_name_global
            };
        }
    }
}

fn merge_stats(a: &mut MemberReportStats, b: &MemberReportStats) {
    a.assists += b.assists;
    a.deaths += b.deaths;
    a.defeats += b.defeats;
    a.kills += b.kills;
    a.victories += b.victories;
    a.activity_completions += b.activity_completions;
    a.activities += b.activities;
    a.activities_completed_start_to_finish += b.activities_completed_start_to_finish;
}

async fn merge_reports(
    bungie_name: String,
    reports: Vec<MemberConstructionReport>,
    snapshot_range: String,
    state: &mut AppState,
) -> MemberReport {
    // now merge
    let mut activity_attempts = 0;
    let mut activity_attempts_with_clan = 0;
    let mut activity_completions = 0;
    let mut clan_members = HashMap::new();
    let mut non_clan_members = HashMap::new();
    let mut activity_modes = HashMap::new();
    let mut last_played_at = 0;

    let mut stats_pve = MemberReportStats::default();
    let mut stats_pvp = MemberReportStats::default();
    let mut stats_gambit = MemberReportStats::default();
    let mut stats_private_matches = MemberReportStats::default();
    let mut stats_reckoning = MemberReportStats::default();
    let mut membership_id = 0;
    let mut instance_timestamps = HashSet::new();

    let mut activity_map = HashMap::new();

    for report in reports.into_iter() {
        activity_attempts += report.activity_attempts;
        activity_attempts_with_clan += report.activity_attempts_with_clan;
        activity_completions += report.activity_completions;
        last_played_at = report.last_played_at; // this should always be the same between all reports so just pull from whatever is available
        membership_id = report.membership_id;

        // get activity map
        for activity_def in report.activity_definitions.into_iter() {
            let entry = activity_map.entry(activity_def.activity_hash.to_string());
            match entry {
                Entry::Vacant(_) => {
                    entry.or_insert(MemberReportActivity {
                        name: activity_def.activity_name,
                        completions: activity_def.total_completed.to_u64().unwrap_or_default(),
                        attempts: activity_def.total.to_u64().unwrap_or_default(),
                        description: activity_def.activity_description,
                    });
                }
                Entry::Occupied(_) => {
                    entry.and_modify(|v| {
                        v.attempts += activity_def.total.to_u64().unwrap_or_default();
                        v.completions += activity_def.total_completed.to_u64().unwrap_or_default()
                    });
                }
            }
        }

        // group our timestamps together
        instance_timestamps.extend(report.instance_timestamps.into_iter());

        merge_stats(&mut stats_pve, &report.stats_pve);
        merge_stats(&mut stats_pvp, &report.stats_pvp);
        merge_stats(&mut stats_gambit, &report.stats_gambit);
        merge_stats(&mut stats_private_matches, &report.stats_private_matches);
        merge_stats(&mut stats_reckoning, &report.stats_reckoning);

        for (member_bungie_name, common_instances) in report.clan_members.into_iter() {
            clan_members
                .entry(member_bungie_name)
                .and_modify(|v| *v += common_instances)
                .or_insert(common_instances);
        }

        for (member_bungie_name, common_instances) in report.non_clan_members.into_iter() {
            non_clan_members
                .entry(member_bungie_name)
                .and_modify(|v| *v += common_instances)
                .or_insert(common_instances);
        }

        for (activity_mode, attempts) in report.activity_modes.into_iter() {
            activity_modes
                .entry(activity_mode)
                .and_modify(|v| *v += attempts)
                .or_insert(attempts);
        }
    }

    // now flatten
    let mut clan_members = clan_members
        .into_iter()
        .map(|(member_name, common_instances)| MemberReportFireteamMember {
            display_name: member_name,
            activities: common_instances,
        })
        .collect::<Vec<MemberReportFireteamMember>>();

    // sort by descending order by amount of activities completed
    clan_members.sort_by(|a, b| b.activities.cmp(&a.activities));

    // flatten
    let mut non_clan_members = non_clan_members
        .into_iter()
        .map(|(member_name, common_instances)| MemberReportFireteamMember {
            display_name: member_name,
            activities: common_instances,
        })
        .collect::<Vec<MemberReportFireteamMember>>();

    // sort by descending order
    non_clan_members.sort_by(|a, b| b.activities.cmp(&a.activities));

    let total_clan_members = clan_members.len() as u64;
    let total_non_clan_members = non_clan_members.len() as u64;

    // fill in missing info now
    clan_members = clan_members.into_iter().take(10).collect();

    for member in clan_members.iter_mut() {
        coalesce_display_name(member, state).await;
    }

    non_clan_members = non_clan_members.into_iter().take(10).collect();
    for member in non_clan_members.iter_mut() {
        coalesce_display_name(member, state).await;
    }

    // flatten
    let mut activity_modes = activity_modes
        .into_iter()
        .map(|(mode, count)| {
            let mode: DestinyActivityModeType = mode.into();
            MemberReportActivityMode {
                mode: mode.as_str().to_string(),
                count,
            }
        })
        .collect::<Vec<MemberReportActivityMode>>();

    // sort by descending order
    activity_modes.sort_by(|a, b| b.count.cmp(&a.count));

    tracing::info!(
        "Generating activity time periods for: {} at id {}",
        bungie_name,
        membership_id
    );

    tracing::info!("Getting titles tied to user: {} at id {}", bungie_name, membership_id);
    let titles_db = database::triumph::member_titles(membership_id, &state.database).await;
    let titles = titles_db.into_iter().map(MemberTitle::from_db).collect();

    tracing::info!("Sorting activities for {} at {}", bungie_name, membership_id);
    let mut activities = Vec::new();
    for (hash, activity) in activity_map.iter() {
        activities.push(activity.clone());
    }

    activities.sort_by(|a, b| b.attempts.cmp(&a.attempts));

    tracing::info!(
        "Getting member response for user: {} at id {}",
        bungie_name,
        membership_id
    );
    let member = app::member::profile(membership_id, state).await;

    tracing::info!("Done for: {}", bungie_name);

    MemberReport {
        version: VERSION_MEMBER_REPORT_CURRENT,
        snapshot_range,
        activity_timestamps: HashMap::from_iter(instance_timestamps.into_iter()),
        membership_id,
        display_name_global: bungie_name,
        titles,
        last_played_at,
        activity_attempts,
        activity_attempts_with_clan,
        activity_completions,
        top_activity_modes: activity_modes,
        frequent_clan_members: clan_members,
        frequent_non_clan_members: non_clan_members,
        total_clan_members,
        total_non_clan_members,
        stats_pve,
        stats_pvp,
        stats_gambit,
        stats_private_matches,
        stats_reckoning,
        member: MemberResponse::from_db(member.unwrap_or_default()),
        top_activities: activities,
        activity_map,
    }
}

pub async fn season<T: Into<String>>(
    bungie_name: T,
    modes: &[i32],
    season: i32,
    priority: bool,
    state: &mut AppState,
) -> (UnixTimestamp, Option<MemberReport>) {
    let bungie_name = bungie_name.into();
    let modes = modes.to_vec();

    let mut state = state.clone();
    let mode_str = modes.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(",");
    let cache_key = format!("{}{}||{}||{}", CACHE_KEY_SEASON, bungie_name, season, mode_str);
    let report = match state.cache.access(&cache_key).await {
        Some(CacheItem::MemberReport(record)) => Some(record),
        _ => None,
    };

    if let Some(report) = report {
        tracing::info!("Report was fully cached for: {} || {}", bungie_name, season);
        (unix_timestamp(), Some(*report))
    } else {
        let long_task_key = cache_key.clone();
        let starting_timestamp = match state.task_running.access(&long_task_key).await {
            Some(starting_timestamp) => starting_timestamp,
            _ => 0,
        };

        if starting_timestamp > 0 {
            tracing::info!("Task running for: {} || {}", bungie_name, season);
            (starting_timestamp, None)
        } else {
            let mut thread_app_state = state.clone();
            let bungie_name = bungie_name.clone();
            let long_task_key_cpy = long_task_key.clone();

            let target_manager = if priority { state.priority_tasks } else { state.tasks };
            target_manager
                .queue(Box::new(move || {
                    Box::pin(async move {
                        let max_snapshotable_season = std::env::var("REPORT_SEASON_MAX")
                            .unwrap_or_default()
                            .parse::<i32>()
                            .unwrap_or(20);

                        let season = database::seasons::get(season, &thread_app_state.database).await;
                        let (season_start, season_end, season_number) = match season {
                            Some(record) => (record.starts_at, record.ends_at, record.number),
                            _ => (0, 0, -1),
                        };

                        let end_timestamp: u64 = if season_number > max_snapshotable_season {
                            unix_timestamp()
                        } else {
                            season_end
                        };

                        if season_number > max_snapshotable_season {
                            tracing::info!(
                                "Target {} snapshot is being ran from season {} start to unix_timestamp()",
                                bungie_name,
                                season_number,
                            );
                        } else {
                            tracing::info!(
                                "Target {} snapshot is being ran from season {} start to season end",
                                bungie_name,
                                season_number,
                            );
                        }

                        let start_timestamp = season_start;
                        let mut reports = Vec::new();

                        let start_datetime = chrono::DateTime::<chrono::Utc>::from_utc(
                            chrono::NaiveDateTime::from_timestamp_opt(start_timestamp as i64, 0).unwrap(),
                            chrono::Utc,
                        );
                        let end_datetime = chrono::DateTime::<chrono::Utc>::from_utc(
                            chrono::NaiveDateTime::from_timestamp_opt(end_timestamp as i64, 0).unwrap(),
                            chrono::Utc,
                        );

                        // fetch reports
                        // **currently** at the moment since we have predefined time ranges we will only get one report
                        // but we will keep the same merge formatting that lifetime does just in case
                        tracing::info!("Date Range: {} to {}", start_datetime, end_datetime);

                        let report = construct(
                            bungie_name.clone(),
                            start_timestamp,
                            end_timestamp,
                            &modes,
                            &mut thread_app_state,
                        )
                        .await;

                        let mut display_name_global = String::new();
                        if let Some(report) = report {
                            display_name_global = report.display_name_global.clone();
                            reports.push(report);
                        }

                        // stop getting reports.
                        let snapshot_range = format!("Season {}", season_number);
                        let report =
                            merge_reports(display_name_global, reports, snapshot_range, &mut thread_app_state).await;

                        thread_app_state
                            .cache
                            .write(
                                cache_key,
                                CacheValue::with_duration(
                                    CacheItem::MemberReport(Box::new(report.clone())),
                                    CACHE_DURATION_REPORT,
                                    CACHE_DURATION_REPORT,
                                ),
                            )
                            .await;

                        // remove it from our running task track
                        thread_app_state.task_running.delete(&long_task_key).await;
                    })
                }))
                .await;

            state
                .task_running
                .write(
                    long_task_key_cpy,
                    CacheValue::with_duration(unix_timestamp(), CacheDuration::Persistant, CacheDuration::Persistant),
                )
                .await;

            (unix_timestamp(), None)
        }
    }
}

/// generates a lifetime report of activities/completions/etc
pub async fn lifetime<T: Into<String>>(
    bungie_name: T,
    modes: &[i32],
    priority: bool,
    state: &mut AppState,
) -> (u64, Option<MemberReport>) {
    let bungie_name = bungie_name.into();
    let modes = modes.to_vec();
    let mut thread_app_state = state.clone();
    let mut state = state.clone();
    let modes_str = modes.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",");
    let cache_key = format!("{}{}||{}", CACHE_KEY_LIFETIME, bungie_name, modes_str);
    let report = match thread_app_state.cache.access(&cache_key).await {
        Some(CacheItem::MemberReport(record)) => Some(record),
        _ => None,
    };

    if let Some(report) = report {
        tracing::info!("Report was fully cached locally for {} lifetime", bungie_name);
        (unix_timestamp(), Some(*report))
    } else {
        let long_task_key = cache_key.clone();
        let starting_timestamp = match thread_app_state.task_running.access(&long_task_key).await {
            Some(starting_time) => starting_time,
            _ => 0,
        };

        if starting_timestamp > 0 {
            tracing::info!("Task running for: {} lifetime", bungie_name);
            (starting_timestamp, None)
        } else {
            let long_task_key_cpy = long_task_key.clone();
            let target_manager = if priority { state.priority_tasks } else { state.tasks };
            target_manager
                .queue(Box::new(move || {
                    Box::pin(async move {
                        let mut display_name_global = String::new();
                        let mut reports = Vec::new();
                        let mut timestamps = Vec::new();

                        let current_datetime = chrono::Utc::now();
                        let current_month_start_str = format!(
                            "{}-{:0>2}-{:0>2} 00:00:00",
                            current_datetime.year(),
                            current_datetime.month(),
                            current_datetime.day()
                        );

                        let destiny2_launch_month_start = chrono::Utc
                            .datetime_from_str("2017-09-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                            .unwrap_or_default();

                        let current_month_start = chrono::Utc
                            .datetime_from_str(&current_month_start_str, "%Y-%m-%d %H:%M:%S")
                            .unwrap_or_default();

                        const DURATION_DAYS: i64 = 30;
                        let time_diff = current_month_start.signed_duration_since(destiny2_launch_month_start);
                        let total_months = (time_diff.num_days() as f64 / DURATION_DAYS as f64).ceil() as i64;
                        let mut previous_datetime = current_datetime;
                        for month in 0..(total_months + 1) {
                            let new_datetime = current_month_start - chrono::Duration::days(month * DURATION_DAYS);
                            let month_str = format!("{}-{:0>2}-01 00:00:00", new_datetime.year(), new_datetime.month());
                            let target_month_start = chrono::Utc
                                .datetime_from_str(&month_str, "%Y-%m-%d %H:%M:%S")
                                .unwrap_or_default();

                            timestamps.push((target_month_start, previous_datetime));
                            previous_datetime = target_month_start;
                        }

                        for (start_datetime, end_datetime) in timestamps.into_iter() {
                            tracing::info!("Date Range: {} to {}", start_datetime, end_datetime);
                            let report = construct(
                                bungie_name.clone(),
                                start_datetime.timestamp() as u64,
                                end_datetime.timestamp() as u64,
                                &modes,
                                &mut thread_app_state,
                            )
                            .await;
                            if let Some(report) = report {
                                display_name_global = report.display_name_global.clone();
                                reports.push(report);
                            }
                        }

                        let report = merge_reports(
                            display_name_global,
                            reports,
                            "Lifetime".to_string(),
                            &mut thread_app_state,
                        )
                        .await;

                        thread_app_state
                            .cache
                            .write(
                                cache_key,
                                CacheValue::with_duration(
                                    CacheItem::MemberReport(Box::new(report.clone())),
                                    CACHE_DURATION_REPORT,
                                    CACHE_DURATION_REPORT,
                                ),
                            )
                            .await;

                        // remove it from our running task track
                        thread_app_state.task_running.delete(&long_task_key).await;
                    })
                }))
                .await;

            state
                .task_running
                .write(
                    long_task_key_cpy,
                    CacheValue::with_duration(unix_timestamp(), CacheDuration::Persistant, CacheDuration::Persistant),
                )
                .await;

            (unix_timestamp(), None)
        }
    }
}

async fn get_stats(membership_id: MembershipId, instance_ids: &[InstanceId], state: &AppState) -> MemberReportStats {
    //
    let (
        instances_completed,
        instances_kills,
        instances_assists,
        instances_deaths,
        instances_victory,
        instances_defeat,
        instances_data,
    ) = tokio::join!(
        database::activity_stats::get_instances(
            "completed",
            membership_id,
            instance_ids,
            StatFilter::Value(1f64),
            &state.database,
        ),
        database::activity_stats::get_instances(
            "kills",
            membership_id,
            instance_ids,
            StatFilter::None,
            &state.database,
        ),
        database::activity_stats::get_instances(
            "assists",
            membership_id,
            instance_ids,
            StatFilter::None,
            &state.database,
        ),
        database::activity_stats::get_instances(
            "deaths",
            membership_id,
            instance_ids,
            StatFilter::None,
            &state.database,
        ),
        database::activity_stats::get_instances(
            "standing",
            membership_id,
            instance_ids,
            StatFilter::ValueDisplay("Victory".to_string()),
            &state.database,
        ),
        database::activity_stats::get_instances(
            "standing",
            membership_id,
            instance_ids,
            StatFilter::ValueDisplay("Defeat".to_string()),
            &state.database,
        ),
        database::instance::multi_get(instance_ids, &state.database),
    );

    let mut unique_instances = HashSet::new();
    let mut complete_full_instances = Vec::new();
    for instance in instances_completed.iter() {
        unique_instances.insert(instance.instance_id);
    }

    for instance in instances_data.iter() {
        if instance.started_from_beginning == 1 && instance.completed == 1 {
            complete_full_instances.push(instance.instance_id);
        }
    }

    let activity_completions = unique_instances.len() as u64;
    let victories = instances_victory.len() as u64;
    let defeats = instances_defeat.len() as u64;
    let kills = {
        let mut total = 0u64;
        for data in instances_kills.into_iter() {
            total += data.value.ceil() as u64;
        }
        total
    };

    let deaths = {
        let mut total = 0u64;
        for data in instances_deaths.into_iter() {
            total += data.value.ceil() as u64;
        }
        total
    };
    let assists = {
        let mut total = 0u64;
        for data in instances_assists.into_iter() {
            total += data.value.ceil() as u64;
        }
        total
    };

    let activities = instance_ids.len() as u64;

    MemberReportStats {
        kills,
        deaths,
        assists,
        victories,
        defeats,
        activity_completions,
        activities,
        activities_completed_start_to_finish: complete_full_instances.len() as u64,
    }
}

async fn construct<T: Into<String>>(
    bungie_name: T,
    timestamp_start: u64,
    timestamp_end: u64,
    modes: &[i32],
    state: &mut AppState,
) -> Option<MemberConstructionReport> {
    let bungie_name = bungie_name.into();

    // we may of actually been feed a membership id potentially
    let membership_id = bungie_name.parse::<i64>().unwrap_or_default();

    // get our member, but if we don't have one, just return None right now and exit
    let member = if membership_id > 0 {
        app::member::profile(membership_id, state).await
    } else {
        app::member::by_bungie_name(&bungie_name, state).await
    };

    if member.is_none() {
        return None;
    }

    // extract out. Above here we would of already done our own check to see if its populated and returned out
    let member = member.expect("Member should be populate");

    // check db for snapshot first
    let modes_str = modes.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",");
    let snapshot_key = format!(
        "{}_{}_{}||{}",
        member.membership_id, timestamp_start, timestamp_end, modes_str
    );
    tracing::info!("Checking {} if snapshotted", snapshot_key);
    let snapshot = database::member::get_snapshot(
        member.membership_id,
        &snapshot_key,
        VERSION_MEMBER_REPORT_CURRENT,
        &state.database,
    )
    .await;
    if let Some(snapshot) = snapshot {
        let data = serde_json::from_str::<'_, MemberConstructionReport>(&snapshot.data);
        if let Ok(data) = data {
            tracing::info!("Returning: {}", snapshot_key);
            return Some(data);
        } else {
            let err = data.err().unwrap();
            tracing::error!("An error occurred while deserializing the snapshot: {}", err);
        }
    }

    tracing::info!("Still here: {}", snapshot_key);

    let activities = database::activity_history::member(
        member.membership_id,
        timestamp_start,
        timestamp_end,
        modes,
        &state.database,
    )
    .await;

    let mut unique_instances = HashSet::new();
    for activity in activities.iter() {
        unique_instances.insert(activity.instance_id);
    }

    let activity_attempts = unique_instances.len() as u64;
    let mut activity_modes = HashMap::new();
    let mut membership_activities = HashMap::new();

    // instance ids
    let mut instance_timestamps = HashSet::new();
    let mut instance_ids = HashSet::new();
    let mut pve_instance_ids = HashSet::new();
    let mut pvp_instance_ids = HashSet::new();
    let mut gambit_instance_ids = HashSet::new();
    let mut private_matches_ids = HashSet::new();
    let mut reckoning_instance_ids = HashSet::new();
    let mut unknown_instance_ids = HashSet::new();
    for activity in activities.iter() {
        instance_ids.insert(activity.instance_id);

        let modes = activity.modes.split(',');
        let mut found = false;
        'mode_check: for mode in modes.into_iter() {
            if mode == "5" {
                //  per bungie doc, if this is included then this is a PvP activity
                pvp_instance_ids.insert(activity.instance_id);
                found = true;
                break 'mode_check;
            } else if mode == "7" {
                // per bungie doc if this is included then this is a PvE activity
                pve_instance_ids.insert(activity.instance_id);
                found = true;
                break 'mode_check;
            } else if mode == "64" {
                // per bungie doc  if this is included then this is a PvE comp activity. Aka Gambit
                gambit_instance_ids.insert(activity.instance_id);
                found = true;
                break 'mode_check;
            } else if mode == "32" {
                private_matches_ids.insert(activity.instance_id);
                found = true;
                break 'mode_check;
            } else if mode == "76" {
                reckoning_instance_ids.insert(activity.instance_id);
                found = true;
                break 'mode_check;
            }
        }

        if !found {
            unknown_instance_ids.insert(activity.instance_id);
        }

        // track how many times
        membership_activities
            .entry(activity.instance_id)
            .or_insert_with(HashMap::new);

        instance_timestamps.insert((activity.instance_id, activity.occurred_at));

        if let Some(instance) = membership_activities.get_mut(&activity.instance_id) {
            if !instance.contains_key(&activity.membership_id) {
                activity_modes.entry(activity.mode).and_modify(|v| *v += 1).or_insert(1);
            }

            // track how many times this membership appears. They can appear multiple times due to character joining
            instance
                .entry(activity.membership_id)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    if !unknown_instance_ids.is_empty() {
        tracing::warn!("Unknown instance ids: {:?}", unknown_instance_ids);
    }

    let instance_ids = Vec::from_iter(instance_ids.into_iter());

    let pve_instance_ids = Vec::from_iter(pve_instance_ids.into_iter());
    let pvp_instance_ids = Vec::from_iter(pvp_instance_ids.into_iter());
    let gambit_instance_ids = Vec::from_iter(gambit_instance_ids.into_iter());
    let private_matches_ids = Vec::from_iter(private_matches_ids.into_iter());
    let reckoning_instance_ids = Vec::from_iter(reckoning_instance_ids.into_iter());

    let instances_completed = database::activity_stats::get_instances(
        // this is from the member perspective of what was completed
        "completed",
        member.membership_id,
        &instance_ids,
        StatFilter::Value(1f64),
        &state.database,
    )
    .await;

    let mut unique_instances_completed = HashSet::new();
    for instance in instances_completed.iter() {
        unique_instances_completed.insert(instance.instance_id);
    }

    let activity_completions = unique_instances_completed.len() as u64;
    let (
        stats_pve,
        stats_pvp,
        stats_gambit,
        stats_private_matches,
        stats_reckoning,
        activity_definitions,
        instance_members,
    ) = tokio::join!(
        get_stats(member.membership_id, &pve_instance_ids, state),
        get_stats(member.membership_id, &pvp_instance_ids, state),
        get_stats(member.membership_id, &gambit_instance_ids, state),
        get_stats(member.membership_id, &private_matches_ids, state),
        get_stats(member.membership_id, &reckoning_instance_ids, state),
        database::activity::from_instances(&instance_ids, &state.database),
        database::instance::multi_get_members(&instance_ids, &state.database)
    );

    // analyze instances
    let mut membership_ids = HashSet::new();
    for instance_member in instance_members.iter() {
        membership_ids.insert(instance_member.membership_id);
        //   character_ids.insert(instance_member.character_id);
    }

    let membership_ids = Vec::from_iter(membership_ids.into_iter());
    //  let character_ids = Vec::from_iter(character_ids.into_iter());

    let members = database::member::multi_get(&membership_ids, &state.database).await;
    let members =
        HashMap::<i64, MemberResult>::from_iter(members.into_iter().map(|result| (result.membership_id, result)));

    // scan through instance members and determine which instances we did with clan members
    let mut instances_with_clan_members = HashMap::new();
    for instance_member in instance_members.iter() {
        let is_clan_member = match members.get(&instance_member.membership_id) {
            Some(result) => result.clan_is_network == 1,
            _ => false,
        };

        if is_clan_member && instance_member.membership_id != member.membership_id {
            instances_with_clan_members
                .entry(instance_member.instance_id)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    // store how many attempts we have done with clan members
    let activity_attempts_with_clan = instances_with_clan_members.len() as u64;

    // loop through instance_members and determine how many times someone has appeared
    let mut instance_member_counts = HashMap::new();
    let mut mapped_instance_members = HashMap::new();
    for instance_member in instance_members.iter() {
        mapped_instance_members
            .entry(instance_member.instance_id)
            .or_insert_with(HashMap::new);

        if member.membership_id != instance_member.membership_id {
            if let Some(instance) = mapped_instance_members.get_mut(&instance_member.instance_id) {
                // only add if this is the fist time the member has connected to the instance
                // it is possible for  a member to appear multiple times due to characters used which can skew our desired data
                if !instance.contains_key(&instance_member.membership_id) {
                    // track this member count
                    instance_member_counts
                        .entry(instance_member.membership_id)
                        .and_modify(|v| *v += 1)
                        .or_insert(1u64);
                }

                instance
                    .entry(instance_member.membership_id)
                    .and_modify(|v| *v += 1)
                    .or_insert(1u64);
            }
        }
    }

    let mut clan_members = HashMap::new();
    let mut non_clan_members = HashMap::new();
    let mut unknown_members = 0;

    for (membership_id, common_instances) in instance_member_counts.iter() {
        let member = members.get(membership_id);
        let (display_name_global, in_network) = match member {
            Some(result) => {
                if result.display_name_global == "#0000" {
                    (membership_id.to_string(), result.clan_is_network == 1)
                } else {
                    (result.display_name_global.clone(), result.clan_is_network == 1)
                }
            }
            _ => {
                //let local_name = format!("NotInSystem#{:0>4}", unknown_members);
                let local_name = membership_id.to_string();
                // import
                unknown_members += 1;
                (local_name, false)
            }
        };

        let common_instances = *common_instances;
        if in_network {
            clan_members
                .entry(display_name_global)
                .and_modify(|v| *v += common_instances)
                .or_insert(common_instances);
        } else {
            non_clan_members
                .entry(display_name_global)
                .and_modify(|v| *v += common_instances)
                .or_insert(common_instances);
        }
    }

    let construct_report = MemberConstructionReport {
        version: VERSION_MEMBER_REPORT_CURRENT,
        instance_timestamps,
        display_name_global: member.display_name_global,
        membership_id: member.membership_id,
        last_played_at: member.last_played_at,
        activity_attempts,
        activity_attempts_with_clan,
        activity_completions,
        activity_modes,
        instance_members_profiles: members,
        instance_members: instance_member_counts,
        clan_members,
        non_clan_members,
        stats_pve,
        stats_pvp,
        stats_gambit,
        stats_private_matches,
        stats_reckoning,
        activity_definitions,
    };

    let serialize_result = serde_json::to_string(&construct_report);
    if let Ok(result) = serialize_result {
        tracing::info!("Serialized snapshot...storing into {} ", snapshot_key);
        database::member::write_snapshot(
            member.membership_id,
            &snapshot_key,
            VERSION_MEMBER_REPORT_CURRENT,
            result,
            &state.database,
        )
        .await;
    }
    Some(construct_report)
}
