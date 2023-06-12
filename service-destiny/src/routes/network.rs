use super::responses::{APIClanInfoMultiResponse, APINetworkRosterResponse, ClanInformation, MemberResponse};
use crate::app;
use crate::app::state::AppState;
use axum::{extract::State, routing::get, Json, Router};
use levelcrush::axum;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(network_clans))
        .route("/info", get(network_clans))
        .route("/roster", get(network_clans_roster))
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
