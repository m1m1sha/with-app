use std::path::PathBuf;

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
        buf.to_string_lossy().to_string()
    }
}
