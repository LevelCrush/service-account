use crate::{app::state::AppState, database};
use levelcrush::{
    axum::{extract::State, routing::get, Json, Router},
    server::APIResponse,
};

use super::responses::{Leaderboard, LeaderboardEntry};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/titles", get(leaderboard_titles))
        .route("/raids", get(leaderboard_raids))
}

async fn leaderboard_titles(State(mut state): State<AppState>) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();
    let entries = database::leaderboard::titles(&state.database).await;

    let leaderboard = Leaderboard {
        name: "Title Leaderboard".to_string(),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}

async fn leaderboard_raids(State(mut state): State<AppState>) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();
    let entries = database::leaderboard::raids(&state.database).await;

    let leaderboard = Leaderboard {
        name: "Raid Leaderboard".to_string(),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}
