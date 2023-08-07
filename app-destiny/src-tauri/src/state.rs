use levelcrush::tokio::sync::RwLock;
use std::sync::Mutex;
pub type LibDestinyState = RwLock<lib_destiny::app::state::AppState>;
