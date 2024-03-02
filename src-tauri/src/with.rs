pub mod config;
pub mod entity;

use vnt::{
    core::{Vnt, VntUtil},
    handle::{
        handshake_handler::HandshakeEnum,
        registration_handler::{RegResponse, ReqEnum},
    },
    tun_tap_device::DriverInfo,
};

#[derive(Clone)]
pub struct With {
    pub vnt: Option<Vnt>,
    pub config: vnt::core::Config,
    pub reg: Option<RegResponse>,
    pub driver: Option<DriverInfo>,
}

impl With {
    fn new(&self, config: vnt::core::Config) -> Self {
        Self {
            vnt: None,
            config,
            reg: None,
            driver: None,
        }
    }

    async fn start(&mut self) {
        if self.status() {
            tracing::warn!("虚拟连接已启动");
            return;
        }

        let server_encrypt = self.config.server_encrypt;
        let mut with_util = VntUtil::new(self.config.clone()).unwrap();
        let mut conn_count = 0;
        // 创建连接
        self.reg = Some(loop {
            if conn_count > 0 {
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            }

            conn_count += 1;

            if let Err(e) = with_util.connect() {
                tracing::warn!("连接服务器失败 {}", e);
                return;
            }
            match with_util.handshake() {
                Ok(response) => {
                    if server_encrypt {
                        let finger = response.unwrap().finger().unwrap();
                        tracing::info!("服务器指纹: {}", finger);
                        match with_util.secret_handshake() {
                            Ok(_) => {}
                            Err(e) => {
                                match e {
                                    HandshakeEnum::NotSecret => {}
                                    HandshakeEnum::KeyError => {}
                                    HandshakeEnum::Timeout => {
                                        tracing::warn!("握手超时");
                                    }
                                    HandshakeEnum::ServerError(str) => {
                                        tracing::warn!("服务器发生错误: {}", str);
                                    }
                                    HandshakeEnum::Other(str) => {
                                        tracing::warn!("发生未知错误: {}", str);
                                    }
                                }
                                continue;
                            }
                        }
                    }
                    match with_util.register() {
                        Ok(response) => {
                            break response;
                        }
                        Err(e) => match e {
                            ReqEnum::TokenError => {
                                tracing::error!("token 错误");
                                return;
                            }
                            ReqEnum::AddressExhausted => {
                                tracing::error!("地址用尽");
                                return;
                            }
                            ReqEnum::Timeout => {
                                tracing::warn!("超时...");
                            }
                            ReqEnum::ServerError(str) => {
                                tracing::warn!("服务器发生错误: {}", str);
                            }
                            ReqEnum::Other(str) => {
                                tracing::warn!("发生未知错误: {}", str);
                            }
                            ReqEnum::IpAlreadyExists => {
                                tracing::error!("IP已经存在");
                                return;
                            }
                            ReqEnum::InvalidIp => {
                                tracing::error!("未校验的IP");
                                return;
                            }
                        },
                    }
                }
                Err(e) => match e {
                    HandshakeEnum::NotSecret => {
                        tracing::error!("该服务器不支持加密");
                        return;
                    }
                    HandshakeEnum::KeyError => {}
                    HandshakeEnum::Timeout => {
                        tracing::warn!("握手超时");
                    }
                    HandshakeEnum::ServerError(str) => {
                        tracing::error!("服务器发生错误: {}", str);
                    }
                    HandshakeEnum::Other(str) => {
                        tracing::error!("发生未知错误: {}", str);
                    }
                },
            }
        });

        // 创建虚拟网卡
        self.driver = Some(match with_util.create_iface() {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("虚拟网卡创建失败: {}", e);
                return;
            }
        });

        self.vnt = match with_util.build().await {
            Ok(vnt) => Some(vnt),
            Err(e) => {
                tracing::error!("虚拟连接启动失败: {}", e);
                return;
            }
        };

        tracing::info!("虚拟连接创建成功");

        if let Some(mut vnt) = self.vnt.clone() {
            vnt.wait_stop().await;
        }

        tracing::info!("虚拟连接已停止");
    }

    fn stop(&mut self) {
        if let Some(vnt) = self.vnt.clone() {
            let _ = vnt.stop();
        }
        self.driver = None;
        self.reg = None;
    }

    fn status(&self) -> bool {
        self.vnt.is_some()
    }

    fn route(&self) -> Option<Vec<entity::RouteItem>> {
        match self.vnt.clone() {
            Some(vnt) => {
                let route_table = vnt.route_table();
                let mut route_list = Vec::with_capacity(route_table.len());
                for (destination, route) in route_table {
                    let next_hop = vnt
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
                Some(route_list)
            }
            None => None,
        }
    }

    fn list(&self) -> Option<Vec<entity::DeviceItem>> {
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

    fn info(&self) -> Option<entity::Info> {
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
                })
            }
            None => None,
        }
    }
}
