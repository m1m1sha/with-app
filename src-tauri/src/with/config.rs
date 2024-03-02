use serde::{Deserialize, Serialize};

use std::net::{Ipv4Addr, ToSocketAddrs};
use std::path::PathBuf;
use std::str::FromStr;
use vnt::cipher::CipherModel;
use vnt::{channel::punch::PunchModel, core::Config};

use crate::utils::{self, CurrentPath};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct WithConfig {
    pub tap: bool,
    pub token: String,
    pub device_id: String,
    pub name: String,
    pub server_address: String,
    pub stun_server: Vec<String>,
    pub in_ips: Vec<String>,
    pub out_ips: Vec<String>,
    pub password: Option<String>,
    pub simulate_multicast: bool,
    pub mtu: Option<u16>,
    pub tcp: bool,
    pub ip: Option<String>,
    pub relay: bool,
    pub no_proxy: bool,
    pub server_encrypt: bool,
    pub parallel: usize,
    pub cipher_model: CipherModel,
    pub finger: bool,
    pub punch_model: PunchModel,
    pub port: u16,
    pub first_latency: bool,
}

impl Default for WithConfig {
    fn default() -> Self {
        Self {
            tap: false,
            token: "".to_string(),
            device_id: unique_identifier(),
            name: os_info::get().to_string(),
            server_address: "nat1.wherewego.top:29872".to_string(),
            stun_server: vec![
                "stun1.l.google.com:19302".to_string(),
                "stun2.l.google.com:19302".to_string(),
                "stun.qq.com:3478".to_string(),
            ],
            in_ips: vec![],
            out_ips: vec![],
            password: None,
            simulate_multicast: false,
            mtu: None,
            tcp: false,
            ip: None,
            relay: false,
            no_proxy: false,
            server_encrypt: false,
            parallel: 1,
            cipher_model: CipherModel::AesGcm,
            finger: false,
            punch_model: PunchModel::All,
            port: 0,
            first_latency: false,
        }
    }
}

impl WithConfig {
    fn to_config(&self) -> Result<Config, String> {
        let addr = match self.server_address.to_socket_addrs() {
            Ok(mut addr) => {
                if let Some(addr) = addr.next() {
                    addr
                } else {
                    return Err("addr 格式错误".to_owned());
                }
            }
            Err(_) => {
                return Err("addr 格式错误".to_owned());
            }
        };

        let in_ips = match utils::parse::ips_parse(&self.in_ips) {
            Ok(ip) => ip,
            Err(_) => {
                return Err("in ip格式错误, 例如192.168.0.0/24,10.26.0.3".to_owned());
            }
        };

        let out_ips = match utils::parse::out_ips_parse(&self.in_ips) {
            Ok(ip) => ip,
            Err(_) => {
                return Err("out ip格式错误, 例如192.168.0.0/24,10.26.0.3".to_owned());
            }
        };

        let virtual_ip = match self.ip.clone() {
            Some(ip) => match Ipv4Addr::from_str(ip.as_str()) {
                Ok(i) => Some(i),
                Err(_) => None,
            },
            None => None,
        };

        if let Some(virtual_ip) = virtual_ip {
            if virtual_ip.is_unspecified() || virtual_ip.is_broadcast() || virtual_ip.is_multicast()
            {
                return Err("IP校验错误: {virtual_ip}".to_owned());
            }
        }
        match Config::new(
            self.tap,
            self.token.clone(),
            self.device_id.clone(),
            self.name.clone(),
            addr,
            self.server_address.clone(),
            self.stun_server.clone(),
            in_ips,
            out_ips,
            self.password.clone(),
            self.simulate_multicast,
            self.mtu,
            self.tcp,
            virtual_ip,
            self.relay,
            self.no_proxy,
            self.server_encrypt,
            self.parallel,
            self.cipher_model,
            self.finger,
            self.punch_model,
            self.port,
            self.first_latency,
        ) {
            Ok(c) => Ok(c),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub fn unique_identifier() -> String {
    if let Some(id) = utils::identifier::unique() {
        id
    } else {
        let path_buf = PathBuf::from(CurrentPath::default().config());
        if let Ok(id) = std::fs::read_to_string(path_buf.as_path()) {
            id
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            let _ = std::fs::write(path_buf, &id);
            id
        }
    }
}