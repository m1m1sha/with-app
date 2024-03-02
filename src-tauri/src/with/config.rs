use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use vnt::channel::punch::PunchModel;

use crate::utils::{self, CurrentPath};

#[derive(Serialize, Deserialize, Debug)]
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
    pub cipher_model: String,
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
            cipher_model: "aes_gcm".to_string(),
            finger: false,
            punch_model: PunchModel::All,
            port: 0,
            first_latency: false,
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
