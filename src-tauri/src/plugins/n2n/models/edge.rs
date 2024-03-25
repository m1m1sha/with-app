use crate::{plugins::n2n::error::N2nError, utils::serde::deserialize_i32_to_bool};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RespError {
    /// 授权错误
    BadAuth,
    /// 错误的方法 action: r|w|s
    BadType,

    /// 无法访问 报头加密已开启
    NoAccess,
    /// 命令不存在
    NoCmd,
    /// 选项不存在
    NoOptions,
    /// 方法不存在
    NoType,

    /// 只读
    ReadOnly,

    /// 未实现
    UnImplemented,
    /// 未知命令
    UnknownCmd,
    /// 未知细目
    UnknownTopic,
}

impl RespError {
    pub fn from(str: &str) -> Self {
        // 正常情况下不可能有其他错误类型
        match str {
            "badauth" => RespError::BadAuth,
            "badtype" => RespError::BadType,
            "noaccess" => RespError::NoAccess,
            "nocmd" => RespError::NoCmd,
            "nooptions" => RespError::NoOptions,
            "notype" => RespError::NoType,
            "readonly" => RespError::ReadOnly,
            "unknowncmd" => RespError::UnknownCmd,
            _ => RespError::UnImplemented,
        }
    }

    pub fn to_err(&self) -> N2nError {
        match self {
            RespError::BadAuth => N2nError::BadAuth,
            RespError::BadType => N2nError::BadType,
            RespError::NoAccess => N2nError::NoAccess,
            RespError::NoCmd => N2nError::NoCmd,
            RespError::NoOptions => N2nError::NoOptions,
            RespError::NoType => N2nError::NoType,
            RespError::ReadOnly => N2nError::ReadOnly,
            RespError::UnImplemented => N2nError::UnImplemented,
            RespError::UnknownCmd => N2nError::UnknownCmd,
            RespError::UnknownTopic => N2nError::UnknownTopic,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stop {
    #[serde(rename(serialize = "running", deserialize = "keep_running"))]
    #[serde(deserialize_with = "deserialize_i32_to_bool")]
    pub status: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Verbose {
    #[serde(rename = "traceLevel")]
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Community {
    #[serde(rename(deserialize = "community"))]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct EdgeInfo {
    pub mode: String,
    #[serde(rename(deserialize = "ip4addr"))]
    pub ipv4: String,
    #[serde(deserialize_with = "deserialize_i32_to_bool")]
    pub purgeable: bool,
    #[serde(deserialize_with = "deserialize_i32_to_bool")]
    pub local: bool,
    #[serde(rename(deserialize = "macaddr"))]
    pub mac: String,
    #[serde(rename(deserialize = "sockaddr", serialize = "addr"))]
    pub socket: String,
    pub desc: String,
    pub last_p2p: u64,
    pub last_sent_query: u64,
    pub last_seen: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct SupernodeInfo {
    pub version: String,
    #[serde(deserialize_with = "deserialize_i32_to_bool")]
    pub purgeable: bool,
    #[serde(deserialize_with = "deserialize_i32_to_bool")]
    pub current: bool,
    #[serde(rename(deserialize = "macaddr"))]
    pub mac: String,
    #[serde(rename(deserialize = "sockaddr", serialize = "addr"))]
    pub socket: String,
    pub selection: String,
    pub last_seen: u64,
    pub uptime: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Timestamps {
    pub start_time: u64,
    pub last_super: u64,
    pub last_p2p: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PacketStatsPkt {
    pub tx_pkt: u32,
    pub rx_pkt: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct PacketStats {
    pub transport: PacketStatsPkt,
    pub p2p: PacketStatsPkt,
    pub supernode: PacketStatsPkt,
    pub supernode_broadcast: PacketStatsPkt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PacketStatsType {
    #[serde(rename = "transop")]
    Transport { tx_pkt: u32, rx_pkt: u32 },
    #[serde(rename = "p2p")]
    P2P { tx_pkt: u32, rx_pkt: u32 },
    #[serde(rename = "super")]
    Supernode { tx_pkt: u32, rx_pkt: u32 },
    #[serde(rename = "super_broadcast")]
    SupernodeBroadcast { tx_pkt: u32, rx_pkt: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeFlagPayload {
    Stop(Stop),
    Status(Stop),
    Verbose(Verbose),
    Timestamps(Timestamps),
    Community(Community),
    SupernodeInfo(SupernodeInfo),
    PacketStats(PacketStats),
    EdgeInfo(Vec<EdgeInfo>),
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeFlag {
    Stop,
    Status,
    Verbose,
    Timestamps,
    Community,
    SupernodeInfo,
    PacketStats,
    EdgeInfo,
}
