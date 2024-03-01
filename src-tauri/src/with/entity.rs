use serde::{Deserialize, Serialize};

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