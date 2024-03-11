use std::{os::windows::process::CommandExt, process::Command};

use crate::util;

pub fn win_ip_broadcast() {
    tauri::async_runtime::spawn(async {
        let mut child = Command::new("cmd")
            .creation_flags(0x08000000)
            .args(["/C", "with_winIPBroadcast.exe", "run"])
            .current_dir(util::CurrentPath::default().bin_string())
            .spawn()
            .unwrap();
        tracing::info!("winIPBroadcast start");
        child.wait().unwrap();
        tracing::info!("winIPBroadcast stop");
    });
}
