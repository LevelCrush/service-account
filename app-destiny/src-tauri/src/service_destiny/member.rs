use levelcrush::anyhow;
use levelcrush::chrono;
use levelcrush::chrono::TimeZone;
use levelcrush::server::APIResponse;
use levelcrush::tokio;
use levelcrush::tracing;
use lib_destiny::app;
use lib_destiny::app::responses::*;
use std::collections::HashMap;
use tauri::State;

use crate::state::LibDestinyState;

use super::ReportQueries;

#[tauri::command]
pub async fn member_titles(
    bungie_name: String,
    state: State<'_, LibDestinyState>,
) -> Result<APIMemberTitleResponse, ()> {
    let mut state = state.write().await;
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
    Ok(response)
}

#[tauri::command]
pub async fn member_season_report(
    state: State<'_, LibDestinyState>,
    bungie_name: String,
    season: String,
    report_queries: ReportQueries,
) -> Result<APIResponse<ReportOutput>, ()> {
    let mut state = state.write().await;
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
                .map(|v| v.parse::<i64>().unwrap_or_default())
                .collect::<Vec<i64>>()
        } else {
            Vec::new()
        };

        let (task_started, report) = app::report::member::season(
            bungie_name,
            &modes,
            season.parse::<i64>().unwrap_or_default(),
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
    Ok(response)
}

#[tauri::command]
pub async fn member_lifetime_report(
    bungie_name: String,
    report_queries: ReportQueries,
    state: State<'_, LibDestinyState>,
) -> Result<APIResponse<ReportOutput>, ()> {
    let mut state = state.write().await;
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
                .map(|v| v.parse::<i64>().unwrap_or_default())
                .collect::<Vec<i64>>()
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
    Ok(response)
}

#[tauri::command]
pub async fn member_info(bungie_name: String, state: State<'_, LibDestinyState>) -> Result<APIMemberResponse, ()> {
    let mut state = state.write().await;
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
    Ok(response)
}

#[tauri::command]
pub async fn member_clan(bungie_name: String, state: State<'_, LibDestinyState>) -> Result<APIClanInfoResponse, ()> {
    let mut state = state.write().await;

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
    Ok(response)
}
