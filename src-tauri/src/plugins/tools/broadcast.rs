use tauri::{AppHandle, Manager as _};
use tokio::process::Command;

use crate::utils;
const WIN_IP_BROADCAST_EXE: &str = "with_winIPBroadcast.exe";

#[tauri::command]
pub fn win_ip_broadcast_start(app: AppHandle) -> Result<(), String> {
    let _ = utils::cmd::kill_process(WIN_IP_BROADCAST_EXE.to_owned());
    let mut child = match Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", WIN_IP_BROADCAST_EXE, "run"])
        .kill_on_drop(true)
        .current_dir(
            app.path()
                .resource_dir()
                .unwrap_or_default()
                .join("bin")
                .to_string_lossy()
                .replace("\\\\?\\", ""),
        )
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
        child.wait().await.unwrap();
        tracing::info!("winIPBroadcast stop");
    });
    Ok(())
}

#[tauri::command]
pub fn win_ip_broadcast_stop() {
    let _ = utils::cmd::kill_process(WIN_IP_BROADCAST_EXE.to_owned());
}