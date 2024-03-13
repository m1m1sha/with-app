use std::{os::windows::process::CommandExt, process::Command};

use crate::util;

#[tauri::command]
pub fn win_ip_broadcast_start() -> Result<(), String> {
    util::kill_win_ip_broadcast();
    let mut child = match Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", "with_winIPBroadcast.exe", "run"])
        .current_dir(util::CurrentPath::default().bin())
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
    util::kill_win_ip_broadcast();
}
