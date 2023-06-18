use std::collections::HashMap;

use crate::app;
use crate::app::state::AppState;
use axum::routing::get;
use axum::{Json, Router};
use levelcrush::axum;
use levelcrush::server::{APIResponse, PaginationData, PaginationResponse};

pub fn router() -> Router<AppState> {
    Router::new()
        // .route("/member/:bungie_name", get(search_by_bungie_name))
        .route("/modes", get(get_modes))
}

async fn get_modes() -> Json<APIResponse<HashMap<&'static str, &'static str>>> {
    let mut response = APIResponse::new();

    let map = app::DESTINY_MODE_GROUPS
        .iter()
        .map(|(name, combo)| (*name, *combo))
        .collect::<HashMap<&'static str, &'static str>>();

    response.data(Some(map));

    response.complete();
    Json(response)
}
