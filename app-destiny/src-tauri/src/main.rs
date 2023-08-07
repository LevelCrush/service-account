// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod service_destiny;
pub mod state;

use levelcrush::anyhow;
use levelcrush::tokio;
use levelcrush::tokio::sync::RwLock;
use levelcrush::tracing;
use state::LibDestinyState;
use std::sync::Mutex;
use std::time::Duration;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    levelcrush::env();

    let app_state = lib_destiny::app::state::AppState::new().await;

    tauri::Builder::default()
        .manage(LibDestinyState::new(app_state))
        .invoke_handler(tauri::generate_handler![
            service_destiny::network::network_clans,
            service_destiny::network::network_roster,
            service_destiny::network::network_breakdown_season,
            service_destiny::network::network_breakdown_lifetime,
            service_destiny::network::network_lifetime_report,
            service_destiny::network::network_season_report,
            service_destiny::member::member_info,
            service_destiny::member::member_clan,
            service_destiny::member::member_lifetime_report,
            service_destiny::member::member_season_report,
            service_destiny::member::member_titles,
            service_destiny::clan::clan_info,
            service_destiny::clan::clan_roster
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
