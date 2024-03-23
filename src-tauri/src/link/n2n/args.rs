use std::{ffi::OsStr, net::Ipv4Addr, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EdgeArgs {
    #[serde(rename = "c")]
    group: String,
    #[serde(rename = "l")]
    server: String,
    #[serde(rename = "p")]
    local_port: Option<u16>,
    #[serde(rename = "D")]
    pmtu: Option<bool>,
    #[serde(rename = "e")]
    local_multicast: Option<String>,
    #[serde(rename = "S")]
    relay_mode: Option<EdgeRelayMode>,
    #[serde(rename = "i")]
    interval: Option<u16>,
    #[serde(rename = "L")]
    ttl: Option<u16>,
    #[serde(rename = "k")]
    group_passwd: Option<String>,
    #[serde(rename = "A")]
    encrypt_mode: Option<EdgeEncryptMode>,
    #[serde(rename = "H")]
    head: Option<bool>,
    #[serde(rename = "Z")]
    zip_mode: Option<EdgeZipMode>,
    #[serde(rename = "selectRtt")]
    select_rtt: Option<bool>,
    #[serde(rename = "selectMac")]
    select_mac: Option<bool>,

    #[serde(rename = "a")]
    virtual_ip: Option<String>,
    #[serde(rename = "m")]
    mac: Option<String>,
    #[serde(rename = "D")]
    device_name: Option<String>,
    #[serde(rename = "M")]
    mtu: Option<u16>,
    #[serde(rename = "r")]
    relay_data: Option<bool>,
    #[serde(rename = "E")]
    multicast: Option<bool>,
    #[serde(rename = "I")]
    desc: Option<String>,
    #[serde(rename = "J")]
    node_passwd: Option<String>,
    #[serde(rename = "P")]
    node_public_key: Option<String>,
    #[serde(rename = "R")]
    route_rule: Option<Vec<String>>,
    #[serde(rename = "x")]
    hop: Option<u16>,

    #[serde(rename = "t")]
    manager_port: Option<u16>,
    #[serde(rename = "managerPasswd")]
    manager_passwd: Option<String>,
    #[serde(rename = "v")]
    trace: Option<EdgeTraceMode>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeRelayMode {
    #[default]
    None,
    S1,
    #[cfg(not(target_os = "windows"))]
    S2,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeEncryptMode {
    #[default]
    A1,
    A2,
    A3,
    A4,
    A5,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeZipMode {
    #[default]
    None,
    Z1,
    Z2,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeTraceMode {
    #[default]
    Normal,
    Detailed,
    Brief,
}

impl EdgeArgs {
    pub fn to_args(&self) -> std::io::Result<Vec<String>> {
        if self.group.is_empty() {
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        if self.server.is_empty() {
            return Err(std::io::ErrorKind::InvalidData.into());
        }

        self.server.parse::<Ipv4Addr>().unwrap();

        let mut args = Vec::new();

        // group
        args.push("-c".to_owned());
        args.push(self.group.clone());
        // server
        args.push("-l".to_owned());
        args.push(self.server.clone());

        if let Some(s) = self.local_port.clone() {
            args.push("-p".to_owned());
            args.push(format!("{}", s));
        }

        if let Some(s) = self.pmtu.clone() {
            if s {
                args.push("-D".to_owned());
            }
        }

        if let Some(s) = self.local_multicast.clone() {
            args.push("-e".to_owned());
            args.push(s);
        }

        if let Some(s) = self.relay_mode.clone() {
            match s {
                EdgeRelayMode::S1 => args.push("-S1".to_owned()),
                #[cfg(not(target_os = "windows"))]
                EdgeRelayMode::S2 => args.push("-S2".to_owned()),
                _ => {}
            }
        }

        if let Some(s) = self.interval.clone() {
            args.push("-i".to_owned());
            args.push(format!("{}", s));
        }

        if let Some(s) = self.ttl.clone() {
            args.push("-L".to_owned());
            args.push(format!("{}", s));
        }

        if let Some(s) = self.group_passwd.clone() {
            args.push("-k".to_owned());
            args.push(s);
        }

        if let Some(s) = self.encrypt_mode.clone() {
            match s {
                EdgeEncryptMode::A1 => args.push("-A1".to_owned()),
                EdgeEncryptMode::A2 => args.push("-A2".to_owned()),
                EdgeEncryptMode::A3 => args.push("-A3".to_owned()),
                EdgeEncryptMode::A4 => args.push("-A4".to_owned()),
                EdgeEncryptMode::A5 => args.push("-A5".to_owned()),
            }
        }

        if let Some(s) = self.head.clone() {
            if s {
                args.push("-H".to_owned());
            }
        }

        if let Some(s) = self.zip_mode.clone() {
            match s {
                EdgeZipMode::Z1 => args.push("-Z1".to_owned()),
                EdgeZipMode::Z2 => args.push("-Z1".to_owned()),
                _ => {}
            }
        }

        if let Some(s) = self.select_rtt.clone() {
            if s {
                args.push("-select-rtt".to_owned());
            }
        }

        if let Some(s) = self.select_mac.clone() {
            if s {
                args.push("-select-mac".to_owned());
            }
        }

        if let Some(s) = self.virtual_ip.clone() {
            args.push("-a".to_owned());
            args.push(s);
        }

        if let Some(s) = self.mac.clone() {
            args.push("-m".to_owned());
            args.push(s);
        }

        if let Some(s) = self.device_name.clone() {
            args.push("-d".to_owned());
            args.push(s);
        }

        if let Some(s) = self.mtu.clone() {
            args.push("-M".to_owned());
            args.push(format!("{}", s));
        }

        if let Some(s) = self.relay_data.clone() {
            if s {
                args.push("-r".to_owned());
            }
        }

        if let Some(s) = self.multicast.clone() {
            if s {
                args.push("-E".to_owned());
            }
        }

        if let Some(s) = self.desc.clone() {
            args.push("-I".to_owned());
            args.push(s);
        }

        if let Some(s) = self.node_passwd.clone() {
            args.push("-J".to_owned());
            args.push(s);
        }

        if let Some(s) = self.node_public_key.clone() {
            args.push("-P".to_owned());
            args.push(s);
        }

        if let Some(s) = self.route_rule.clone() {
            for rule in s {
                args.push("-R".to_owned());
                args.push(rule);
            }
        }

        if let Some(s) = self.hop.clone() {
            args.push("-x".to_owned());
            args.push(format!("{}", s));
        }

        if let Some(s) = self.manager_port.clone() {
            args.push("-t".to_owned());
            args.push(format!("{}", s));
        }

        if let Some(s) = self.manager_passwd.clone() {
            args.push("-manager-passwd".to_owned());
            args.push(s);
        }

        if let Some(s) = self.trace.clone() {
            match s {
                EdgeTraceMode::Detailed => args.push("-v".to_owned()),
                EdgeTraceMode::Brief => args.push("-V".to_owned()),
                _ => {}
            }
        }

        Ok(args)
    }
}
