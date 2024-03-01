#[cfg(target_os = "android")]
mod android;
#[cfg(any(target_os = "linux"))]
mod linux;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod linux_mac;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "android")]
pub use android::create;
#[cfg(target_os = "android")]
pub use android::{DeviceReader, DeviceWriter};
#[cfg(any(target_os = "linux"))]
pub use linux::create_device;
#[cfg(any(target_os = "linux"))]
pub use linux::delete_device;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use linux_mac::{DeviceReader, DeviceWriter};
#[cfg(target_os = "macos")]
pub use mac::create_device;
#[cfg(target_os = "macos")]
pub use mac::delete_device;

#[cfg(target_os = "windows")]
pub use windows::create_device;
#[cfg(target_os = "windows")]
pub use windows::delete_device;
#[cfg(target_os = "windows")]
pub use windows::{DeviceReader, DeviceWriter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeviceType {
    Tun,
    Tap,
}

impl DeviceType {
    pub fn is_tun(&self) -> bool {
        *self == DeviceType::Tun
    }
}

#[derive(Clone)]
pub struct DriverInfo {
    pub device_type: DeviceType,
    pub name: String,
    pub version: String,
    pub mac: Option<String>,
}
