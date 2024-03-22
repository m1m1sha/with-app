use crate::link::n2n;
use crate::utils::string::{self};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{
    io::{Error, ErrorKind},
    net::SocketAddr,
    time::Duration,
};
use tokio::{net::UdpSocket, time::timeout};

/// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c

const EDGE_TYPE_ROW: &str = "\"_type\":\"row\",";
const EDGE_TYPE_END: &str = "\"_type\":\"end\",";
const EDGE_TYPE_ERROR: &str = "\"_type\":\"error\",";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stop {
    #[serde(rename = "keep_running")]
    pub status: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Verbose {
    #[serde(rename = "traceLevel")]
    pub level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Community {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeInfo {
    pub mode: String,
    #[serde(rename = "ip4addr")]
    pub ipv4: String,
    pub purgeable: i32,
    pub local: i32,
    #[serde(rename = "macaddr")]
    pub mac: String,
    #[serde(rename = "sockaddr")]
    pub socket: String,
    pub desc: String,
    pub last_p2p: u64,
    pub last_sent_query: u64,
    pub last_seen: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupernodeInfo {
    pub version: String,
    pub purgeable: i32,
    pub current: i32,
    #[serde(rename = "macaddr")]
    pub mac: String,
    #[serde(rename = "sockaddr")]
    pub socket: String,
    pub selection: String,
    pub last_seen: u64,
    pub uptime: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamps {
    pub start_time: u64,
    pub last_super: u64,
    pub last_p2p: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketStatsPkt {
    pub tx_pkt: u32,
    pub rx_pkt: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TmpPacketStats {
    #[serde(rename = "type")]
    r#type: String,
    tx_pkt: u32,
    rx_pkt: u32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketStats {
    pub transport: PacketStatsPkt,
    pub p2p: PacketStatsPkt,
    pub supernode: PacketStatsPkt,
    pub supernode_broadcast: PacketStatsPkt,
}

pub struct Manager {
    socket: UdpSocket,
    auth: Option<String>,
    timeout: u64,
}

impl Manager {
    pub async fn new(
        addr: SocketAddr,
        auth: Option<String>,
        timeout: Option<u64>,
    ) -> anyhow::Result<Self> {
        let socket = UdpSocket::bind("127.0.0.1:0").await?;
        socket.connect(addr).await?;
        // 默认1秒超时
        let timeout = timeout.unwrap_or(1000);
        Ok(Self {
            socket,
            auth,
            timeout,
        })
    }

    /// 发送数据
    async fn send(&mut self, cmd: n2n::Cmd, action: n2n::Action) -> anyhow::Result<u16> {
        let auth = match self.auth.clone() {
            Some(a) => format!("1:{}", a),
            None => format!("0"),
        };
        // 生成本次数据标志
        let flag = fastrand::u16(..);
        let content = format!("{} {}:{} {}", action.to_str(), flag, auth, cmd.to_str());
        self.socket.send(content.as_bytes()).await?;

        Ok(flag)
    }

    /// 接收数据
    async fn recv(&mut self, flag: u16) -> anyhow::Result<Vec<String>> {
        // 超时处理, 以防edge已经关闭
        if let Err(_) = timeout(Duration::from_millis(self.timeout), async {
            self.socket.readable().await.unwrap()
        })
        .await
        {
            return Err(Error::new(ErrorKind::TimedOut, "获取数据超时").into());
        }

        let flag = format!("\"_tag\":\"{}\",", flag);
        let mut recv_vec = vec![];
        let mut block_num = 0;

        loop {
            let mut buf = vec![0; 2048];
            match self.socket.try_recv(&mut buf[..]) {
                Ok(n) => {
                    // 清除\n和空格
                    let str = string::from_utf8_or_gbk(&buf[..(n - 1)]).trim().to_string();

                    // 判断是否为空或是否是同标志数据
                    if str.is_empty() || !str.contains(&flag) {
                        continue;
                    }

                    // 出现error标识时返回错误, 除去stop的badauth
                    if str.contains(EDGE_TYPE_ERROR) {
                        return Err(anyhow!("发生错误: {:?}", str));
                    }

                    // 出现end标识时结束接收
                    if str.contains(EDGE_TYPE_END) {
                        break;
                    }

                    // 只返回有用row数据
                    if str.contains(EDGE_TYPE_ROW) {
                        recv_vec.push(str.replace(&flag, "").replace(EDGE_TYPE_ROW, ""));
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // 阻塞最多尝试3次
                    if block_num >= 3 {
                        break;
                    }
                    block_num += 1;
                }
                Err(_) => {
                    break;
                }
            }
        }
        Ok(recv_vec)
    }

    /// 发送数据并接收
    async fn send_and_recv(
        &mut self,
        cmd: n2n::Cmd,
        action: n2n::Action,
    ) -> anyhow::Result<Vec<String>> {
        let flag = self.send(cmd.clone(), action).await?;

        Ok(self.recv(flag).await?)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L363
    ///
    /// 重载组数据, edge无用, 官方未实现
    pub async fn reload_communities(&self) -> anyhow::Result<()> {
        Err(anyhow!("未实现"))
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L162
    ///
    /// 停止edge
    ///
    /// write会报错无权限`{"_tag":"1313","_type":"error","error":"badauth"}`但能正常结束
    pub async fn stop(&mut self, write: bool) -> anyhow::Result<Stop> {
        let req = self
            .send_and_recv(
                n2n::Cmd::Stop,
                match write {
                    true => n2n::Action::Write,
                    false => n2n::Action::Read,
                },
            )
            .await?;
        Ok(serde_json::from_str::<Stop>(&req[0])?)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L171
    ///
    /// 日志等级
    pub async fn verbose(&mut self) -> anyhow::Result<Verbose> {
        let req = self
            .send_and_recv(n2n::Cmd::Verbose, n2n::Action::Read)
            .await?;
        Ok(serde_json::from_str::<Verbose>(&req[0])?)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L182
    ///
    /// 获取当前组名, supernode的应该是communities
    pub async fn community(&mut self) -> anyhow::Result<Community> {
        let req = self
            .send_and_recv(n2n::Cmd::Communities, n2n::Action::Read)
            .await?;
        Ok(serde_json::from_str::<Community>(&req[0])?)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L269
    ///
    /// 获取组内设备信息
    pub async fn edges(&mut self) -> anyhow::Result<Vec<EdgeInfo>> {
        let req = self
            .send_and_recv(n2n::Cmd::Edges, n2n::Action::Read)
            .await?;
        let mut edges = Vec::new();
        for r in req {
            edges.push(serde_json::from_str::<EdgeInfo>(&r)?);
        }
        Ok(edges)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L192
    ///
    /// supernode节点信息
    ///
    /// `selection`字段有问题：`"selection": "load =        0"`
    pub async fn supernodes(&mut self) -> anyhow::Result<SupernodeInfo> {
        let req = self
            .send_and_recv(n2n::Cmd::Supernodes, n2n::Action::Read)
            .await?;
        Ok(serde_json::from_str::<SupernodeInfo>(&req[0])?)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L283
    ///
    /// 连接时间戳
    pub async fn timestamps(&mut self) -> anyhow::Result<Timestamps> {
        let req = self
            .send_and_recv(n2n::Cmd::Timestamps, n2n::Action::Read)
            .await?;
        Ok(serde_json::from_str::<Timestamps>(&req[0])?)
    }

    /// 来源：https://github.com/ntop/n2n/blob/3.1.1/src/edge_management.c#L301
    ///
    /// 流量统计
    pub async fn packet_stats(&mut self) -> anyhow::Result<PacketStats> {
        let req = self
            .send_and_recv(n2n::Cmd::PacketStats, n2n::Action::Read)
            .await?;
        let mut stats = PacketStats::default();
        for r in req {
            let tmp = serde_json::from_str::<TmpPacketStats>(&r)?;

            let tmp_pkt = PacketStatsPkt {
                tx_pkt: tmp.tx_pkt,
                rx_pkt: tmp.rx_pkt,
            };

            match tmp.r#type.as_str() {
                "transop" => stats.transport = tmp_pkt,
                "p2p" => stats.p2p = tmp_pkt,
                "super" => stats.supernode = tmp_pkt,
                "super_broadcast" => stats.supernode_broadcast = tmp_pkt,
                _ => {
                    return Err(anyhow!("unknown packet stats type: {}", tmp.r#type));
                }
            }
        }
        Ok(stats)
    }
}
