pub mod clan;
pub mod leaderboard;
pub mod member;
pub mod network;
pub mod queries;
pub mod rank;
pub mod responses;
pub mod search;
pub mod settings;

use levelcrush::axum::Router;
use lib_destiny::app::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/search", search::router())
        .nest("/network", network::router())
        .nest("/member", member::router())
        .nest("/clan", clan::router())
        .nest("/leaderboard", leaderboard::router())
        .nest("/rank", rank::router())
        .nest("/settings", settings::router())
}
