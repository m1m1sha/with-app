#![allow(dead_code)]

use crate::plugins::n2n::{
    error::N2nError,
    models::{self, edge},
};
/// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c
use crate::utils::string;
use std::{io::ErrorKind, net::SocketAddr, time::Duration};
use tokio::{net::UdpSocket, time::timeout};

const EDGE_MSG_TYPE_ROW: &str = "\"_type\":\"row\",";
const SOCKET_TIMEOUT_MILLIS: u64 = 1000;

#[derive(Clone)]
pub struct EdgeSocketConfig {
    pub addr: SocketAddr,
    pub auth: Option<String>,
    pub timeout: Option<u64>,
}

pub struct Manager {
    socket: UdpSocket,
    auth: String,
    timeout: u64,
}

impl Manager {
    pub async fn new(config: EdgeSocketConfig) -> Result<Self, N2nError> {
        let socket = match UdpSocket::bind("127.0.0.1:0").await {
            Ok(s) => s,
            Err(_) => return Err(N2nError::AddrInUse),
        };
        if socket.connect(config.addr).await.is_err() {
            return Err(N2nError::ConnectFailed);
        }

        let auth = match config.auth {
            Some(a) => {
                if a.is_empty() {
                    "n2n".to_owned()
                } else {
                    a
                }
            }
            None => "n2n".to_owned(),
        };
        // 默认1秒超时
        Ok(Self {
            socket,
            auth,
            timeout: config.timeout.unwrap_or(SOCKET_TIMEOUT_MILLIS),
        })
    }

    /// 发送数据
    async fn send(&mut self, cmd: models::Cmd, action: models::Action) -> Result<u16, N2nError> {
        let auth = format!("1:{}", self.auth);
        // 生成本次数据标志
        let flag = fastrand::u16(..);
        let content = format!("{} {}:{} {}", action.to_str(), flag, auth, cmd.to_str());
        if self.socket.send(content.as_bytes()).await.is_err() {
            return Err(N2nError::SendFailed);
        }

        Ok(flag)
    }

