// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;
use lib::{MusicService, SearchRequest};

#[tauri::command]
async fn search_music(request: SearchRequest) -> Result<lib::AggregatedResult, String> {
    MusicService::search(request)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_app_info() -> Result<serde_json::Value, String> {
    MusicService::get_app_info()
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![search_music, get_app_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
