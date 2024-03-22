use std::{os::windows::process::CommandExt, process::Command};

use crate::utils;

const TAP_EXE: &str = "with_tap_v9.24.7.exe";

pub fn install() {
    // 导入证书
    // certutil -addstore "TrustedPublisher" tap.cer

    Command::new("cmd")
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .args(["/C", TAP_EXE, "/S"])
        .current_dir(utils::path::CurrentPath::default().bin_string())
        .output()
        .expect("Failed to install tap!");
}
