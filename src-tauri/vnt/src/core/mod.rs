use std::collections::HashMap;
use std::io;
use std::net::UdpSocket;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::Duration;

use crossbeam_utils::atomic::AtomicCell;
use parking_lot::{Mutex, RwLock};
use rand::Rng;
use tokio::sync::mpsc::channel;

use crate::channel::channel::{Channel, Context};
use crate::channel::idle::Idle;
use crate::channel::punch::{NatInfo, Punch, PunchModel};
use crate::channel::sender::ChannelSender;
use crate::channel::{Route, RouteKey};
use crate::cipher::{Cipher, CipherModel, RsaCipher};
use crate::core::status::VntStatusManger;
use crate::error::Error;
use crate::external_route::{AllowExternalRoute, ExternalRoute};
use crate::handle::handshake_handler::HandshakeEnum;
use crate::handle::recv_handler::ChannelDataHandler;
use crate::handle::registration_handler::{RegResponse, ReqEnum};
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
use crate::handle::tun_tap::tap_handler;
use crate::handle::tun_tap::tun_handler;
use crate::handle::{
    handshake_handler, heartbeat_handler, punch_handler, registration_handler, ConnectStatus,
    CurrentDeviceInfo, PeerDeviceInfo,
};
use crate::igmp_server::IgmpServer;
use crate::nat::NatTest;
use crate::tun_tap_device;
use crate::tun_tap_device::{DeviceReader, DeviceWriter};

pub mod status;
pub mod sync;

#[derive(Clone)]
pub struct Vnt {
    config: Config,
    current_device: Arc<AtomicCell<CurrentDeviceInfo>>,
    context: Context,
    vnt_status_manager: VntStatusManger,
    device_writer: DeviceWriter,
    /// 0. 机器纪元，每一次上线或者下线都会增1，用于感知网络中机器变化
    /// 服务端和客户端的不一致，则服务端会推送新的设备列表
    /// 1. 网络中的虚拟ip列表
    device_list: Arc<Mutex<(u16, Vec<PeerDeviceInfo>)>>,
    nat_test: NatTest,
    connect_status: Arc<AtomicCell<ConnectStatus>>,
    peer_nat_info_map: Arc<RwLock<HashMap<Ipv4Addr, NatInfo>>>,
}

pub struct VntUtil {
    config: Config,
    main_channel: UdpSocket,
    main_tcp_channel: Option<TcpStream>,
    response: Option<RegResponse>,
    iface: Option<(DeviceWriter, DeviceReader)>,
    server_cipher: Cipher,
    rsa_cipher: Option<RsaCipher>,
}

impl VntUtil {
    pub fn new(config: Config) -> io::Result<VntUtil> {
        let address: SocketAddr = format!("[::]:{}", config.port).parse().unwrap();
        //单个udp用同步的性能更好，但是代理和多端口监听用异步更方便，这里将两者结合起来
        let socket = socket2::Socket::new(socket2::Domain::IPV6, socket2::Type::DGRAM, None)?;
        socket.set_only_v6(false)?;
        socket.bind(&address.into())?;
        let main_channel: UdpSocket = socket.into();
        main_channel.set_write_timeout(Some(Duration::from_secs(5)))?;
        main_channel.set_read_timeout(Some(Duration::from_secs(2)))?;
        let server_cipher = if config.server_encrypt {
            let mut key = [0u8; 32];
            rand::thread_rng().fill(&mut key);
            Cipher::new_key(key, config.token.clone())?
        } else {
            Cipher::None
        };
        Ok(VntUtil {
            config,
            main_channel,
            main_tcp_channel: None,
            response: None,
            iface: None,
            server_cipher,
            rsa_cipher: None,
        })
    }
    ///链接
    pub fn connect(&mut self) -> io::Result<()> {
        if self.config.tcp {
            let tcp = TcpStream::connect(self.config.server_address)?;
            tcp.set_read_timeout(Some(Duration::from_secs(10)))?;
            let _ = self.main_tcp_channel.insert(tcp);
        }
        Ok(())
    }

