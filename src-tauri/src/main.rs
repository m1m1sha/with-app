// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::WithState;
use encoding::{all::GBK, DecoderTrap, Encoding};
use std::sync::Mutex;
use tauri::{
    CustomMenuItem, Manager, PackageInfo, PathResolver, SystemTray, SystemTrayEvent, SystemTrayMenu,
};

mod core;
mod tool;
mod util;

pub struct DeeplinkState(pub Mutex<String>);

fn main() {
    tracing_subscriber::fmt().init();

    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new().add_item(quit); // insert the menu items here
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri_plugin_deep_link::prepare("cn.smjb.with");
    tauri::Builder::default()
        .manage(core::WithState(Mutex::new(None)))
        .manage(DeeplinkState(Mutex::new(String::new())))
        .system_tray(system_tray)
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
            core::with_status,
            tool::win_ip_broadcast_start,
            tool::win_ip_broadcast_stop,
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
        .run(|handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
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
