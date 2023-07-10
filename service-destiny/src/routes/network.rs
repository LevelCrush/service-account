use std::collections::HashMap;

use super::{
    member::ReportQueries,
    responses::{
        APIClanInfoMultiResponse, APINetworkRosterResponse, ClanInformation, MemberResponse,
        NetworkActivityClanBreakdown, ReportOutput,
    },
};
use crate::app::state::AppState;
use crate::{app, database};
use axum::{extract::State, routing::get, Json, Router};
use levelcrush::{
    axum::{
        self,
        extract::{Path, Query},
    },
    chrono::{self, TimeZone},
    server::APIResponse,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(network_clans))
        .route("/info", get(network_clans))
        .route("/roster", get(network_clans_roster))
        .route("/report", get(network_lifetime_report))
        .route("/report/season/:season", get(network_season_report))
        .route("/report/lifetime", get(network_lifetime_report))
        .route("/report/activity/lifetime", get(network_breakdown_lifetime))
        .route("/report/activity/season/:season", get(network_breakdown_season))
}

async fn network_breakdown_season(
    Path(season): Path<String>,
    Query(report_queries): Query<ReportQueries>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<HashMap<String, NetworkActivityClanBreakdown>>> {
    let mut response = APIResponse::new();

    let modes = if let Some(input_modes) = report_queries.modes {
        input_modes
            .split(',')
            .map(|v| v.parse::<i32>().unwrap_or_default())
            .collect::<Vec<i32>>()
    } else {
        Vec::new()
    };

    let season_input_number = season.parse::<i32>().unwrap_or_default();

    let season = database::seasons::get(season_input_number, &state.database).await;
    let (season_start, season_end, season_number) = match season {
        Some(record) => (record.starts_at, record.ends_at, record.number),
        _ => (0, 0, -1),
    };

    // fetch from db
    let network_breakdown = app::clan::network_breakdown(&modes, season_start, season_end, &mut state).await;

    // convert to our response type
    let network_breakdown = network_breakdown
        .into_iter()
        .map(|(group_id, r)| (group_id.to_string(), NetworkActivityClanBreakdown::from_db(r)))
        .collect::<HashMap<String, NetworkActivityClanBreakdown>>();

    response.data(Some(network_breakdown));

    response.complete();
    Json(response)
}

async fn network_breakdown_lifetime(
    Query(report_queries): Query<ReportQueries>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<HashMap<String, NetworkActivityClanBreakdown>>> {
    let mut response = APIResponse::new();

    let modes = if let Some(input_modes) = report_queries.modes {
        input_modes
            .split(',')
            .map(|v| v.parse::<i32>().unwrap_or_default())
            .collect::<Vec<i32>>()
    } else {
        Vec::new()
    };

    let destiny2_launch_month_start = chrono::Utc
        .datetime_from_str("2017-09-01 00:00:00", "%Y-%m-%d %H:%M:%S")
        .unwrap_or_default();
    let current_datetime = chrono::Utc::now();

    // fetch from db
    let network_breakdown = app::clan::network_breakdown(
        &modes,
        destiny2_launch_month_start.timestamp() as u64,
        current_datetime.timestamp() as u64,
        &mut state,
    )
    .await;

    // convert to our response type
    let network_breakdown = network_breakdown
        .into_iter()
        .map(|(group_id, r)| (group_id.to_string(), NetworkActivityClanBreakdown::from_db(r)))
        .collect::<HashMap<String, NetworkActivityClanBreakdown>>();

    response.data(Some(network_breakdown));

    response.complete();
    Json(response)
}

async fn network_lifetime_report(
    Query(report_queries): Query<ReportQueries>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<HashMap<String, ReportOutput>>> {
    let mut response = APIResponse::new();

    let modes = if let Some(input_modes) = report_queries.modes {
        input_modes
            .split(',')
            .map(|v| v.parse::<i32>().unwrap_or_default())
            .collect::<Vec<i32>>()
    } else {
        Vec::new()
    };

    let network_members = app::clan::network_roster(&mut state).await;
    let mut network_report_map = HashMap::new();
    for member in network_members.iter() {
        let (task_started, report) =
            app::report::member::lifetime(member.membership_id.to_string(), &modes, true, &mut state).await;

        if report.is_some() {
            network_report_map.insert(
                member.membership_id.to_string(),
                ReportOutput::Report(Box::new(report.expect("Report should of been here"))),
            );
        } else {
            network_report_map.insert(
                member.membership_id.to_string(),
                ReportOutput::TaskRunning(task_started),
            );
        }
    }

    response.data(Some(network_report_map));
    response.complete();
    Json(response)
}

async fn network_season_report(
    Path(season): Path<String>,
    Query(report_queries): Query<ReportQueries>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<HashMap<String, ReportOutput>>> {
    let mut response = APIResponse::new();

    let modes = if let Some(input_modes) = report_queries.modes {
        input_modes
            .split(',')
            .map(|v| v.parse::<i32>().unwrap_or_default())
            .collect::<Vec<i32>>()
    } else {
        Vec::new()
    };

    let network_members = app::clan::network_roster(&mut state).await;
    let mut network_report_map = HashMap::new();
    for member in network_members.iter() {
        let (task_started, report) = app::report::member::season(
            member.membership_id.to_string(),
            &modes,
            season.parse::<i32>().unwrap_or_default(),
            true,
            &mut state,
        )
        .await;

        if report.is_some() {
            network_report_map.insert(
                member.membership_id.to_string(),
                ReportOutput::Report(Box::new(report.expect("Report should of been here"))),
            );
        } else {
            network_report_map.insert(
                member.membership_id.to_string(),
                ReportOutput::TaskRunning(task_started),
            );
        }
    }

    response.data(Some(network_report_map));
    response.complete();
    Json(response)
}

async fn network_clans(State(mut state): State<AppState>) -> Json<APIClanInfoMultiResponse> {
    let mut response = APIClanInfoMultiResponse::new();

    let network_clans = app::clan::network(&mut state).await;
    let mut network_clans_info = network_clans
        .into_iter()
        .map(ClanInformation::from_db)
        .collect::<Vec<ClanInformation>>();

    // sort
    network_clans_info.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // alphabetically

    response.data(Some(network_clans_info));

    response.complete();
    Json(response)
}

async fn network_clans_roster(State(mut state): State<AppState>) -> Json<APINetworkRosterResponse> {
    let mut response = APINetworkRosterResponse::new();

    let basic_results = app::clan::network_roster(&mut state).await;

    let clan_roster = basic_results
        .into_iter()
        .map(MemberResponse::from_db)
        .collect::<Vec<MemberResponse>>();

    response.data(Some(clan_roster));

    response.complete();
    Json(response)
}
