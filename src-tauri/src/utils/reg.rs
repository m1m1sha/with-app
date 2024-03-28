use std::io::Result;

use serde::{Deserialize, Serialize};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE},
        Storage::FileSystem::{
            CreateFileA, FILE_ATTRIBUTE_SYSTEM, FILE_FLAG_OVERLAPPED, FILE_SHARE_NONE,
            OPEN_EXISTING,
        },
    },
};
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

const NETWORK_CONNECTIONS_KEY: &str =
    "SYSTEM\\CurrentControlSet\\Control\\Network\\{4D36E972-E325-11CE-BFC1-08002BE10318}";
const USER_MODE_DEVICE_DIR: &str = "\\\\.\\Global\\";
const TAP_SUFFIX: &str = ".tap";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAPDevice {
    pub guid: String,
    pub name: String,
}

pub fn network_adapters() -> Result<Vec<TAPDevice>> {
    let network_connection =
        RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(NETWORK_CONNECTIONS_KEY)?;
    let mut adapters = vec![];
    for guid in network_connection.enum_keys().map(|x| x.unwrap()) {
        let connection = match network_connection.open_subkey(format!("{}\\Connection", guid)) {
            Ok(key) => key,
            Err(_) => continue,
        };

        let name: String = connection.get_value("name").unwrap_or_default();

        if name.is_empty() {
            continue;
        }

        let tap_name = format!("{}{}{}", USER_MODE_DEVICE_DIR, guid, TAP_SUFFIX);

        let handle = unsafe {
            CreateFileA(
                PCSTR(tap_name.as_ptr()),
                GENERIC_WRITE.0 | GENERIC_READ.0,
                FILE_SHARE_NONE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_SYSTEM | FILE_FLAG_OVERLAPPED,
                HANDLE::default(),
            )
        };

        if let Ok(handle) = handle {
            if !handle.is_invalid() {
                adapters.push(TAPDevice { guid, name });
                unsafe {
                    let _ = CloseHandle(handle);
                };
            }
        }
    }
    Ok(adapters)
}
