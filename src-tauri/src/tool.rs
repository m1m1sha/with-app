use std::{os::windows::process::CommandExt, process::Command};

use crate::util;

#[tauri::command]
pub fn win_ip_broadcast_start() -> Result<(), String> {
    let mut child = match Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", "with_winIPBroadcast.exe", "run"])
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
    let pids = util::get_process_list("with_winIPBroadcast".to_owned());
    if pids.len() > 0 {
        for p in pids {
            let _ = util::kill_process_force(p.to_string());
        }
    }
}
