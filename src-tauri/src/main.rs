// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;
mod utils;

use plugins::{
    n2n::{self, commands::EdgeState},
    tools,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            n2n::commands::edge_action,
            n2n::commands::edge_start,
            tools::broadcast::win_ip_broadcast_start,
            tools::broadcast::win_ip_broadcast_stop,
            tools::tap::install
        ])
        .setup(|app| {
            use tauri::Manager;

            let app_config = app.path().app_config_dir().unwrap();

            if !app_config.exists() {
                std::fs::create_dir_all(app_config).unwrap();
            }

            app.manage(EdgeState::default());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
