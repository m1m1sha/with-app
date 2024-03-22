use std::{os::windows::process::CommandExt, process::Command};

use crate::utils;
const WIN_IP_BROADCAST_NAME: &str = "with_winIPBroadcast.exe";

#[tauri::command]
pub fn win_ip_broadcast_start() -> Result<(), String> {
    let _ = utils::cmd::kill_process(WIN_IP_BROADCAST_NAME.to_owned());
    let mut child = match Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", WIN_IP_BROADCAST_NAME, "run"])
        .current_dir(utils::path::CurrentPath::default().bin())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("winIPBroadcast failed to start");
            return Err(e.to_string());
        }
    };

    tauri::async_runtime::spawn(async move {
        tracing::info!("winIPBroadcast start");
        child.wait().unwrap();
        tracing::info!("winIPBroadcast stop");
    });
    Ok(())
}

#[tauri::command]
pub fn win_ip_broadcast_stop() {
    let _ = utils::cmd::kill_process(WIN_IP_BROADCAST_NAME.to_owned());
}

pub fn kill_win_ip_broadcast() {
    let _ = utils::cmd::kill_process(WIN_IP_BROADCAST_NAME.to_owned());
}
