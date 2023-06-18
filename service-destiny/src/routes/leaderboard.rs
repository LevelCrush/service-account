use crate::{app::state::AppState, database};
use levelcrush::{
    axum::{extract::State, routing::get, Json, Router},
    server::APIResponse,
};

use super::responses::{Leaderboard, LeaderboardEntry};

pub fn router() -> Router<AppState> {
    Router::new().route("/titles", get(leadeboard_titles))
}

async fn leadeboard_titles(State(mut state): State<AppState>) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();
    let entries = database::leaderboard::titles(&state.database).await;

    let leaderboard = Leaderboard {
        name: "Title Collectors".to_string(),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}
