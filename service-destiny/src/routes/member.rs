use super::responses::{
    APIClanInfoResponse, APIMemberTitleResponse, ClanInformation, MemberTitle, MemberTitleResponse, ReportOutput,
};
use crate::app;
use crate::app::state::AppState;
use crate::routes::responses::{APIMemberResponse, MemberResponse};
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use levelcrush::axum;
use levelcrush::axum::extract::Query;
use levelcrush::server::APIResponse;
use levelcrush::tracing;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ReportQueries {
    pub modes: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:bungie_name", get(member))
        .route("/:bungie_name/clan", get(clan))
        .route("/:bungie_name/report/lifetime", get(lifetime_report))
        .route("/:bungie_name/report", get(lifetime_report))
        .route("/:bungie_name/report/season/:season", get(season_report))
        .route("/:bungie_name/titles", get(member_titles))
}

async fn member_titles(
    Path(bungie_name): Path<String>,
    State(mut state): State<AppState>,
) -> Json<APIMemberTitleResponse> {
    let mut response = APIMemberTitleResponse::new();
    let membership_id = bungie_name.parse::<i64>().unwrap_or_default();
    let member = if membership_id == 0 {
        app::member::by_bungie_name(bungie_name.as_str(), &mut state).await
    } else {
        app::member::profile(membership_id, &mut state).await
    };

    if let Some(member) = member {
        let titles = app::member::titles(member.membership_id, &mut state).await;
        response.data(Some(MemberTitleResponse {
            member: MemberResponse::from_db(member),
            titles: titles.into_iter().map(MemberTitle::from_db).collect(),
        }));
    }

    response.complete();
    Json(response)
}

async fn season_report(
    Path((bungie_name, season)): Path<(String, String)>,
    Query(report_queries): Query<ReportQueries>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<ReportOutput>> {
    let mut response = APIResponse::new();
    let membership_id = bungie_name.parse::<i64>().unwrap_or_default();
    let member = if membership_id == 0 {
        app::member::by_bungie_name(bungie_name.as_str(), &mut state).await
    } else {
        app::member::profile(membership_id, &mut state).await
    };

    if let Some(member) = member {
        let modes = if let Some(input_modes) = report_queries.modes {
            input_modes
                .split(',')
                .map(|v| v.parse::<i32>().unwrap_or_default())
                .collect::<Vec<i32>>()
        } else {
            Vec::new()
        };

        let (task_started, report) = app::report::member::season(
            bungie_name,
            &modes,
            season.parse::<i32>().unwrap_or_default(),
            member.clan_is_network == 1,
            &mut state,
        )
        .await;
        if report.is_some() {
            response.data(Some(ReportOutput::Report(Box::new(
                report.expect("Report should of been here"),
            ))));
        } else {
            response.data(Some(ReportOutput::TaskRunning(task_started)));
        }
    }

    response.complete();
    Json(response)
}

async fn lifetime_report(
    Path(bungie_name): Path<String>,
    Query(report_queries): Query<ReportQueries>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<ReportOutput>> {
    let mut response = APIResponse::new();

    let membership_id = bungie_name.parse::<i64>().unwrap_or_default();
    let member = if membership_id == 0 {
        app::member::by_bungie_name(bungie_name.as_str(), &mut state).await
    } else {
        app::member::profile(membership_id, &mut state).await
    };

    if let Some(member) = member {
        let modes = if let Some(input_modes) = report_queries.modes {
            input_modes
                .split(',')
                .map(|v| v.parse::<i32>().unwrap_or_default())
                .collect::<Vec<i32>>()
        } else {
            Vec::new()
        };

        let (task_started, report) =
            app::report::member::lifetime(bungie_name, &modes, member.clan_is_network == 1, &mut state).await;

        if report.is_some() {
            response.data(Some(ReportOutput::Report(Box::new(
                report.expect("Report should of been here"),
            ))));
        } else {
            response.data(Some(ReportOutput::TaskRunning(task_started)));
        }
    }

    response.complete();
    Json(response)
}

async fn member(Path(bungie_name): Path<String>, State(mut state): State<AppState>) -> Json<APIMemberResponse> {
    let mut response = APIMemberResponse::new();
    let membership_id = bungie_name.parse::<i64>().unwrap_or_default();
    tracing::info!("Search Request: {}", bungie_name);
    let member = if membership_id == 0 {
        app::member::by_bungie_name(bungie_name.as_str(), &mut state).await
    } else {
        app::member::profile(membership_id, &mut state).await
    };
    if let Some(member) = member {
        response.data(Some(MemberResponse::from_db(member)));
    } else {
        response.error("bungie_name", "We could not find this account");
    }

    // mark the response as completed and send it off
    response.complete();
    Json(response)
}

async fn clan(Path(bungie_name): Path<String>, State(mut state): State<AppState>) -> Json<APIClanInfoResponse> {
    let mut response = APIClanInfoResponse::new();

    let membership_id = bungie_name.parse::<i64>().unwrap_or_default();
    let member = if membership_id == 0 {
        app::member::by_bungie_name(bungie_name.as_str(), &mut state).await
    } else {
        app::member::profile(membership_id, &mut state).await
    };

    if let Some(member) = &member {
        let clan_info = app::clan::from_membership(member.membership_id, member.platform, &mut state).await;
        if let Some(clan_info) = clan_info {
            response.data(Some(ClanInformation::from_db(clan_info)));
        } else {
            response.error("group_id", "Could not find clan tied to this member");
        }
    }

    response.complete();
    Json(response)
}
