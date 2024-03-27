use tokio::process::Command;

use crate::utils;

const TAP_EXE: &str = "with_tap_v9.24.7.exe";

#[tauri::command]
pub async fn install(silent: Option<bool>) -> Result<bool, ()> {
    // 导入证书
    // certutil -addstore "TrustedPublisher" tap.cer
    let silent = silent.unwrap_or(true);
    if let Ok(o) = Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", TAP_EXE, if silent { "/S" } else { "" }])
        .current_dir(utils::path::CurrentPath::default().bin())
        .output()
        .await
    {
        Ok(o.status.success())
    } else {
        Ok(false)
    }
}