    ///握手 用于获取公钥
    pub fn handshake(&mut self) -> Result<Option<RsaCipher>, HandshakeEnum> {
        let rsa_cipher = handshake_handler::handshake(
            &self.main_channel,
            self.main_tcp_channel.as_mut(),
            self.config.server_address,
            self.config.server_encrypt,
        )?;
        self.rsa_cipher = rsa_cipher.clone();
        Ok(rsa_cipher)
    }
    /// 加密握手 用于同步密钥
    pub fn secret_handshake(&mut self) -> Result<(), HandshakeEnum> {
        handshake_handler::secret_handshake(
            &self.main_channel,
            self.main_tcp_channel.as_mut(),
            self.config.server_address,
            self.rsa_cipher.as_ref().unwrap(),
            &self.server_cipher,
            self.config.token.clone(),
        )
    }
    /// 注册
    pub fn register(&mut self) -> Result<RegResponse, ReqEnum> {
        match registration_handler::registration(
            &self.main_channel,
            self.main_tcp_channel.as_mut(),
            &self.server_cipher,
            self.config.server_address,
            self.config.token.clone(),
            self.config.device_id.clone(),
            self.config.name.clone(),
            self.config.ip.unwrap_or(Ipv4Addr::UNSPECIFIED),
            self.config.password.is_some(),
        ) {
            Ok(res) => {
                let _ = self.response.insert(res.clone());
                Ok(res)
            }
            Err(e) => Err(e),
        }
    }
    #[cfg(any(target_os = "android"))]
    pub fn create_iface(&mut self, vpn_fd: i32) {
        let (device_writer, device_reader) = tun_tap_device::create(vpn_fd);
        let _ = self.iface.insert((device_writer, device_reader));
    }
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    pub fn create_iface(&mut self) -> io::Result<tun_tap_device::DriverInfo> {
        if self.iface.is_some() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
        let response = match &self.response {
            None => {
                return Err(io::Error::from(io::ErrorKind::AlreadyExists));
            }
            Some(res) => res,
        };
        let device_type = if self.config.tap {
            {
                //删除tun网卡避免ip冲突，因为非正常退出会保留网卡
                tun_tap_device::delete_device(tun_tap_device::DeviceType::Tun);
            }
            tun_tap_device::DeviceType::Tap
        } else {
            {
                //删除tap网卡避免ip冲突，非正常退出会保留网卡
                tun_tap_device::delete_device(tun_tap_device::DeviceType::Tap);
            }
            tun_tap_device::DeviceType::Tun
        };
        let mtu = match self.config.mtu {
            None => {
                if self.config.password.is_none() {
                    1450
                } else {
                    1410
                }
            }
            Some(mtu) => mtu,
        };
        let in_ips = self
            .config
            .in_ips
            .iter()
            .map(|(dest, mask, _)| (Ipv4Addr::from(*dest & *mask), Ipv4Addr::from(*mask)))
            .collect::<Vec<(Ipv4Addr, Ipv4Addr)>>();

        let (device_writer, device_reader, driver_info) = tun_tap_device::create_device(
            device_type,
            response.virtual_ip,
            response.virtual_netmask,
            response.virtual_gateway,
            in_ips,
            mtu,
        )?;
        let _ = self.iface.insert((device_writer, device_reader));
        Ok(driver_info)
    }
    pub async fn build(self) -> crate::Result<Vnt> {
        //将读的超时时间清空
        self.main_channel.set_read_timeout(None)?;
        let response = match self.response {
            None => {
                return Err(Error::Stop("response None".to_string()));
            }
            Some(res) => res,
        };
        let (device_writer, device_reader) = match self.iface {
            None => {
                return Err(Error::Stop("iface None".to_string()));
            }
            Some(res) => res,
        };
        let config = self.config.clone();
        let vnt_status_manager = VntStatusManger::new();
        let finger = if config.finger {
            Some(config.token.clone())
        } else {
            None
        };
        let client_cipher =
            Cipher::new_password(config.cipher_model, config.password.clone(), finger);
        let virtual_ip = response.virtual_ip;
        let virtual_gateway = response.virtual_gateway;
        let virtual_netmask = response.virtual_netmask;
        let current_device = Arc::new(AtomicCell::new(CurrentDeviceInfo::new(
            virtual_ip,
            virtual_gateway,
            virtual_netmask,
            config.server_address,
        )));

        let (cone_sender, cone_receiver) = channel(3);
        let (symmetric_sender, symmetric_receiver) = channel(2);
        let (tcp_sender, tcp_receiver) = if let Some(main_tcp_channel) = self.main_tcp_channel {
            (Some(main_tcp_channel.try_clone()?), Some(main_tcp_channel))
        } else {
            (None, None)
        };
        let tcp_listener = TcpListener::bind(format!("[::]:{}", config.port))?;
        let local_tcp_port = tcp_listener.local_addr()?.port();
        let context = Context::new(
            self.main_channel,
            tcp_sender,
            current_device.clone(),
            1,
            config.first_latency,
            local_tcp_port,
        );
        let idle = Idle::new(Duration::from_secs(16), context.clone());
        let channel_sender = ChannelSender::new(context.clone());

        let register = Arc::new(registration_handler::Register::new(
            self.server_cipher.clone(),
            channel_sender.clone(),
            config.server_address,
            config.token.clone(),
            config.device_id.clone(),
            config.name.clone(),
            config.password.is_some(),
        ));
        let device_list: Arc<Mutex<(u16, Vec<PeerDeviceInfo>)>> =
            Arc::new(Mutex::new((response.epoch, response.device_info_list)));
        let peer_nat_info_map: Arc<RwLock<HashMap<Ipv4Addr, NatInfo>>> =
            Arc::new(RwLock::new(HashMap::with_capacity(16)));
        let connect_status = Arc::new(AtomicCell::new(ConnectStatus::Connected));
        let public_ip = response.public_ip;
        let public_port = response.public_port;
        let local_udp_port = context.main_local_udp_port().unwrap_or(0);
        let local_ipv4 = crate::nat::local_ipv4();
        let ipv6 = crate::nat::local_ipv6();

        // NAT检测
        let nat_test = NatTest::new(
            config.stun_server.clone(),
            public_ip,
            public_port,
            local_ipv4,
            ipv6,
            local_udp_port,
            local_tcp_port,
        );
        let in_external_route = if config.in_ips.is_empty() {
            None
        } else {
            Some(ExternalRoute::new(config.in_ips))
        };
        #[cfg(feature = "ip_proxy")]
        let (tcp_proxy, udp_proxy, ip_proxy_map) = if config.out_ips.is_empty() || config.no_proxy {
            (None, None, None)
        } else {
            let (tcp_proxy, udp_proxy, ip_proxy_map) = crate::ip_proxy::init_proxy(
                #[cfg(not(target_os = "android"))]
                channel_sender.clone(),
                #[cfg(not(target_os = "android"))]
                current_device.clone(),
                #[cfg(not(target_os = "android"))]
                client_cipher.clone(),
            )
            .await?;
            (Some(tcp_proxy), Some(udp_proxy), Some(ip_proxy_map))
        };
        let out_external_route = AllowExternalRoute::new(config.out_ips);

        let igmp_server = if config.simulate_multicast {
            Some(IgmpServer::new(device_writer.clone()))
        } else {
            None
        };
        #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
        if config.tap {
            tap_handler::start(
                vnt_status_manager.worker("tap_handler"),
                channel_sender.clone(),
                device_reader,
                device_writer.clone(),
                igmp_server.clone(),
                current_device.clone(),
                in_external_route,
                #[cfg(feature = "ip_proxy")]
                ip_proxy_map.clone(),
                client_cipher.clone(),
                self.server_cipher.clone(),
                config.parallel,
            );
        } else {
            tun_handler::start(
                vnt_status_manager.worker("tun_handler"),
                channel_sender.clone(),
                device_reader,
                device_writer.clone(),
                igmp_server.clone(),
                current_device.clone(),
                in_external_route,
                #[cfg(feature = "ip_proxy")]
                ip_proxy_map.clone(),
                client_cipher.clone(),
                self.server_cipher.clone(),
                config.parallel,
            );
        }
        #[cfg(any(target_os = "android"))]
        tun_handler::start(
            vnt_status_manager.worker("android tun_handler"),
            channel_sender.clone(),
            device_reader,
            device_writer.clone(),
            igmp_server.clone(),
            current_device.clone(),
            in_external_route,
            #[cfg(feature = "ip_proxy")]
            ip_proxy_map.clone(),
            client_cipher.clone(),
            self.server_cipher.clone(),
            config.parallel,
        );

        //外部数据接收处理
        let channel_recv_handler = ChannelDataHandler::new(
            current_device.clone(),
            device_list.clone(),
            register.clone(),
            nat_test.clone(),
            igmp_server,
            device_writer.clone(),
            connect_status.clone(),
            peer_nat_info_map.clone(),
            #[cfg(feature = "ip_proxy")]
            ip_proxy_map,
            out_external_route,
            cone_sender,
            symmetric_sender,
            client_cipher.clone(),
            self.server_cipher.clone(),
            self.rsa_cipher.clone(),
            config.relay,
            config.token.clone(),
            14,
        );
        let punch = Punch::new(
            context.clone(),
            config.punch_model,
            config.tcp,
            channel_recv_handler.clone(),
        );
        {
            let channel = Channel::new(context.clone(), channel_recv_handler, tcp_listener);
            let channel_worker = vnt_status_manager.worker("channel_worker");
            let relay = config.relay;
            tokio::spawn(
                async move { channel.start(channel_worker, tcp_receiver, 65, relay).await },
            );
        }
        {
            let nat_test = nat_test.clone();
            let device_list = device_list.clone();
            let current_device = current_device.clone();
            // 定时心跳
            heartbeat_handler::start_heartbeat_main(
                vnt_status_manager.worker("main-heartbeat"),
                channel_sender.clone(),
                device_list.clone(),
                current_device.clone(),
                config.server_address_str,
                client_cipher.clone(),
                self.server_cipher.clone(),
            );
            heartbeat_handler::start_heartbeat(
                vnt_status_manager.worker("heartbeat"),
                channel_sender.clone(),
                device_list.clone(),
                current_device.clone(),
                client_cipher.clone(),
                self.server_cipher.clone(),
            );
            // 空闲检查
            heartbeat_handler::start_idle(
                vnt_status_manager.worker("idle"),
                idle,
                channel_sender.clone(),
            );
            if !config.relay {
                // 打洞处理
                punch_handler::start(
                    vnt_status_manager.worker("cone_receiver"),
                    cone_receiver,
                    punch.clone(),
                    current_device.clone(),
                    client_cipher.clone(),
                );
                punch_handler::start(
                    vnt_status_manager.worker("symmetric_receiver"),
                    symmetric_receiver,
                    punch,
                    current_device.clone(),
                    client_cipher.clone(),
                );
                tokio::spawn(punch_handler::start_punch(
                    vnt_status_manager.worker("punch_handler"),
                    nat_test,
                    device_list,
                    channel_sender,
                    current_device,
                    client_cipher.clone(),
                ));
            }
        }
        #[cfg(feature = "ip_proxy")]
        {
            //代理
            if let Some(tcp_proxy) = tcp_proxy {
                tokio::spawn(tcp_proxy.start());
            }
            if let Some(udp_proxy) = udp_proxy {
                tokio::spawn(udp_proxy.start());
            }
        }
        {
            let context = context.clone();
            let nat_test = nat_test.clone();
            tokio::spawn(async move {
                let info = nat_test
                    .re_test(
                        public_ip,
                        public_port,
                        local_ipv4,
                        ipv6,
                        local_udp_port,
                        local_tcp_port,
                    )
                    .await;
                context.switch(info.nat_type);
            });
        }
        Ok(Vnt {
            config: self.config,
            current_device,
            context,
            vnt_status_manager,
            device_writer,
            nat_test,
            device_list,
            connect_status,
            peer_nat_info_map,
        })
    }
}

