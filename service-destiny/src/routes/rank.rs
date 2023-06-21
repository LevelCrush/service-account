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
        .route("/titles/:display_name", get(rank_titles))
        .route("/Titles/:display_name", get(rank_titles))
        .route("/raids/:display_name", get(rank_raids))
        .route("/Raids/:display_name", get(rank_raids))
        .route("/raid/:display_name", get(rank_raids))
        .route("/Raid/:display_name", get(rank_raids))
        .route("/:activity/:display_name", get(rank_generic))
}

fn extract_leaderboard_modes(modes: Vec<SettingModeRecord>) -> Vec<SettingModeRecord> {
    modes
        .into_iter()
        .filter_map(|r| if r.leaderboard == 1 { Some(r) } else { None })
        .collect()
}

async fn rank_generic(
    Path((activity, display_name)): Path<(String, String)>,
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
    let (should_cache, entries) = match state.ranks.access(&mode_str).await {
        Some(data) => (false, data),
        _ => (
            true,
            if group_name.contains("PvP") {
                database::rank::pvp_based(&display_name, &modes, &state.database).await
            } else {
                database::rank::generic(&display_name, &modes, &state.database).await
            },
        ),
    };

    if should_cache {
        // not in app groups should be cached for only an hour
        state
            .ranks
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

async fn rank_titles(
    Path(display_name): Path<String>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();

    let (should_cache, entries) = match state.ranks.access("Titles").await {
        Some(data) => (false, data),
        _ => (true, database::rank::titles(&display_name, &state.database).await),
    };

    if should_cache {
        state
            .ranks
            .write(
                "Titles",
                CacheValue::with_duration(entries.clone(), CacheDuration::OneHour, CacheDuration::OneHour),
            )
            .await;
    }

    let leaderboard = Leaderboard {
        name: "Title Leaderboard".to_string(),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
        description: String::new(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}

async fn rank_raids(
    Path(display_name): Path<String>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();

    let (should_cache, entries) = match state.ranks.access("Raid").await {
        Some(data) => (false, data),
        _ => (true, database::rank::raids(&display_name, &state.database).await),
    };

    if should_cache {
        state
            .ranks
            .write(
                "Raid",
                CacheValue::with_duration(entries.clone(), CacheDuration::OneHour, CacheDuration::OneHour),
            )
            .await;
    }

    let leaderboard = Leaderboard {
        name: "Raid Leaderboard".to_string(),
        entries: entries.into_iter().map(LeaderboardEntry::from_db).collect(),
        description: String::new(),
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
}
