use serde::{Deserialize, Serialize};
use std::{
    net::{Ipv4Addr, ToSocketAddrs},
    sync::Mutex,
};
use tauri::{api::path::app_config_dir, Manager, Window};
use vnt::{
    channel::{
        punch::{NatType, PunchModel},
        Route, UseChannelType,
    },
    cipher::CipherModel,
    core::{Config, Vnt},
    handle::{callback, PeerDeviceStatus},
};

use crate::util;
pub struct WithState(pub Mutex<Option<vnt::core::Vnt>>);
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum CipherMode {
    AesGcm,
    AesCbc,
    AesEcb,
    Sm4Cbc,
    None,
}

impl CipherMode {
    fn to_model(&self) -> CipherModel {
        match self {
            CipherMode::AesGcm => CipherModel::AesGcm,
            CipherMode::AesCbc => CipherModel::AesCbc,
            CipherMode::AesEcb => CipherModel::AesEcb,
            CipherMode::Sm4Cbc => CipherModel::Sm4Cbc,
            CipherMode::None => CipherModel::None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FrontConfig {
    pub stuns: Vec<String>,      // stun 节点
    pub server: String,          // withs 节点
    pub token: String,           // 组网 token | 房间名
    pub passwd: String,          // 组网密码
    pub name: String,            // 组网昵称
    pub proxy: bool,             // 内置代理
    pub server_encrypt: bool,    // 服务端加密
    pub metric: u16,             // 网卡跃点
    pub tcp: bool,               // 强制tcp
    pub ip: String,              // 自定义 ip
    pub latency: bool,           // 延迟优先
    pub parallel: usize,         // 处理协程数
    pub finger: bool,            // 指纹
    pub cipher: CipherMode,      // 加密模式
    pub punch: PunchModel,       // 打洞模式
    pub channel: UseChannelType, // 信道模式
}

#[tauri::command]
pub async fn with_start(
    config: FrontConfig,
    state: tauri::State<'_, WithState>,
    window: Window,
) -> Result<(), String> {
    let server = match config.server.to_socket_addrs() {
        Ok(mut addr) => {
            if let Some(addr) = addr.next() {
                addr
            } else {
                return Err("server error".to_owned());
            }
        }
        Err(_) => {
            return Err("server error".to_owned());
        }
    };
    let passwd = if config.passwd.is_empty() {
        None
    } else {
        Some(config.passwd)
    };
    let udi = get_udi();

    let name = if config.name.is_empty() {
        udi.clone()
    } else {
        config.name
    };
    let cfg = Config::new(
        config.token,
        udi,
        name,
        server,
        config.server,
        config.stuns,
        vec![],
        vec![],
        passwd,
        None,
        config.tcp,
        None,
        false,
        config.server_encrypt,
        config.parallel,
        config.cipher.to_model(),
        config.finger,
        config.punch,
        None,
        config.latency,
        Some("with-tun".to_owned()),
        config.channel,
        Some("./bin/wintun.dll".to_owned()),
        None,
        100,
    )
    .unwrap();
    let _ = util::clear_network_list();
    let vnt = Vnt::new(cfg, AppCallback { window }).unwrap();
    let vnt_c = vnt.clone();
    tokio::spawn(async move {
        vnt_c.wait();
    });
    *state.0.lock().unwrap() = Some(vnt);
    Ok(())
}

#[tauri::command]
pub async fn with_stop(state: tauri::State<'_, WithState>) -> Result<(), String> {
    if state.0.lock().unwrap().is_some() {
        state.0.lock().unwrap().as_ref().unwrap().stop();
        *state.0.lock().unwrap() = None;
    }
    Ok(())
}

#[tauri::command]
pub fn with_route(state: tauri::State<'_, WithState>) -> Vec<(Ipv4Addr, Vec<Route>)> {
    if let Some(with) = state.0.lock().unwrap().as_ref() {
        with.route_table()
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn with_status(state: tauri::State<'_, WithState>) -> bool {
    if let Some(with) = state.0.lock().unwrap().as_ref() {
        with.current_device().status.online()
    } else {
        false
    }
}

pub fn get_udi() -> String {
    if let Some(id) = util::unique() {
        id
    } else {
        let path_buf = match app_config_dir(&tauri::Config::default()) {
            Some(path_buf) => path_buf.join("udi"),
            None => {
                return String::new();
            }
        };
        if let Ok(id) = std::fs::read_to_string(path_buf.as_path()) {
            id
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            let _ = std::fs::write(path_buf, &id);
            id
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppCallback {
    window: Window,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload {
    flag: String,
    data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutePayload {
    ip: Ipv4Addr,
    routes: Vec<Route>,
}

impl callback::VntCallback for AppCallback {
    fn success(&self) {
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "success".to_owned(),
                data: "".to_owned(),
            },
        );
        tracing::info!("success");
    }

    fn create_tun(&self, _info: callback::DeviceInfo) {
        let data = match serde_json::to_string(&_info) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "created".to_owned(),
                data,
            },
        );
        tracing::info!("create: {:?}", _info);
    }

    fn connect(&self, _info: callback::ConnectInfo) {
        let data = match serde_json::to_string(&_info) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "connect".to_owned(),
                data,
            },
        );
        tracing::info!("cnt: {:?}", _info);
    }

    fn handshake(&self, _info: callback::HandshakeInfo) -> bool {
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "handshake".to_owned(),
                data: "".to_owned(),
            },
        );
        tracing::info!("handshake: {:?}", _info);
        true
    }

    fn register(&self, _info: callback::RegisterInfo) -> bool {
        let data = match serde_json::to_string(&_info) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "register".to_owned(),
                data,
            },
        );
        tracing::info!("register: {:?}", _info);
        true
    }

