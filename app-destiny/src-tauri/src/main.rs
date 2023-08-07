// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use levelcrush::anyhow;
use levelcrush::tokio;
use levelcrush::tracing;

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

fn main() {
    levelcrush::env();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![test1])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
