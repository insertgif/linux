#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod klipy;
mod provider;

use klipy::KlipyProvider;
use provider::{AnyProvider, Gif};
use tauri::State;

#[tauri::command]
async fn search_gifs(query: String, provider: State<'_, AnyProvider>) -> Result<Vec<Gif>, String> {
    provider.search(&query, 50).await
}

#[tauri::command]
async fn trending_gifs(provider: State<'_, AnyProvider>) -> Result<Vec<Gif>, String> {
    provider.trending(50).await
}

fn main() {
    let api_key = config::load_api_key().unwrap_or_else(|e| {
        eprintln!("insertgif: {e} (searches will fail until configured — see README)");
        String::new()
    });
    let provider = AnyProvider::Klipy(KlipyProvider::new(api_key));

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .manage(provider)
        .invoke_handler(tauri::generate_handler![search_gifs, trending_gifs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
