#![allow(dead_code)]

use std::io::Result;
use tokio::process::Command;

pub fn clear_network_profiles() -> Result<()> {
    let hklm = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let network_list =
        hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\NetworkList")?;

    let profiles = network_list.open_subkey("Profiles")?;

    for profile in profiles.enum_keys().map(|p| p.unwrap()) {
        profiles.delete_subkey(profile)?;
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub async fn unique() -> Option<String> {
    let output = match Command::new("wmic")
        .creation_flags(0x08000000)
        .kill_on_drop(true)
        .args(&["csproduct", "get", "UUID"])
        .output()
        .await
    {
        Ok(output) => output,
        Err(_) => {
            return None;
        }
    };

    let result = String::from_utf8_lossy(&output.stdout);
    let identifier = result.lines().nth(1).unwrap_or("").trim();
    if identifier.is_empty() {
        None
    } else {
        Some(identifier.to_string())
    }
}

#[cfg(target_os = "macos")]
pub async fn unique() -> Option<String> {
    let output = match Command::new("ioreg")
        .args(&["-rd1", "-c", "IOPlatformExpertDevice"])
        .kill_on_drop(true)
        .output()
        .await
    {
        Ok(output) => output,
        Err(_) => {
            return None;
        }
    };

    let result = String::from_utf8_lossy(&output.stdout);
    let identifier = result
        .lines()
        .find(|line| line.contains("IOPlatformUUID"))
        .unwrap_or("")
        .trim();
    if identifier.is_empty() {
        None
    } else {
        Some(identifier.to_string())
    }
}

#[cfg(target_os = "linux")]
pub async fn unique() -> Option<String> {
    let output = match Command::new("dmidecode")
        .arg("-s")
        .arg("system-uuid")
        .kill_on_drop(true)
        .output()
        .await
    {
        Ok(output) => output,
        Err(_) => {
            return None;
        }
    };

    let result = String::from_utf8_lossy(&output.stdout);
    let identifier = result.trim().to_string();
    if identifier.is_empty() {
        None
    } else {
        Some(identifier.to_string())
    }
}