impl Vnt {
    pub fn name(&self) -> &str {
        &self.config.name
    }
    pub fn server_encrypt(&self) -> bool {
        self.config.server_encrypt
    }
    pub fn client_encrypt(&self) -> bool {
        self.config.password.is_some()
    }
    pub fn current_device(&self) -> CurrentDeviceInfo {
        self.current_device.load()
    }
    pub fn peer_nat_info(&self, ip: &Ipv4Addr) -> Option<NatInfo> {
        self.peer_nat_info_map.read().get(ip).cloned()
    }
    pub fn connection_status(&self) -> ConnectStatus {
        self.connect_status.load()
    }
    pub fn nat_info(&self) -> NatInfo {
        self.nat_test.nat_info()
    }
    pub fn device_list(&self) -> Vec<PeerDeviceInfo> {
        let device_list_lock = self.device_list.lock();
        let (_epoch, device_list) = device_list_lock.clone();
        drop(device_list_lock);
        device_list
    }
    pub fn route(&self, ip: &Ipv4Addr) -> Option<Route> {
        self.context.route_one(ip)
    }
    pub fn route_key(&self, route_key: &RouteKey) -> Option<Ipv4Addr> {
        self.context.route_to_id(route_key)
    }
    pub fn route_table(&self) -> Vec<(Ipv4Addr, Route)> {
        self.context.route_table_one()
    }
    pub fn stop(&self) -> io::Result<()> {
        let _ = self.context.close();
        self.vnt_status_manager.stop_all();
        let _ = self.device_writer.close();
        let virtual_gateway = self.current_device.load().virtual_gateway;
        let _ = UdpSocket::bind("0.0.0.0:0")?.send_to(
            b"stop",
            SocketAddr::V4(SocketAddrV4::new(virtual_gateway, 10000)),
        );

        Ok(())
    }
    pub async fn wait_stop(&mut self) {
        self.vnt_status_manager.wait().await;
        let _ = self.stop();
    }
    pub async fn wait_stop_ms(&mut self, ms: Duration) -> bool {
        tokio::select! {
            _=self.vnt_status_manager.wait()=>{
                let _ = self.stop();
                return true;
            }
            _=tokio::time::sleep(ms)=>{
                return false;
            }
        }
    }
}

