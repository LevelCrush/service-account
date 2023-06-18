use crate::{
    app::{self, state::AppState},
    bungie::enums::DestinyActivityModeType,
    database,
};
use levelcrush::{
    axum::{
        extract::{Path, State},
        routing::get,
        Json, Router,
    },
    server::APIResponse,
    tracing,
};

use super::responses::{Leaderboard, LeaderboardEntry};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/titles", get(leaderboard_titles))
        .route("/raids", get(leaderboard_raids))
        .route("/:activity", get(leaderboard_generic))
}

async fn leaderboard_generic(
    Path(activity): Path<String>,
    State(mut state): State<AppState>,
) -> Json<APIResponse<Leaderboard>> {
    let mut response = APIResponse::new();
    let group_modes = app::settings::modes(&state).await;

    let mut target_group_modes = {
        let mut target = None;
        'group_mode: for group_mode in group_modes.iter() {
            if group_mode.name == activity {
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
        target
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

    let modes = target_group_modes.unwrap_or_default();
    let entries = database::leaderboard::generic(&modes, &state.database).await;
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
    };

    response.data(Some(leaderboard));

    response.complete();
    Json(response)
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
