pub mod clan;
pub mod setting;
pub mod leaderboard;
pub mod member;
pub mod network;
pub mod queries;
pub mod responses;
pub mod search;

use crate::app::state::AppState;
use levelcrush::axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/search", search::router())
        .nest("/network", network::router())
        .nest("/member", member::router())
        .nest("/clan", clan::router())
        .nest("/leaderboard", leaderboard::router())
        .nest("/control", setting::router())
}
