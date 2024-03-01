use crate::error::Error;
pub const VNT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub type Result<T> = std::result::Result<T, Error>;

pub mod channel;
pub mod cipher;
pub mod core;
pub mod error;
pub mod external_route;
pub mod handle;
pub mod igmp_server;
#[cfg(feature = "ip_proxy")]
pub mod ip_proxy;
pub mod nat;
pub mod proto;
pub mod protocol;
pub mod tun_tap_device;
pub mod util;
