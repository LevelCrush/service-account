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
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn test1() {
    tracing::info!("Waiting 5 seconds");
    tokio::time::sleep(Duration::from_secs(5)).await;
    tracing::info!("Hello World!");
}

#[tokio::main]
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
            service_destiny::network::network_season_report
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
