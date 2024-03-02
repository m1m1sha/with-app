use vnt::core::Vnt;

use self::entity::RouteItem;

pub mod callback;
pub mod config;
pub mod entity;

#[derive(Clone)]
pub struct With {
    pub vnt: Option<Vnt>,
    pub config: vnt::core::Config,
}

impl With {
    pub fn new(config: vnt::core::Config) -> Self {
        Self { vnt: None, config }
    }

    pub fn run(&mut self) {
        if self.status() {
            tracing::warn!("虚拟连接已启动");
            return;
        }
        let vnt_util = Vnt::new(self.config.clone(), callback::VntHandler {}).unwrap();
        self.vnt = Some(vnt_util.clone());

        tracing::info!("虚拟连接启动");
        vnt_util.wait()
    }

    pub fn stop(&mut self) {
        if let Some(vnt) = self.vnt.clone() {
            let _ = vnt.stop();
        }
    }

    pub fn status(&self) -> bool {
        self.vnt.is_some()
    }

    pub fn route(&self) -> Option<Vec<entity::RouteItem>> {
        match self.vnt.clone() {
            Some(vnt) => {
                let route_table = vnt.route_table();
                let mut route_list = Vec::with_capacity(route_table.len());
                for (destination, routes) in route_table {
                    for route in routes {
                        let next_hop = vnt
                            .route_key(&route.route_key())
                            .map_or(String::new(), |v| v.to_string());
                        let metric = route.metric.to_string();
                        let interface = route.addr.to_string();
                        let item = RouteItem {
                            destination: destination.to_string(),
                            next_hop,
                            metric,
                            rt: route.rt,
                            interface,
                        };
                        route_list.push(item);
                    }
                }
                Some(route_list)
            }
            None => None,
        }
    }

    pub fn list(&self) -> Option<Vec<entity::DeviceItem>> {
        match self.vnt.clone() {
            Some(vnt) => {
                let info = vnt.current_device();
                let device_list = vnt.device_list();
                let mut list = Vec::new();
                let current_client_secret = vnt.client_encrypt();
                for peer in device_list {
                    let name = peer.name;
                    let virtual_ip = peer.virtual_ip.to_string();
                    let (nat_type, public_ips, local_ip, ipv6) =
                        if let Some(nat_info) = vnt.peer_nat_info(&peer.virtual_ip) {
                            let nat_type = format!("{:?}", nat_info.nat_type);
                            let public_ips: Vec<String> =
                                nat_info.public_ips.iter().map(|v| v.to_string()).collect();
                            let public_ips = public_ips.join(",");
                            let local_ip = nat_info
                                .local_ipv4()
                                .map(|v| v.to_string())
                                .unwrap_or("None".to_string());
                            let ipv6 = nat_info
                                .ipv6()
                                .map(|v| v.to_string())
                                .unwrap_or("None".to_string());
                            (nat_type, public_ips, local_ip, ipv6)
                        } else {
                            (
                                "".to_string(),
                                "".to_string(),
                                "".to_string(),
                                "".to_string(),
                            )
                        };
                    let (nat_traversal_type, rt) = if let Some(route) = vnt.route(&peer.virtual_ip)
                    {
                        let nat_traversal_type = if route.metric == 1 {
                            if route.is_tcp {
                                entity::NatTraversalType::P2PTcp
                            } else {
                                entity::NatTraversalType::P2P
                            }
                        } else if route.addr == info.connect_server {
                            entity::NatTraversalType::RelayServer
                        } else {
                            entity::NatTraversalType::RelayClient
                        };

                        (nat_traversal_type, route.rt)
                    } else {
                        (entity::NatTraversalType::Relay, -1)
                    };
                    let status = format!("{:?}", peer.status);
                    let client_secret = peer.client_secret;
                    let item = entity::DeviceItem {
                        name,
                        virtual_ip,
                        nat_type,
                        public_ips,
                        local_ip,
                        ipv6,
                        nat_traversal_type,
                        rt,
                        status,
                        client_secret,
                        current_client_secret,
                    };
                    list.push(item);
                }
                Some(list)
            }
            None => None,
        }
    }

    pub fn info(&self) -> Option<entity::Info> {
        match self.vnt.clone() {
            Some(vnt) => {
                let current_device = vnt.current_device();
                let nat_info = vnt.nat_info();
                let name = vnt.name().to_string();
                let virtual_ip = current_device.virtual_ip().to_string();
                let virtual_gateway = current_device.virtual_gateway().to_string();
                let virtual_netmask = current_device.virtual_netmask.to_string();
                let connect_status = format!("{:?}", vnt.connection_status());
                let relay_server = current_device.connect_server.to_string();
                let nat_type = format!("{:?}", nat_info.nat_type);
                let public_ips: Vec<String> =
                    nat_info.public_ips.iter().map(|v| v.to_string()).collect();
                let public_ips = public_ips.join(",");
                let local_addr = nat_info
                    .local_ipv4()
                    .map(|v| v.to_string())
                    .unwrap_or("None".to_string());
                let ipv6_addr = nat_info
                    .ipv6()
                    .map(|v| v.to_string())
                    .unwrap_or("None".to_string());
                let up = vnt.up_stream();
                let down = vnt.down_stream();
                Some(entity::Info {
                    name,
                    virtual_ip,
                    virtual_gateway,
                    virtual_netmask,
                    connect_status,
                    relay_server,
                    nat_type,
                    public_ips,
                    local_addr,
                    ipv6_addr,
                    up,
                    down,
                })
            }
            None => None,
        }
    }
}
