use levelcrush::anyhow;
use levelcrush::chrono;
use levelcrush::chrono::TimeZone;
use levelcrush::server::APIResponse;
use levelcrush::server::PaginationData;
use levelcrush::server::PaginationResponse;
use levelcrush::tokio;
use levelcrush::tracing;
use lib_destiny::app;
use lib_destiny::app::responses::*;
use std::collections::HashMap;
use tauri::State;

use crate::state::LibDestinyState;

use super::PaginationQuery;
use super::ReportQueries;

/// unlike a normal search by nungie name, the  wild version queries **ONLY** the database
/// and as a result can return **multiple** members even without the membership id. But this requires
/// the member profile to be in our system
#[tauri::command]
pub async fn search_bungie_name(
    display_name: String,
    pagination: PaginationQuery,
    state: State<'_, LibDestinyState>,
) -> Result<APIMemberSearchResponse, ()> {
    let mut state = state.write().await;
    let mut response = APIMemberSearchResponse::new();

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
    Ok(response)
}
