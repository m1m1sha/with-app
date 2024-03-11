// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Mutex, vec};

mod core;
mod util;

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .manage(core::WithState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            core::with_start,
            core::with_stop,
            core::with_route,
            core::with_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
