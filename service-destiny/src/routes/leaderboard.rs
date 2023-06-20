use crate::{
    app::{self, state::AppState},
    bungie::enums::DestinyActivityModeType,
    database::{self, setting::SettingModeRecord},
};
use levelcrush::{
    axum::{
        extract::{Path, State},
        routing::get,
        Json, Router,
    },
    cache::{CacheDuration, CacheValue},
    server::APIResponse,
    tracing,
};

use super::responses::{Leaderboard, LeaderboardEntry};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/titles", get(leaderboard_titles))
        .route("/Titles", get(leaderboard_titles))
        .route("/raids", get(leaderboard_raids))
        .route("/Raids", get(leaderboard_raids))
        .route("/raid", get(leaderboard_raids))
        .route("/Raid", get(leaderboard_raids))
        .route("/:activity", get(leaderboard_generic))
}

fn extract_leaderboard_modes(modes: Vec<SettingModeRecord>) -> Vec<SettingModeRecord> {
    modes
        .into_iter()
        .filter_map(|r| if r.leaderboard == 1 { Some(r) } else { None })
        .collect()
}

async fn leaderboard_generic(
    Path(activity): Path<String>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();
    let group_modes = app::settings::modes(&state).await;
    let group_modes = extract_leaderboard_modes(group_modes);

    let (group_name, mut target_group_modes) = {
        let mut group_name = String::new();
        let mut target = None;
        'group_mode: for group_mode in group_modes.iter() {
            if group_mode.name == activity {
                group_name = group_mode.name.clone();
                tracing::info!("Found a matching group mode! {}", group_mode.name);
                target = Some(
                    group_mode
                        .value
                        .split(',')
                        .map(|v| v.parse::<i32>().unwrap_or_default())
                        .collect::<Vec<i32>>(),
                );
                break 'group_mode;
            }
        }
        (group_name, target)
    };

    if target_group_modes.is_none() {
        target_group_modes = {
            let activity_mode = DestinyActivityModeType::from(activity.as_str());
            match activity_mode {
                DestinyActivityModeType::Unknown => None,
                target_mode => Some(vec![target_mode as i32]),
            }
        }
    }

    let mut modes = target_group_modes.unwrap_or_default();
    modes.sort();

    let mode_str = modes.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",");
    let mut did_db_update = false;
    let entries = match state.leaderboards.access(&mode_str).await {
        Some(data) => data,
        _ => {
            did_db_update = true;
            if group_name.contains("PvP") {
                database::leaderboard::pvp_based(&modes, &state.database).await
            } else {
                database::leaderboard::generic(&modes, &state.database).await
            }
        }
    };

    if did_db_update {
        // not in app groups should be cached for only an hour
        state
            .leaderboards
            .write(
                &mode_str,
                CacheValue::with_duration(entries.clone(), CacheDuration::OneHour, CacheDuration::OneHour),
            )
            .await
    }

    let mode_names = modes
        .into_iter()
        .map(|m| {
            let mode = DestinyActivityModeType::from(m);
            mode.as_str()
        })
        .collect::<Vec<&str>>()
        .join(", ");

    let leaderboard = Leaderboard {
        name: format!("{} Leaderboard", mode_names),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
        description: String::new(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}

async fn leaderboard_titles(State(state): State<AppState>) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();

    let entries = match state.leaderboards.access("Titles").await {
        Some(data) => data,
        _ => Vec::new(),
    };

    let leaderboard = Leaderboard {
        name: "Title Leaderboard".to_string(),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
        description: String::new(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}

async fn leaderboard_raids(State(state): State<AppState>) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();

    let entries = match state.leaderboards.access("Raid").await {
        Some(data) => data,
        _ => Vec::new(),
    };

    let leaderboard = Leaderboard {
        name: "Raid Leaderboard".to_string(),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
        description: String::new(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}
