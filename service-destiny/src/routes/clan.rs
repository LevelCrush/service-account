use super::responses::{APIClanInfoResponse, APIClanRosterResponse, ClanInformation, ClanResponse};
use crate::app;
use crate::app::state::AppState;
use crate::routes::responses::MemberResponse;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use levelcrush::types::destiny::GroupId;
use levelcrush::{axum, tracing};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:group_id", get(get_clan))
        .route("/:group_id/roster", get(get_clan_roster))
}

async fn get_clan(Path(group_id): Path<String>, State(mut state): State<AppState>) -> Json<APIClanInfoResponse> {
    let mut response = APIClanInfoResponse::new();

    let clan_info = if let Ok(group_id) = group_id.parse::<GroupId>() {
        app::clan::get(group_id, &mut state).await
    } else {
        app::clan::get_by_slug(&group_id, &mut state).await
    };

    if let Some(clan_info) = clan_info {
        response.data(Some(ClanInformation::from_db(clan_info)));
    } else {
        response.error("group_id", "Group not found");
    }

    response.complete();
    Json(response)
}

async fn get_clan_roster(
    Path(group_id): Path<String>,
    State(mut state): State<AppState>,
) -> Json<APIClanRosterResponse> {
    let mut response = APIClanRosterResponse::new();

    let clan_info = if let Ok(group_id) = group_id.parse::<GroupId>() {
        app::clan::get(group_id, &mut state).await
    } else {
        app::clan::get_by_slug(&group_id, &mut state).await
    };

    if let Some(clan_info) = clan_info {
        tracing::info!("Working on getting roster for group: {}", clan_info.group_id);
        let clan_roster_db = app::clan::get_roster(clan_info.group_id, &mut state).await;

        // convert db records into our api response
        let clan_roster = clan_roster_db
            .into_iter()
            .map(MemberResponse::from_db)
            .collect::<Vec<MemberResponse>>();

        response.data(Some(ClanResponse {
            data: ClanInformation::from_db(clan_info),
            roster: clan_roster,
        }));
    } else {
        response.error("group_id", "Group not found");
    }

    response.complete();
    Json(response)
}
