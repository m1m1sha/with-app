use std::{io::Result, os::windows::process::CommandExt, path::PathBuf, process::Command};
use sysinfo::{Pid, Process, System};

pub fn kill_process_force(pid: String) -> Result<()> {
    if pid.is_empty() {
        return Ok(());
    }

    tracing::info!("ctrlc: [pid: {}]", pid);
    let _ = Command::new("cmd")
        .creation_flags(0x08000000)
        .args(["/C", "tskill", pid.as_str(), "/A"])
        .spawn()
        .unwrap();
    Ok(())
}

pub fn get_process_list(name: String) -> Vec<Pid> {
    if name.is_empty() {
        return vec![];
    }

    System::new_all()
        .processes()
        .values()
        .filter(move |val: &&Process| {
            let mut flag = false;

            if val.name().contains(&name) {
                flag = true;
            }
            flag
        })
        .map(|p| p.pid())
        .collect()
}

pub fn kill_win_ip_broadcast() {
    let pids = get_process_list("with_winIPBroadcast".to_owned());
    if pids.len() > 0 {
        for p in pids {
            let _ = kill_process_force(p.to_string());
        }
    }
}

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
pub fn unique() -> Option<String> {
    let output = match Command::new("wmic")
        .creation_flags(0x08000000)
        .args(&["csproduct", "get", "UUID"])
        .output()
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
pub fn unique() -> Option<String> {
    use std::process::Command;
    let output = match Command::new("ioreg")
        .args(&["-rd1", "-c", "IOPlatformExpertDevice"])
        .output()
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
pub fn unique() -> Option<String> {
    use std::process::Command;
    let output = match Command::new("dmidecode")
        .arg("-s")
        .arg("system-uuid")
        .output()
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