    fn error(&self, _info: callback::ErrorInfo) {
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "error".to_owned(),
                data: "".to_owned(),
            },
        );
        tracing::warn!("error: {:?}", _info);
    }

    fn route_change(&self, _info: Vec<(Ipv4Addr, Vec<Route>)>) {
        let arr = device_items(self.window.state());
        let data = match serde_json::to_string(&arr) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "route".to_owned(),
                data,
            },
        );
        tracing::info!("route: {:?}", arr);
    }

    fn stop(&self) {
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "stop".to_owned(),
                data: "".to_owned(),
            },
        );
        tracing::warn!("stop");
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceItem {
    pub name: String,
    pub virtual_ip: String,
    pub tcp: bool,
    pub nat_type: NatType,
    pub public_ips: Vec<String>,
    pub local_ip: String,
    pub ipv6: String,
    pub metric: DeviceMetric,
    pub rt: i64,
    pub online: bool,
    pub same_secret: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceMetric {
    P2p = 0,
    ClientRelay,
    ServerRelay,
}

pub fn device_items(state: tauri::State<'_, WithState>) -> Vec<DeviceItem> {
    let mut list = Vec::new();
    if let Some(with) = state.0.lock().unwrap().as_ref() {
        let info = with.current_device();
        let device_list = with.device_list();
        for peer in device_list {
            let name = peer.name;
            let virtual_ip = peer.virtual_ip.to_string();
            let (nat_type, public_ips, local_ip, ipv6) =
                if let Some(nat_info) = with.peer_nat_info(&peer.virtual_ip) {
                    let nat_type = nat_info.nat_type;
                    let public_ips: Vec<String> =
                        nat_info.public_ips.iter().map(|v| v.to_string()).collect();
                    let local_ip = nat_info
                        .local_ipv4()
                        .map(|v| v.to_string())
                        .unwrap_or("".to_string());
                    let ipv6 = nat_info
                        .ipv6()
                        .map(|v| v.to_string())
                        .unwrap_or("".to_string());
                    (nat_type, public_ips, local_ip, ipv6)
                } else {
                    (NatType::Cone, vec![], "".to_string(), "".to_string())
                };

            let (metric, rt, tcp) = if let Some(route) = with.route(&peer.virtual_ip) {
                let metric = if route.metric == 1 {
                    DeviceMetric::P2p
                } else {
                    if let Some(next_hop) = with.route_key(&route.route_key()) {
                        if !info.is_gateway(&next_hop) {
                            DeviceMetric::ClientRelay
                        } else {
                            DeviceMetric::ServerRelay
                        }
                    } else {
                        DeviceMetric::ServerRelay
                    }
                };
                (metric, route.rt, route.is_tcp)
            } else {
                (DeviceMetric::ServerRelay, 0, false)
            };

            let online = match peer.status {
                PeerDeviceStatus::Online => true,
                PeerDeviceStatus::Offline => false,
            };
            // 仅显示加密以后需改为加密方式
            let same_secret = peer.client_secret == with.client_encrypt();
            let item = DeviceItem {
                name,
                virtual_ip,
                tcp,
                nat_type,
                public_ips,
                local_ip,
                ipv6,
                metric,
                rt,
                online,
                same_secret,
            };
            list.push(item);
        }
    }
    list
}
