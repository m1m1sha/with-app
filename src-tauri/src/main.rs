// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    net::{Ipv4Addr, ToSocketAddrs},
    sync::{Arc, Mutex},
    vec,
};

use serde::{Deserialize, Serialize};
use tauri::{api::path::app_config_dir, Window};
use with::channel::{punch::PunchMode, ChannelMode, Route};
use with::cipher::CipherMode;
use with::handler::callback;

mod util;

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .manage(WithState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            with_start,
            with_stop,
            with_route,
            with_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub struct WithState(Mutex<Option<with::core::With>>);
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FrontConfig {
    pub stuns: Vec<String>,   // stun 节点
    pub server: String,       // withs 节点
    pub token: String,        // 组网 token | 房间名
    pub passwd: String,       // 组网密码
    pub name: String,         // 组网昵称
    pub proxy: bool,          // 内置代理
    pub server_encrypt: bool, // 服务端加密
    pub tcp: bool,            // 强制tcp
    pub ip: String,           // 自定义 ip
    pub latency: bool,        // 延迟优先
    pub parallel: usize,      // 处理协程数
    pub finger: bool,         // 指纹
    pub cipher: CipherMode,   // 加密模式
    pub punch: PunchMode,     // 打洞模式
    pub channel: ChannelMode, // 信道模式
}

#[tauri::command]
async fn with_start(
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

    let mtu = if passwd.is_none() { 1450 } else { 1410 };
    let udi = get_udi();
    let cfg = with::config::Config::new(
        udi.clone(),
        None,
        server,
        config.token,
        passwd,
        udi,
        config.proxy,
        config.server_encrypt,
        None,
        mtu,
        config.tcp,
        None,
        vec![],
        vec![],
        config.latency,
        config.parallel,
        config.finger,
        config.cipher,
        config.punch,
        config.channel,
        Some("./bin/wintun.dll".to_owned()),
        None,
        100,
    )
    .unwrap();
    let core = with::core::Core::new(cfg).unwrap();

    let with = core.init(AppCallback { window }).await.unwrap();
    let with_c = with.clone();
    tauri::async_runtime::spawn(async move {
        with_c.wait();
    });
    *state.0.lock().unwrap() = Some(with);
    Ok(())
}

#[tauri::command]
async fn with_stop(state: tauri::State<'_, WithState>) -> Result<(), String> {
    state.0.lock().unwrap().as_ref().unwrap().stop();
    *state.0.lock().unwrap() = None;
    Ok(())
}

#[tauri::command]
fn with_route(state: tauri::State<'_, WithState>) -> Vec<(Ipv4Addr, Vec<with::channel::Route>)> {
    if let Some(with) = state.0.lock().unwrap().as_ref() {
        with.route_table()
    } else {
        vec![]
    }
}

#[tauri::command]
fn with_status(state: tauri::State<'_, WithState>) -> bool {
    if let Some(with) = state.0.lock().unwrap().as_ref() {
        with.current_device().status.online()
    } else {
        false
    }
}

fn get_udi() -> String {
    if let Some(id) = with::utils::identifier::unique() {
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
struct EventPayload {
    flag: String,
    data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoutePayload {
    ip: Ipv4Addr,
    routes: Vec<Route>,
}

impl callback::Callback for AppCallback {
    fn success(&self) {
        // emit("with-success", ());
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
                flag: "connecting".to_owned(),
                data,
            },
        );
        tracing::info!("cnt: {:?}", _info);
    }

    fn handshake(&self, _info: callback::HandshakeInfo) -> bool {
        let data = match serde_json::to_string(&_info) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "handshake".to_owned(),
                data,
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
        let data = match serde_json::to_string(&_info) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let _ = self.window.emit(
            "with_event_connect",
            EventPayload {
                flag: "error".to_owned(),
                data,
            },
        );
        tracing::warn!("error: {:?}", _info);
    }

    fn route_change(&self, _info: Vec<(Ipv4Addr, Vec<Route>)>) {
        let mut arr = Vec::new();
        for ele in _info {
            arr.push(RoutePayload {
                ip: ele.0,
                routes: ele.1,
            });
        }

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
