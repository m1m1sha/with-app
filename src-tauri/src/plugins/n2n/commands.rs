use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::Stdio,
    time::Duration,
};

use async_channel::Sender;
use tauri::{AppHandle, Manager as _};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::Mutex,
    time::timeout,
};

use crate::{
    plugins::n2n::{
        error::N2nError,
        manager::edge::{EdgeSocketConfig, Manager},
        models::{
            args::EdgeArgs,
            edge::{EdgeFlag, EdgeFlagPayload},
        },
    },
    utils::{self},
};

const N2N_EDGE_EXECUTABLE: &str = "with_n2n_edge_v3.1.1.exe";

const N2N_EDGE_EVENT_NAME: &str = "n2n_edge_event";

const N2N_EDGE_CONNECT_TIMEOUT: Duration = Duration::from_millis(3000);

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
    app: AppHandle,
    state: tauri::State<'_, EdgeState>,
) -> Result<(), N2nError> {
    let mut child = match Command::new("cmd")
        .creation_flags(0x08000000)
        .stdout(Stdio::piped())
        .stdin(Stdio::null())
        .kill_on_drop(true)
        .current_dir(
            app.path()
                .resource_dir()
                .unwrap_or_default()
                .join("bin")
                .to_string_lossy()
                .replace("\\\\?\\", ""),
        )
        .args(match args.clone().to_args() {
            Ok(mut a) => {
                let mut v = vec!["/C".to_owned(), N2N_EDGE_EXECUTABLE.to_owned()];
                v.append(&mut a);
                v
            }
            Err(_) => return Err(N2nError::ArgsInvalid),
        })
        .spawn()
    {
        Ok(c) => c,
        Err(_) => {
            let _ = utils::process::kill_process(N2N_EDGE_EXECUTABLE.to_owned());
            let _ = utils::process::kill_process(N2N_EDGE_EXECUTABLE.to_owned());
            return Err(N2nError::EdgeStartFailed);
        }
    };

    let (n_tx, n_rx) = async_channel::bounded::<bool>(1);

    tauri::async_runtime::spawn(async move {
        let stdout = child.stdout.take().unwrap();

        let mut stdout_reader = BufReader::new(stdout);
        let mut stdout_buf: Vec<u8> = vec![];
        while let Ok(n) = stdout_reader.read_until(b'\n', &mut stdout_buf).await {
            match n.cmp(&0) {
                std::cmp::Ordering::Greater => {
                    let mut out = utils::string::from_utf8_or_gbk(&stdout_buf);
                    out = out.trim().to_string();
                    if !out.is_empty() {
                        if out.contains("[OK] edge <<< ================ >>> supernode") {
                            let _ = n_tx.send(true).await;
                        }
                        let _ = app.emit(N2N_EDGE_EVENT_NAME, out.clone());
                        println!("{}", out);
                    }
                }
                _ => break,
            }
            stdout_buf.clear();
        }

        child.wait().await.expect("Failed to wait process");
    });

    let mut manager = Manager::new(args.to_socket_config()).await?;

    // 最长等待连接
    if let Err(_) = timeout(N2N_EDGE_CONNECT_TIMEOUT, async {
        loop {
            if let Ok(_) = n_rx.recv().await {
                break;
            }
        }
    })
    .await
    {
        return Err(N2nError::EdgeStartFailed);
    };

    if let Err(_) = manager.stop(false).await {
        *state.tx.lock().await = None;
        let _ = utils::process::kill_process(N2N_EDGE_EXECUTABLE.to_owned());
        let _ = utils::process::kill_process(N2N_EDGE_EXECUTABLE.to_owned());
        return Err(N2nError::SocketConnectTimeout);
    }
    let (tx, rx) =
        async_channel::bounded::<(EdgeFlag, Sender<Result<EdgeFlagPayload, N2nError>>)>(20);

    if let Some(tx) = state.tx.lock().await.take() {
        tx.close();
    }
    *state.tx.lock().await = Some(tx);
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

                    if let Err(e) = payload.clone() {
                        // n2n错误 继续循环, 其他错误结束manager并退出
                        match e {
                            N2nError::BadAuth
                            | N2nError::BadType
                            | N2nError::NoAccess
                            | N2nError::NoCmd
                            | N2nError::NoFile
                            | N2nError::NoOptions
                            | N2nError::NoType
                            | N2nError::ReadOnly
                            | N2nError::WriteOnly
                            | N2nError::UnImplemented
                            | N2nError::UnknownCmd
                            | N2nError::UnknownTopic => {}
                            _ => {
                                rx.close();
                                let _ = send.send(payload).await;
                                break;
                            }
                        }
                    }

                    let _ = send.send(payload).await;
                }
                Err(e) => match e {
                    async_channel::TryRecvError::Empty => continue,
                    async_channel::TryRecvError::Closed => {
                        rx.close();
                        break;
                    }
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
    let state_tx = state.tx.lock().await.clone();
    if let Some(tx) = state_tx {
        let (_tx, _rx) = async_channel::bounded::<Result<EdgeFlagPayload, N2nError>>(1);

        match tx.try_send((flag, _tx)) {
            Ok(_) => loop {
                match _rx.try_recv() {
                    Ok(p) => match p {
                        Ok(p) => match p {
                            EdgeFlagPayload::Stop(_) => {
                                tx.close();
                                *state.tx.lock().await = None;
                                return Ok(p);
                            }
                            _ => return Ok(p),
                        },
                        Err(e) => return Err(e),
                    },
                    Err(e) => match e {
                        async_channel::TryRecvError::Empty => continue,
                        async_channel::TryRecvError::Closed => {
                            tx.close();
                            *state.tx.lock().await = None;
                            return Err(N2nError::ActionChannelRecvClosed);
                        }
                    },
                }
            },
            Err(e) => match e {
                async_channel::TrySendError::Full(_) => Err(N2nError::ActionChannelSendFull),
                async_channel::TrySendError::Closed(_) => {
                    tx.close();
                    *state.tx.lock().await = None;
                    Err(N2nError::ActionChannelSendClosed)
                }
            },
        }
    } else {
        if flag == EdgeFlag::Stop {
            let _ = utils::process::kill_process(N2N_EDGE_EXECUTABLE.to_owned());
            let _ = utils::process::kill_process(N2N_EDGE_EXECUTABLE.to_owned());
        }
        Err(N2nError::EdgeIsStopped)
    }
}
