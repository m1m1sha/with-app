pub mod config;
pub mod entity;
use vnt::core::Vnt;

pub struct With {
    pub vnt: Vnt,
}

impl With {
    fn route(&self) -> Vec<entity::RouteItem> {
        let route_table = self.vnt.route_table();
        let mut route_list = Vec::with_capacity(route_table.len());
        for (destination, route) in route_table {
            let next_hop = self
                .vnt
                .route_key(&route.route_key())
                .map_or(String::new(), |v| v.to_string());
            let metric = route.metric.to_string();

            let interface = route.addr.to_string();
            let item = entity::RouteItem {
                destination: destination.to_string(),
                next_hop,
                metric,
                interface,
                rt: route.rt,
            };
            route_list.push(item);
        }
        route_list
    }

    fn list(&self) -> Vec<entity::DeviceItem> {
        let info = self.vnt.current_device();
        let device_list = self.vnt.device_list();
        let mut list = Vec::new();
        let current_client_secret = self.vnt.client_encrypt();
        for peer in device_list {
            let name = peer.name;
            let virtual_ip = peer.virtual_ip.to_string();
            let (nat_type, public_ips, local_ip, ipv6) =
                if let Some(nat_info) = self.vnt.peer_nat_info(&peer.virtual_ip) {
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
            let (nat_traversal_type, rt) = if let Some(route) = self.vnt.route(&peer.virtual_ip) {
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
        list
    }

    fn info(&self) -> entity::Info {
        let current_device = self.vnt.current_device();
        let nat_info = self.vnt.nat_info();
        let name = self.vnt.name().to_string();
        let virtual_ip = current_device.virtual_ip().to_string();
        let virtual_gateway = current_device.virtual_gateway().to_string();
        let virtual_netmask = current_device.virtual_netmask.to_string();
        let connect_status = format!("{:?}", self.vnt.connection_status());
        let relay_server = current_device.connect_server.to_string();
        let nat_type = format!("{:?}", nat_info.nat_type);
        let public_ips: Vec<String> = nat_info.public_ips.iter().map(|v| v.to_string()).collect();
        let public_ips = public_ips.join(",");
        let local_addr = nat_info
            .local_ipv4()
            .map(|v| v.to_string())
            .unwrap_or("None".to_string());
        let ipv6_addr = nat_info
            .ipv6()
            .map(|v| v.to_string())
            .unwrap_or("None".to_string());
        entity::Info {
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
        }
    }
}
