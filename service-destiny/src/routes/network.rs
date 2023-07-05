use std::collections::HashMap;

use super::{
    member::ReportQueries,
    responses::{APIClanInfoMultiResponse, APINetworkRosterResponse, ClanInformation, MemberResponse, ReportOutput},
};
use crate::app::state::AppState;
use crate::{app, database};
use axum::{extract::State, routing::get, Json, Router};
use levelcrush::{
    axum::{
        self,
        extract::{Path, Query},
    },
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