    /// 接收数据
    async fn recv(&mut self, flag: u16) -> Result<Vec<String>, N2nError> {
        let flag = format!("{}", flag);
        let _flag = format!("\"_tag\":\"{}\",", flag);
        let mut recv_vec = vec![];
        let mut block_num = 0;
        let mut recv_cmd = String::new();

        loop {
            // 超时处理, 以防edge已经关闭
            if let Err(e) = timeout(Duration::from_millis(self.timeout), async {
                self.socket.readable().await.unwrap()
            })
            .await
            {
                println!("{:?}", e.to_string());
                return Err(N2nError::Timeout);
            }

            let mut buf = vec![0; 2048];

            match self.socket.try_recv(&mut buf[..]) {
                Ok(n) => {
                    let str = string::from_utf8_or_gbk(&buf[..n]).trim().to_string();

                    // flag判断前置
                    if !str.contains(&_flag) {
                        break;
                    }

                    let resp = match serde_json::from_str::<models::Resp>(str.as_str()) {
                        Ok(r) => r,
                        Err(_) => return Err(N2nError::Parse),
                    };

                    match resp {
                        models::Resp::Error { error } => {
                            return Err(edge::RespError::from(error.as_str()).to_err());
                        }
                        models::Resp::Begin { cmd } => {
                            recv_cmd = cmd;
                            continue;
                        }
                        models::Resp::End { cmd } => {
                            if recv_cmd != cmd {
                                continue;
                            }
                            break;
                        }
                        models::Resp::Row {} => {
                            recv_vec.push(str.replace(&_flag, "").replace(EDGE_MSG_TYPE_ROW, ""));
                        }
                    }
                }
                Err(ref e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        // 阻塞最多尝试3次
                        if block_num < 3 {
                            block_num += 1;
                            continue;
                        }
                    }
                    break;
                }
            }
        }
        Ok(recv_vec)
    }

    /// 发送数据并接收
    async fn send_and_recv(
        &mut self,
        cmd: models::Cmd,
        action: models::Action,
    ) -> Result<Vec<String>, N2nError> {
        let flag = self.send(cmd.clone(), action).await?;
        Ok(self.recv(flag).await?)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L363
    ///
    /// 重载组数据, edge无用, 官方未实现
    pub async fn reload_communities(&self) -> Result<(), N2nError> {
        Err(N2nError::UnImplemented)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L162
    ///
    /// 停止edge
    pub async fn stop(&mut self, write: bool) -> Result<edge::Stop, N2nError> {
        let req = self
            .send_and_recv(
                models::Cmd::Stop,
                match write {
                    true => models::Action::Write,
                    false => models::Action::Read,
                },
            )
            .await?;
        match serde_json::from_str::<edge::Stop>(&req[0]) {
            Ok(s) => Ok(s),
            Err(_) => Err(N2nError::Parse),
        }
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L171
    ///
    /// 日志等级
    pub async fn verbose(&mut self) -> Result<edge::Verbose, N2nError> {
        let req = self
            .send_and_recv(models::Cmd::Verbose, models::Action::Read)
            .await?;
        match serde_json::from_str::<edge::Verbose>(&req[0]) {
            Ok(s) => Ok(s),
            Err(_) => Err(N2nError::Parse),
        }
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L182
    ///
    /// 获取当前组名, supernode的应该是communities
    pub async fn community(&mut self) -> Result<edge::Community, N2nError> {
        let req = self
            .send_and_recv(models::Cmd::Communities, models::Action::Read)
            .await?;
        match serde_json::from_str::<edge::Community>(&req[0]) {
            Ok(s) => Ok(s),
            Err(_) => Err(N2nError::Parse),
        }
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L269
    ///
    /// 获取组内设备信息
    pub async fn edges(&mut self) -> Result<Vec<edge::EdgeInfo>, N2nError> {
        let req = self
            .send_and_recv(models::Cmd::Edges, models::Action::Read)
            .await?;
        let mut edges = Vec::new();
        for r in req {
            match serde_json::from_str::<edge::EdgeInfo>(&r) {
                Ok(s) => edges.push(s),
                Err(_) => return Err(N2nError::Parse),
            }
        }
        Ok(edges)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L192
    ///
    /// supernode节点信息
    ///
    /// `selection`字段有问题：`"selection": "load =        0"`
    pub async fn supernodes(&mut self) -> Result<edge::SupernodeInfo, N2nError> {
        let req = self
            .send_and_recv(models::Cmd::Supernodes, models::Action::Read)
            .await?;
        match serde_json::from_str::<edge::SupernodeInfo>(&req[0]) {
            Ok(s) => Ok(s),
            Err(_) => Err(N2nError::Parse),
        }
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L283
    ///
    /// 连接时间戳
    pub async fn timestamps(&mut self) -> Result<edge::Timestamps, N2nError> {
        let req = self
            .send_and_recv(models::Cmd::Timestamps, models::Action::Read)
            .await?;
        match serde_json::from_str::<edge::Timestamps>(&req[0]) {
            Ok(s) => Ok(s),
            Err(_) => Err(N2nError::Parse),
        }
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L301
    ///
    /// 流量统计
    pub async fn packet_stats(&mut self) -> Result<edge::PacketStats, N2nError> {
        let req = self
            .send_and_recv(models::Cmd::PacketStats, models::Action::Read)
            .await?;
        let mut stats = edge::PacketStats::default();
        for r in req {
            let tmp = match serde_json::from_str::<edge::PacketStatsType>(&r) {
                Ok(s) => s,
                Err(_) => return Err(N2nError::Parse),
            };
            match tmp {
                edge::PacketStatsType::Transport { tx_pkt, rx_pkt } => {
                    stats.transport = edge::PacketStatsPkt { tx_pkt, rx_pkt }
                }
                edge::PacketStatsType::P2P { tx_pkt, rx_pkt } => {
                    stats.p2p = edge::PacketStatsPkt { tx_pkt, rx_pkt }
                }
                edge::PacketStatsType::Supernode { tx_pkt, rx_pkt } => {
                    stats.supernode = edge::PacketStatsPkt { tx_pkt, rx_pkt }
                }
                edge::PacketStatsType::SupernodeBroadcast { tx_pkt, rx_pkt } => {
                    stats.supernode_broadcast = edge::PacketStatsPkt { tx_pkt, rx_pkt }
                }
            }
        }
        Ok(stats)
    }
}
