use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

use crate::{core::WithState, util};

pub fn menu() -> SystemTray {
    SystemTray::new()
        .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("quit".to_string(), "退出")))
}

pub fn event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
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

                util::kill_win_ip_broadcast();
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
