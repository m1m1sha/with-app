pub mod identifier;
mod root;

use std::path::PathBuf;

use encoding::{all::GBK, DecoderTrap, Encoding};

pub use root::is_app_elevated;

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
