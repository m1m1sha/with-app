use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_channel::Sender;
use tokio::sync::Mutex;

use crate::plugins::n2n::{
    error::N2nError,
    manager::edge::{EdgeSocketConfig, Manager},
    models::{
        args::EdgeArgs,
        edge::{
            Community, EdgeFlag, EdgeFlagPayload, EdgeInfo, PacketStats, Stop, SupernodeInfo,
            Timestamps, Verbose,
        },
    },
};

pub struct EdgeState {
    pub config: Mutex<EdgeSocketConfig>,
    pub tx: Mutex<Option<async_channel::Sender<(EdgeFlag, Sender<EdgeFlagPayload>)>>>,
}

impl EdgeState {
    pub fn default() -> Self {
        Self {
            config: Mutex::new(EdgeSocketConfig {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5644),
                auth: None,
                timeout: None,
            }),
            tx: Mutex::new(None),
        }
    }
}

pub async fn edge_start(
    args: EdgeArgs,
    state: tauri::State<'_, EdgeState>,
) -> Result<(), N2nError> {
    // n2n 启动
    let mut manager = Manager::new(args.to_socket_config()).await?;
    let (tx, rx) = async_channel::bounded::<(EdgeFlag, Sender<EdgeFlagPayload>)>(20);
    *state.tx.lock().await = Some(tx.clone());
    tauri::async_runtime::spawn(async move {
        loop {
            match rx.try_recv() {
                Ok((flag, send)) => {
                    let payload = match flag {
                        EdgeFlag::Stop => match manager.stop(false).await {
                            Ok(r) => EdgeFlagPayload::Stop(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                        EdgeFlag::Status => match manager.stop(true).await {
                            Ok(r) => EdgeFlagPayload::Status(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                        EdgeFlag::Verbose => match manager.verbose().await {
                            Ok(r) => EdgeFlagPayload::Verbose(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                        EdgeFlag::Timestamps => match manager.timestamps().await {
                            Ok(r) => EdgeFlagPayload::Timestamps(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                        EdgeFlag::Community => match manager.community().await {
                            Ok(r) => EdgeFlagPayload::Community(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                        EdgeFlag::SupernodeInfo => match manager.supernodes().await {
                            Ok(r) => EdgeFlagPayload::SupernodeInfo(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                        EdgeFlag::PacketStats => match manager.packet_stats().await {
                            Ok(r) => EdgeFlagPayload::PacketStats(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                        EdgeFlag::EdgeInfo => match manager.edges().await {
                            Ok(r) => EdgeFlagPayload::EdgeInfo(r),
                            Err(e) => EdgeFlagPayload::Error(e.to_string()),
                        },
                    };
                    let _ = send.send(payload).await;
                }
                Err(e) => match e {
                    async_channel::TryRecvError::Empty => continue,
                    async_channel::TryRecvError::Closed => break,
                },
            };
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn edge_action(
    flag: EdgeFlag,
    state: tauri::State<'_, EdgeState>,
) -> Result<EdgeFlagPayload, N2nError> {
    let (_tx, _rx) = async_channel::bounded::<EdgeFlagPayload>(1);

    if let Some(tx) = state.tx.lock().await.clone() {
        return match tx.try_send((flag, _tx)) {
            Ok(_) => match _rx.try_recv() {
                Ok(p) => match p {
                    EdgeFlagPayload::Stop(_) => {
                        *state.tx.lock().await = None;
                        Ok(p)
                    }
                    _ => Ok(p),
                },
                Err(_) => Err(N2nError::RecvFailed),
            },
            Err(_) => Err(N2nError::SendFailed),
        };
    }

    Err(N2nError::EdgeStopped)
}
