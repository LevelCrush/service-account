use super::state::{AppState, Setting};
use crate::database::setting::SettingModeRecord;

pub const CACHE_KEY_MODES: &str = "modes";

pub async fn modes(state: &AppState) -> Vec<SettingModeRecord> {
    let cache = match state.settings.access(CACHE_KEY_MODES).await {
        Some(Setting::Modes(data)) => data,
        _ => Vec::new(),
    };
    cache
}
