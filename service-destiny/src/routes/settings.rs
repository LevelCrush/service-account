use crate::app;
use crate::app::state::AppState;
use crate::database::setting::SettingModeRecord;
use axum::routing::get;
use axum::{Json, Router};
use levelcrush::axum;
use levelcrush::axum::extract::State;
use levelcrush::server::APIResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        // .route("/member/:bungie_name", get(search_by_bungie_name))
        .route("/modes/all", get(get_modes))
        .route("/modes/dashboard", get(get_dashboard_modes))
        .route("/modes/leaderboards", get(get_leaderboard_modes))
}

async fn get_leaderboard_modes(State(state): State<AppState>) -> Json<APIResponse<Vec<SettingModeRecord>>> {
    let mut response = APIResponse::new();

    let map = app::settings::modes(&state).await;

    let leaderboard_modes = map
        .into_iter()
        .filter_map(|r| if r.leaderboard == 1 { Some(r) } else { None })
        .collect::<Vec<SettingModeRecord>>();

    response.data(Some(leaderboard_modes));

    response.complete();
    Json(response)
}

async fn get_dashboard_modes(State(state): State<AppState>) -> Json<APIResponse<Vec<SettingModeRecord>>> {
    let mut response = APIResponse::new();

    let map = app::settings::modes(&state).await;

    let dashboard_modes = map
        .into_iter()
        .filter_map(|r| if r.dashboard == 1 { Some(r) } else { None })
        .collect::<Vec<SettingModeRecord>>();

    response.data(Some(dashboard_modes));

    response.complete();
    Json(response)
}

async fn get_modes(State(state): State<AppState>) -> Json<APIResponse<Vec<SettingModeRecord>>> {
    let mut response = APIResponse::new();

    let map = app::settings::modes(&state).await;

    response.data(Some(map));

    response.complete();
    Json(response)
}
