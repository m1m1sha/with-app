mod root;

use encoding::{all::GBK, DecoderTrap, Encoding};

pub use root::is_app_elevated;

pub fn current_dir() -> String {
    let mut path = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();

    if path.contains('�') {
        // 使用gbk
        if let Ok(s) = GBK.decode(path.as_bytes(), DecoderTrap::Strict) {
            path = s;
        }
    }

    path
}

pub fn current_bin_dir() -> String {
    format!("{}\\bin", current_dir())
}
