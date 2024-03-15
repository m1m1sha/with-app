// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::with_stop;
use std::sync::Mutex;
use tauri::Manager;
use tool::win_ip_broadcast_stop;

mod core;
mod tool;
mod tray;
mod util;

pub struct DeeplinkState(pub Mutex<String>);

fn main() {
    tracing_subscriber::fmt().init();

    tauri_plugin_deep_link::prepare("cn.smjb.with");
    tauri::Builder::default()
        .manage(core::WithState(Mutex::new(None)))
        .manage(DeeplinkState(Mutex::new(String::new())))
        // .system_tray(tray::menu())
        .setup(|app| {
            let handle = app.handle();
            tauri_plugin_deep_link::register("withApp", move |request| {
                let state = handle.state::<DeeplinkState>();
                *state.0.lock().unwrap() = request.clone();
                handle.emit_all("deeplink", request).unwrap();
            })
            .unwrap();

            Ok(())
        })
        .on_system_tray_event(|app, event| {
            tray::event(app, event);
        })
        .invoke_handler(tauri::generate_handler![
            core::with_start,
            core::with_stop,
            core::with_route,
            core::with_status,
            tool::win_ip_broadcast_start,
            tool::win_ip_broadcast_stop,
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // event.window().hide().unwrap();
                // api.prevent_close();
                let state = event.window().state::<core::WithState>();
                win_ip_broadcast_stop();
                with_stop(state)
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                // api.prevent_exit();
                let state = handle.state::<core::WithState>();
                win_ip_broadcast_stop();
                with_stop(state)
            }
            tauri::RunEvent::Ready => {
                let state = handle.state::<DeeplinkState>();
                if !state.0.lock().unwrap().is_empty() {
                    let request = state.0.lock().unwrap().clone();
                    handle.emit_all("deeplink", request).unwrap();
                }
            }
            _ => {}
        });
}
