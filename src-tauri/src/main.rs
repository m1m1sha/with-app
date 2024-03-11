// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::WithState;
use std::sync::Mutex;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod core;
mod tool;
mod util;

fn main() {
    tracing_subscriber::fmt().init();
    tool::win_ip_broadcast();
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new().add_item(quit); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .manage(core::WithState(Mutex::new(None)))
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    let state = app.state::<WithState>();
                    if state.0.lock().unwrap().is_some() {
                        state.0.lock().unwrap().as_ref().unwrap().stop();
                        *state.0.lock().unwrap() = None;
                    }

                    let pids = util::get_process_list("with_winIPBroadcast".to_owned());
                    if pids.len() > 0 {
                        for p in pids {
                            let _ = util::kill_process_force(p.to_string());
                        }
                    }
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            core::with_start,
            core::with_stop,
            core::with_route,
            core::with_status
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
