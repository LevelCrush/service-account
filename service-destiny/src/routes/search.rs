use super::queries::PaginationQuery;
use super::responses::APIMemberSearchResponse;
use crate::app;
use crate::app::state::AppState;
use crate::routes::responses::MemberResponse;
use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use levelcrush::axum;
use levelcrush::server::{PaginationData, PaginationResponse};

pub fn router() -> Router<AppState> {
    Router::new()
        // .route("/member/:bungie_name", get(search_by_bungie_name))
        .route("/members/:display_name", get(search_by_bungie_name_wild))
}

/// unlike a normal search by nungie name, the  wild version queries **ONLY** the database
/// and as a result can return **multiple** members even without the membership id. But this requires
/// the member profile to be in our system
async fn search_by_bungie_name_wild(
    Path(display_name): Path<String>,
    pagination: Query<PaginationQuery>,
    State(mut state): State<AppState>,
) -> Json<APIMemberSearchResponse> {
    let mut response = APIMemberSearchResponse::new();
    let pagination = pagination.0;

    // extract our queries
    let page = pagination.get_page();
    let limit = pagination.get_limit();

    // query the database
    let db_count_results = app::member::search_count(display_name.as_str(), &mut state).await;
    let total_pages = if db_count_results > 0 {
        ((db_count_results as f32) / (limit as f32)).ceil() as u32
    } else {
        0
    };

    // fetch db results if possible
    let db_results = if db_count_results > 0 {
        app::member::search(display_name.as_str(), page, limit, &mut state).await
    } else {
        Vec::new()
    };

    // map into our expected response body
    let member_results = db_results
        .into_iter()
        .map(MemberResponse::from_db)
        .collect::<Vec<MemberResponse>>();

    let pagination_data = PaginationData {
        total_results: db_count_results,
        total_pages,
        page: page + 1,
        limit,
        showing: member_results.len(),
        term: display_name,
    };
    // set as body data
    response.data(Some(PaginationResponse {
        data: member_results,
        pagination: pagination_data,
    }));

    response.complete();
    Json(response)
}
