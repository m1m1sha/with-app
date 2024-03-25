use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_channel::Sender;
use tokio::sync::Mutex;

use crate::plugins::n2n::{
    error::N2nError,
    manager::edge::{EdgeSocketConfig, Manager},
    models::{
        args::EdgeArgs,
        edge::{EdgeFlag, EdgeFlagPayload},
    },
};

pub struct EdgeState {
    pub config: Mutex<EdgeSocketConfig>,
    pub tx:
        Mutex<Option<async_channel::Sender<(EdgeFlag, Sender<Result<EdgeFlagPayload, N2nError>>)>>>,
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

#[tauri::command]
pub async fn edge_start(
    args: EdgeArgs,
    state: tauri::State<'_, EdgeState>,
) -> Result<(), N2nError> {
    // n2n 启动
    let mut manager = Manager::new(args.to_socket_config()).await?;
    let (tx, rx) =
        async_channel::bounded::<(EdgeFlag, Sender<Result<EdgeFlagPayload, N2nError>>)>(20);
    *state.tx.lock().await = Some(tx.clone());
    tauri::async_runtime::spawn(async move {
        loop {
            match rx.try_recv() {
                Ok((flag, send)) => {
                    let payload = match flag {
                        EdgeFlag::Stop => match manager.stop(true).await {
                            Ok(r) => Ok(EdgeFlagPayload::Stop(r)),
                            Err(e) => Err(e),
                        },
                        EdgeFlag::Status => match manager.stop(false).await {
                            Ok(r) => Ok(EdgeFlagPayload::Status(r)),
                            Err(e) => Err(e),
                        },
                        EdgeFlag::Verbose => match manager.verbose().await {
                            Ok(r) => Ok(EdgeFlagPayload::Verbose(r)),
                            Err(e) => Err(e),
                        },
                        EdgeFlag::Timestamps => match manager.timestamps().await {
                            Ok(r) => Ok(EdgeFlagPayload::Timestamps(r)),
                            Err(e) => Err(e),
                        },
                        EdgeFlag::Community => match manager.community().await {
                            Ok(r) => Ok(EdgeFlagPayload::Community(r)),
                            Err(e) => Err(e),
                        },
                        EdgeFlag::SupernodeInfo => match manager.supernodes().await {
                            Ok(r) => Ok(EdgeFlagPayload::SupernodeInfo(r)),
                            Err(e) => Err(e),
                        },
                        EdgeFlag::PacketStats => match manager.packet_stats().await {
                            Ok(r) => Ok(EdgeFlagPayload::PacketStats(r)),
                            Err(e) => Err(e),
                        },
                        EdgeFlag::EdgeInfo => match manager.edges().await {
                            Ok(r) => Ok(EdgeFlagPayload::EdgeInfo(r)),
                            Err(e) => Err(e),
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
    let (_tx, _rx) = async_channel::bounded::<Result<EdgeFlagPayload, N2nError>>(1);

    if let Some(tx) = state.tx.lock().await.clone() {
        return match tx.try_send((flag, _tx)) {
            Ok(_) => loop {
                match _rx.try_recv() {
                    Ok(p) => match p {
                        Ok(p) => match p {
                            EdgeFlagPayload::Stop(_) => {
                                *state.tx.lock().await = None;
                                return Ok(p);
                            }
                            _ => return Ok(p),
                        },
                        Err(e) => return Err(e),
                    },
                    Err(e) => match e {
                        async_channel::TryRecvError::Empty => continue,
                        async_channel::TryRecvError::Closed => return Err(N2nError::RecvFailed),
                    },
                }
            },
            Err(_) => Err(N2nError::SendFailed),
        };
    }

    Err(N2nError::EdgeStopped)
}
