use std::io;
use std::net::{Ipv4Addr, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

use crossbeam_utils::atomic::AtomicCell;
use parking_lot::Mutex;
use rand::prelude::SliceRandom;

use crate::channel::idle::Idle;
use crate::channel::sender::ChannelSender;
use crate::channel::Route;
use crate::cipher::Cipher;
use crate::core::status::VntWorker;
use crate::handle::{CurrentDeviceInfo, PeerDeviceInfo};
use crate::protocol::body::ENCRYPTION_RESERVED;
use crate::protocol::control_packet::PingPacket;
use crate::protocol::{control_packet, NetPacket, Protocol, Version, MAX_TTL};

pub fn start_idle(mut worker: VntWorker, idle: Idle, sender: ChannelSender) {
    tokio::spawn(async move {
        tokio::select! {
             _=worker.stop_wait()=>{
                    return;
             }
             rs=start_idle_(idle, sender)=>{
                 if let Err(e) = rs {
                    log::warn!("空闲检测任务停止:{:?}", e);
                }
            }
        }
        worker.stop_all();
    });
}

async fn start_idle_(idle: Idle, sender: ChannelSender) -> io::Result<()> {
    log::info!("启动空闲检查任务");
    loop {
        let (peer_ip, route) = idle.next_idle().await?;
        log::info!("路由空闲 peer_ip:{:?},route:{:?}", peer_ip, route);
        sender.remove_route(&peer_ip, route);
    }
}

pub fn start_heartbeat(
    mut worker: VntWorker,
    sender: ChannelSender,
    device_list: Arc<Mutex<(u16, Vec<PeerDeviceInfo>)>>,
    current_device: Arc<AtomicCell<CurrentDeviceInfo>>,
    client_cipher: Cipher,
    server_cipher: Cipher,
) {
    tokio::spawn(async move {
        tokio::select! {
             _=worker.stop_wait()=>{
                    return;
             }
            rs=start_heartbeat_(sender, device_list, current_device,client_cipher,server_cipher)=>{
                if let Err(e) = rs {
                    log::warn!("心跳任务停止:{:?}", e);
                }
            }
        }
        worker.stop_all();
    });
}

pub fn start_heartbeat_main(
    mut worker: VntWorker,
    sender: ChannelSender,
    device_list: Arc<Mutex<(u16, Vec<PeerDeviceInfo>)>>,
    current_device: Arc<AtomicCell<CurrentDeviceInfo>>,
    server_address_str: String,
    client_cipher: Cipher,
    server_cipher: Cipher,
) {
    tokio::spawn(async move {
        tokio::select! {
             _=worker.stop_wait()=>{
                    return;
             }
             rs=start_heartbeat_main_(sender, device_list, current_device,server_address_str,client_cipher,server_cipher)=>{
                if let Err(e) = rs {
                    log::warn!("主心跳任务停止:{:?}", e);
                }
             }
        }
        worker.stop_all();
    });
}

fn heartbeat_packet(
    ttl: u8,
    device_list: &Mutex<(u16, Vec<PeerDeviceInfo>)>,
    client_cipher: &Cipher,
    server_cipher: &Cipher,
    gateway: bool,
    src: Ipv4Addr,
    dest: Ipv4Addr,
) -> NetPacket<[u8; 12 + 4 + ENCRYPTION_RESERVED]> {
    let mut net_packet = NetPacket::new_encrypt([0u8; 12 + 4 + ENCRYPTION_RESERVED]).unwrap();
    net_packet.set_version(Version::V1);
    net_packet.set_protocol(Protocol::Control);
    net_packet.set_transport_protocol(control_packet::Protocol::Ping.into());
    net_packet.first_set_ttl(ttl);
    net_packet.set_source(src);
    net_packet.set_destination(dest);
    {
        let mut ping = PingPacket::new(net_packet.payload_mut()).unwrap();
        let epoch = { device_list.lock().0 };
        ping.set_epoch(epoch);
        ping.set_time(crate::handle::now_time() as u16);
    }
    if gateway {
        net_packet.set_gateway_flag(true);
        server_cipher.encrypt_ipv4(&mut net_packet).unwrap();
    } else {
        client_cipher.encrypt_ipv4(&mut net_packet).unwrap();
    }
    net_packet
}

async fn start_heartbeat_main_(
    sender: ChannelSender,
    device_list: Arc<Mutex<(u16, Vec<PeerDeviceInfo>)>>,
    current_device: Arc<AtomicCell<CurrentDeviceInfo>>,
    server_address_str: String,
    client_cipher: Cipher,
    server_cipher: Cipher,
) -> io::Result<()> {
    let mut count = 0;
    log::info!("启动主心跳任务");
    loop {
        if sender.is_close() {
            return Ok(());
        }
        let mut current_dev = current_device.load();
        let src = current_dev.virtual_ip();
        if count % 40 == 19 {
            if let Ok(mut addr) = server_address_str.to_socket_addrs() {
                if let Some(addr) = addr.next() {
                    if addr != current_dev.connect_server {
                        let mut tmp = current_dev.clone();
                        tmp.connect_server = addr;
                        log::info!(
                            "服务端地址变化,旧地址:{}，新地址:{}",
                            current_dev.connect_server,
                            addr
                        );
                        if current_device.compare_exchange(current_dev, tmp).is_ok() {
                            current_dev.connect_server = addr;
                        }
                    }
                }
            }
        }
        let server_packet = heartbeat_packet(
            MAX_TTL,
            &device_list,
            &client_cipher,
            &server_cipher,
            true,
            src,
            current_dev.virtual_gateway,
        );
        if let Err(e) = sender.send_main(server_packet.buffer(), current_dev.connect_server) {
            log::warn!("connect_server:{:?},e:{:?}", current_dev.connect_server, e);
        }
        count += 1;
        tokio::time::sleep(Duration::from_millis(3000)).await;
    }
}

async fn start_heartbeat_(
    sender: ChannelSender,
    device_list: Arc<Mutex<(u16, Vec<PeerDeviceInfo>)>>,
    current_device: Arc<AtomicCell<CurrentDeviceInfo>>,
    client_cipher: Cipher,
    server_cipher: Cipher,
) -> io::Result<()> {
    let mut count = 0;
    log::info!("启动心跳任务");
    loop {
        if sender.is_close() {
            return Ok(());
        }
        let current_dev = current_device.load();
        //如果和服务端使用tcp连接，则维持udp洞的频率要更高些
        if (sender.is_main_tcp() && count % 4 == 0) || (!sender.is_main_tcp() && count % 40 == 1) {
            let mut packet = NetPacket::new_encrypt([0; 12 + ENCRYPTION_RESERVED])?;
            packet.set_version(Version::V1);
            packet.set_gateway_flag(true);
            packet.set_protocol(Protocol::Control);
            packet.set_transport_protocol(control_packet::Protocol::AddrRequest.into());
            packet.first_set_ttl(MAX_TTL);
            packet.set_source(current_dev.virtual_ip());
            packet.set_destination(current_dev.virtual_gateway);
            server_cipher.encrypt_ipv4(&mut packet)?;
            let _ = sender.send_main_udp(packet.buffer(), current_dev.connect_server);
        }

        let src = current_dev.virtual_ip();

        if count % 10 == 7 {
            let mut route_list: Option<Vec<(Ipv4Addr, Vec<Route>)>> = None;
            let peer_list = { device_list.lock().1.clone() };
            for peer in peer_list {
                if peer.virtual_ip == current_dev.virtual_ip {
                    continue;
                }
                let client_packet = heartbeat_packet(
                    MAX_TTL,
                    &device_list,
                    &client_cipher,
                    &server_cipher,
                    false,
                    src,
                    peer.virtual_ip,
                );
                if let Some(route) = sender.route_one(&peer.virtual_ip) {
                    if let Err(e) =
                        sender.try_send_by_key(client_packet.buffer(), &route.route_key())
                    {
                        log::warn!("virtual_ip:{},route:{:?},e:{:?}", peer.virtual_ip, route, e);
                    }
                    if route.is_p2p() && !sender.is_first_latency() {
                        continue;
                    }
                } else {
                    //没有直连路由则发送到网关
                    if let Err(e) =
                        sender.send_main(client_packet.buffer(), current_dev.connect_server)
                    {
                        log::warn!(
                            "virtual_ip:{},connect_server:{:?},e:{:?}",
                            peer.virtual_ip,
                            current_dev.connect_server,
                            e
                        );
                    }
                }

                //再随机发送到其他地址，看有没有客户端符合转发条件
                let route_list = route_list.get_or_insert_with(|| {
                    let mut l = sender.route_table();
                    l.shuffle(&mut rand::thread_rng());
                    l
                });
                let mut num = 0;
                'a: for (peer_ip, route_list) in route_list.iter() {
                    for route in route_list {
                        if peer_ip != &peer.virtual_ip && route.is_p2p() {
                            if let Err(e) =
                                sender.try_send_by_key(client_packet.buffer(), &route.route_key())
                            {
                                log::warn!(
                                    "virtual_ip:{},route:{:?},e:{:?}",
                                    peer.virtual_ip,
                                    route,
                                    e
                                );
                            }
                            num += 1;
                            break;
                        }
                        if num >= 2 {
                            break 'a;
                        }
                    }
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        } else {
            for (peer_ip, route_list) in sender.route_table().iter() {
                if peer_ip == &current_dev.virtual_gateway {
                    continue;
                }
                let client_packet = heartbeat_packet(
                    MAX_TTL,
                    &device_list,
                    &client_cipher,
                    &server_cipher,
                    false,
                    src,
                    *peer_ip,
                );
                for route in route_list {
                    if let Err(e) =
                        sender.try_send_by_key(client_packet.buffer(), &route.route_key())
                    {
                        log::warn!("peer_ip:{:?},route:{:?},e:{:?}", peer_ip, route, e);
                    }
                    tokio::time::sleep(Duration::from_millis(2)).await;
                }
            }
        }

        count += 1;
        tokio::time::sleep(Duration::from_millis(3000)).await;
    }
}
