use serde::{Deserialize, Serialize};
use vnt;
#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub virtual_ip: String,
    pub virtual_gateway: String,
    pub virtual_netmask: String,
    pub connect_status: String,
    pub relay_server: String,
    pub nat_type: String,
    pub public_ips: String,
    pub local_addr: String,
    pub ipv6_addr: String,
    pub up: u64,
    pub down: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RouteItem {
    pub destination: String,
    pub next_hop: String,
    pub metric: String,
    pub rt: i64,
    pub interface: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceItem {
    pub name: String,
    pub virtual_ip: String,
    pub nat_type: String,
    pub public_ips: String,
    pub local_ip: String,
    pub ipv6: String,
    pub nat_traversal_type: NatTraversalType,
    pub rt: i64,
    pub status: String,
    pub client_secret: bool,
    pub current_client_secret: bool,
}

/// 网络穿透状态
#[derive(Serialize, Deserialize, Debug)]
pub enum NatTraversalType {
    P2P = 0,
    P2PTcp,
    Relay,
    RelayServer,
    RelayClient,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum UseChannelType {
    Relay,
    P2p,
    All,
}

impl UseChannelType {
    pub fn to_vnt(&self) -> vnt::channel::UseChannelType {
        match self {
            UseChannelType::Relay => vnt::channel::UseChannelType::Relay,
            UseChannelType::P2p => vnt::channel::UseChannelType::P2p,
            UseChannelType::All => vnt::channel::UseChannelType::All,
        }
    }

    pub fn from_vnt(vnt: vnt::channel::UseChannelType) -> UseChannelType {
        match vnt {
            vnt::channel::UseChannelType::Relay => UseChannelType::Relay,
            vnt::channel::UseChannelType::P2p => UseChannelType::P2p,
            vnt::channel::UseChannelType::All => UseChannelType::All,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum CipherModel {
    AesGcm,
    AesCbc,
    AesEcb,
    Sm4Cbc,
    None,
}

impl CipherModel {
    pub fn to_vnt(&self) -> vnt::cipher::CipherModel {
        match self {
            CipherModel::AesGcm => vnt::cipher::CipherModel::AesGcm,
            CipherModel::AesCbc => vnt::cipher::CipherModel::AesCbc,
            CipherModel::AesEcb => vnt::cipher::CipherModel::AesEcb,
            CipherModel::Sm4Cbc => vnt::cipher::CipherModel::Sm4Cbc,
            CipherModel::None => vnt::cipher::CipherModel::None,
        }
    }

    pub fn from_vnt(vnt: vnt::cipher::CipherModel) -> CipherModel {
        match vnt {
            vnt::cipher::CipherModel::AesGcm => CipherModel::AesGcm,
            vnt::cipher::CipherModel::AesCbc => CipherModel::AesCbc,
            vnt::cipher::CipherModel::AesEcb => CipherModel::AesEcb,
            vnt::cipher::CipherModel::Sm4Cbc => CipherModel::Sm4Cbc,
            vnt::cipher::CipherModel::None => CipherModel::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum PunchModel {
    IPv4,
    IPv6,
    All,
}

impl PunchModel {
    pub fn to_vnt(&self) -> vnt::channel::punch::PunchModel {
        match self {
            PunchModel::IPv4 => vnt::channel::punch::PunchModel::IPv4,
            PunchModel::IPv6 => vnt::channel::punch::PunchModel::IPv6,
            PunchModel::All => vnt::channel::punch::PunchModel::All,
        }
    }

    pub fn from_vnt(vnt: vnt::channel::punch::PunchModel) -> PunchModel {
        match vnt {
            vnt::channel::punch::PunchModel::IPv4 => PunchModel::IPv4,
            vnt::channel::punch::PunchModel::IPv6 => PunchModel::IPv6,
            vnt::channel::punch::PunchModel::All => PunchModel::All,
        }
    }
}
