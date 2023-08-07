use levelcrush::anyhow;
use levelcrush::chrono;
use levelcrush::chrono::TimeZone;
use levelcrush::server::APIResponse;
use levelcrush::tokio;
use levelcrush::tracing;
use lib_destiny::app;
use lib_destiny::app::responses::*;
use lib_destiny::database::setting::SettingModeRecord;
use std::collections::HashMap;
use tauri::State;

use crate::state::LibDestinyState;

pub async fn get_active_seasons(state: State<'_, LibDestinyState>) -> Result<APIResponse<Vec<DestinySeason>>, ()> {
    let state = state.read().await;

    let mut response = APIResponse::new();
    let seasons = match state.seasons.access("active_seasons").await {
        Some(data) => data,
        _ => Vec::new(),
    };

    let mapped_seasons = seasons.into_iter().map(DestinySeason::from_db).collect();
    response.data(Some(mapped_seasons));

    response.complete();
    Ok(response)
}

pub async fn get_leaderboard_modes(
    state: State<'_, LibDestinyState>,
) -> Result<APIResponse<Vec<SettingModeRecord>>, ()> {
    let state = state.read().await;

    let mut response = APIResponse::new();
    let map = app::settings::modes(&state).await;

    let leaderboard_modes = map
        .into_iter()
        .filter_map(|r| if r.leaderboard == 1 { Some(r) } else { None })
        .collect::<Vec<SettingModeRecord>>();

    response.data(Some(leaderboard_modes));

    response.complete();
    Ok(response)
}

pub async fn get_dashboard_modes(state: State<'_, LibDestinyState>) -> Result<APIResponse<Vec<SettingModeRecord>>, ()> {
    let state = state.read().await;

    let mut response = APIResponse::new();
    let map = app::settings::modes(&state).await;

    let dashboard_modes = map
        .into_iter()
        .filter_map(|r| if r.dashboard == 1 { Some(r) } else { None })
        .collect::<Vec<SettingModeRecord>>();

    response.data(Some(dashboard_modes));
    response.complete();

    Ok(response)
}

pub async fn get_modes(state: State<'_, LibDestinyState>) -> Result<APIResponse<Vec<SettingModeRecord>>, ()> {
    let state = state.read().await;

    let mut response = APIResponse::new();
    let map = app::settings::modes(&state).await;

    response.data(Some(map));
    response.complete();

    Ok(response)
}
