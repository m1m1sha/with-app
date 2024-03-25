// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod plugins;
mod tools;
mod utils;

use plugins::n2n::commands::{self, EdgeState};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::edge_community,
            commands::edge_edges,
            commands::edge_status,
            commands::edge_packet_stats,
            commands::edge_supernodes,
            commands::edge_timestamps,
            commands::edge_verbose,
        ])
        .setup(|app| {
            app.manage(EdgeState::default());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
