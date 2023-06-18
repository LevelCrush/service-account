use std::collections::HashMap;

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
        .route("/modes", get(get_modes))
}

async fn get_modes(State(state): State<AppState>) -> Json<APIResponse<Vec<SettingModeRecord>>> {
    let mut response = APIResponse::new();

    let map = app::settings::modes(&state).await;
    response.data(Some(map));

    response.complete();
    Json(response)
}
