pub mod channel;
pub mod responses;
use crate::app::state::AppState;
use axum::Router;
use levelcrush::axum;

pub fn router() -> Router<AppState> {
    Router::new().nest("/channels", channel::router())
}
