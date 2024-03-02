use std::process;

use vnt::handle::callback::ConnectInfo;
use vnt::{DeviceInfo, ErrorInfo, HandshakeInfo, RegisterInfo, VntCallback};

#[derive(Clone)]
pub struct VntHandler {}

impl VntCallback for VntHandler {
    fn create_tun(&self, info: DeviceInfo) {
        tracing::info!("虚拟网卡: {}", info);
    }

    fn connect(&self, info: ConnectInfo) {
        if info.count > 1 {
            tracing::info!("第{:2}次重新连接", info.count - 1);
        }
    }

    fn handshake(&self, info: HandshakeInfo) -> bool {
        tracing::info!("通信握手: {}", info);
        true
    }

    fn register(&self, info: RegisterInfo) -> bool {
        tracing::info!("服务注册: {}", info);
        true
    }

    fn error(&self, info: ErrorInfo) {
        tracing::error!("发生错误: {}", info);
    }

    fn stop(&self) {
        tracing::info!("服务停止");
        process::exit(0)
    }
}
