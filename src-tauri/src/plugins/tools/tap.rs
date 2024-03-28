use tauri::{AppHandle, Manager as _};
use tokio::process::Command;

const TAP_EXE: &str = "with_tap_v9.24.7.exe";

#[tauri::command]
pub async fn install(silent: Option<bool>, app: AppHandle) -> Result<bool, ()> {
    // 导入证书
    // certutil -addstore "TrustedPublisher" tap.cer
    let silent = silent.unwrap_or(true);
    if let Ok(o) = Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", TAP_EXE, if silent { "/S" } else { "" }])
        .current_dir(
            app.path()
                .resource_dir()
                .unwrap_or_default()
                .join("bin")
                .to_string_lossy()
                .replace("\\\\?\\", ""),
        )
        .output()
        .await
    {
        Ok(o.status.success())
    } else {
        Ok(false)
    }
}
