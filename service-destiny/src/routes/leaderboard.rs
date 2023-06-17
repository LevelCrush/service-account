use crate::app::state::AppState;
use levelcrush::axum::{routing::get, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/titles", get(leaderboard_titles))
}

async fn leaderboard_titles() {
    
}
