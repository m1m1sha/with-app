use encoding::{all::GBK, DecoderTrap, Encoding};
use std::{os::windows::process::CommandExt, path::PathBuf, process::Command};
use sysinfo::{Pid, Process, System};

#[derive(Debug, Clone)]
pub struct CurrentPath {
    current: PathBuf,
}

impl Default for CurrentPath {
    fn default() -> Self {
        Self {
            current: std::env::current_dir().unwrap(),
        }
    }
}

#[allow(dead_code)]
impl CurrentPath {
    pub fn current(&self) -> PathBuf {
        self.current.clone()
    }

    pub fn current_string(&self) -> String {
        self.to_string(self.current())
    }

    pub fn bin(&self) -> PathBuf {
        self.current.join("bin")
    }

    pub fn bin_string(&self) -> String {
        self.to_string(self.bin())
    }

    pub fn config(&self) -> PathBuf {
        self.current.join("config")
    }

    pub fn config_string(&self) -> String {
        self.to_string(self.config())
    }

    fn to_string(&self, buf: PathBuf) -> String {
        let mut path = buf.to_string_lossy().to_string();

        if path.contains('�') {
            if let Ok(s) = GBK.decode(path.as_bytes(), DecoderTrap::Strict) {
                path = s;
            }
        }

        path
    }
}

pub fn kill_process_force(pid: String) -> Result<(), String> {
    if pid.is_empty() {
        return Err("Pid cannot be empty".to_owned());
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
