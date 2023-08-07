use levelcrush::anyhow;
use levelcrush::chrono;
use levelcrush::chrono::TimeZone;
use levelcrush::server::APIResponse;
use levelcrush::tokio;
use levelcrush::tracing;
use lib_destiny::aliases::GroupId;
use lib_destiny::app;
use lib_destiny::app::responses::*;
use std::collections::HashMap;
use tauri::State;

use crate::state::LibDestinyState;

#[tauri::command]
pub async fn clan_info(group_id: String, state: State<'_, LibDestinyState>) -> Result<APIClanInfoResponse, ()> {
    let mut state = state.write().await;
    let mut response = APIClanInfoResponse::new();

    let clan_info = if let Ok(group_id) = group_id.parse::<GroupId>() {
        app::clan::get(group_id, &mut state).await
    } else {
        app::clan::get_by_slug(&group_id, &mut state).await
    };

    if let Some(clan_info) = clan_info {
        response.data(Some(ClanInformation::from_db(clan_info)));
    } else {
        response.error("group_id", "Group not found");
    }

    response.complete();
    Ok(response)
}

#[tauri::command]
pub async fn clan_roster(group_id: String, state: State<'_, LibDestinyState>) -> Result<APIClanRosterResponse, ()> {
    let mut state = state.write().await;
    let mut response = APIClanRosterResponse::new();

    let clan_info = if let Ok(group_id) = group_id.parse::<GroupId>() {
        app::clan::get(group_id, &mut state).await
    } else {
        app::clan::get_by_slug(&group_id, &mut state).await
    };

    if let Some(clan_info) = clan_info {
        tracing::info!("Working on getting roster for group: {}", clan_info.group_id);
        let clan_roster_db = app::clan::get_roster(clan_info.group_id, &mut state).await;

        // convert db records into our api response
        let clan_roster = clan_roster_db
            .into_iter()
            .map(MemberResponse::from_db)
            .collect::<Vec<MemberResponse>>();

        response.data(Some(ClanResponse {
            data: ClanInformation::from_db(clan_info),
            roster: clan_roster,
        }));
    } else {
        response.error("group_id", "Group not found");
    }

    response.complete();
    Ok(response)
}