impl Drop for Vnt {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub tap: bool,
    pub token: String,
    pub device_id: String,
    pub name: String,
    pub server_address: SocketAddr,
    pub server_address_str: String,
    pub stun_server: Vec<String>,
    pub in_ips: Vec<(u32, u32, Ipv4Addr)>,
    pub out_ips: Vec<(u32, u32)>,
    pub password: Option<String>,
    pub simulate_multicast: bool,
    pub mtu: Option<u16>,
    pub tcp: bool,
    pub ip: Option<Ipv4Addr>,
    pub relay: bool,
    #[cfg(feature = "ip_proxy")]
    pub no_proxy: bool,
    pub server_encrypt: bool,
    pub parallel: usize,
    pub cipher_model: CipherModel,
    pub finger: bool,
    pub punch_model: PunchModel,
    pub port: u16,
    pub first_latency: bool,
}

impl Config {
    pub fn new(
        tap: bool,
        token: String,
        device_id: String,
        name: String,
        server_address: SocketAddr,
        server_address_str: String,
        mut stun_server: Vec<String>,
        in_ips: Vec<(u32, u32, Ipv4Addr)>,
        out_ips: Vec<(u32, u32)>,
        password: Option<String>,
        simulate_multicast: bool,
        mtu: Option<u16>,
        tcp: bool,
        ip: Option<Ipv4Addr>,
        relay: bool,
        #[cfg(feature = "ip_proxy")] no_proxy: bool,
        server_encrypt: bool,
        parallel: usize,
        cipher_model: CipherModel,
        finger: bool,
        punch_model: PunchModel,
        port: u16,
        first_latency: bool,
    ) -> Result<Self, Error> {
        for x in stun_server.iter_mut() {
            if !x.contains(":") {
                x.push_str(":3478");
            }
        }
        if token.is_empty() || token.len() > 128 {
            return Err(Error::Stop(String::from("token too long")));
        }
        if device_id.is_empty() || device_id.len() > 128 {
            return Err(Error::Stop(String::from("device_id too long")));
        }
        if name.is_empty() || name.len() > 128 {
            return Err(Error::Stop(String::from("name too long")));
        }
        Ok(Self {
            tap,
            token,
            device_id,
            name,
            server_address,
            server_address_str,
            stun_server,
            in_ips,
            out_ips,
            password,
            simulate_multicast,
            mtu,
            tcp,
            ip,
            relay,
            #[cfg(feature = "ip_proxy")]
            no_proxy,
            server_encrypt,
            parallel,
            cipher_model,
            finger,
            punch_model,
            port,
            first_latency,
        })
    }
}
