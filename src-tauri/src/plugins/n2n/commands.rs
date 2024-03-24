use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::sync::Mutex;

use crate::plugins::n2n::{
    error::N2nError,
    manager::edge::{EdgeSocketConfig, Manager},
    models::edge::{Community, EdgeInfo, PacketStats, Stop, SupernodeInfo, Timestamps, Verbose},
};

pub struct EdgeState {
    pub config: Mutex<EdgeSocketConfig>,
}

impl EdgeState {
    pub fn default() -> Self {
        Self {
            config: Mutex::new(EdgeSocketConfig {
                addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5644),
                auth: None,
                timeout: None,
            }),
        }
    }
}

#[tauri::command]
pub async fn edge_community(state: tauri::State<'_, EdgeState>) -> Result<Community, N2nError> {
    Manager::new(state.config.lock().await.clone())
        .await?
        .community()
        .await
}

#[tauri::command]
pub async fn edge_edges(state: tauri::State<'_, EdgeState>) -> Result<Vec<EdgeInfo>, N2nError> {
    Manager::new(state.config.lock().await.clone())
        .await?
        .edges()
        .await
}

#[tauri::command]
pub async fn edge_status(stop: bool, state: tauri::State<'_, EdgeState>) -> Result<Stop, N2nError> {
    Manager::new(state.config.lock().await.clone())
        .await?
        .stop(stop)
        .await
}

#[tauri::command]
pub async fn edge_packet_stats(
    state: tauri::State<'_, EdgeState>,
) -> Result<PacketStats, N2nError> {
    Manager::new(state.config.lock().await.clone())
        .await?
        .packet_stats()
        .await
}

#[tauri::command]
pub async fn edge_supernodes(
    state: tauri::State<'_, EdgeState>,
) -> Result<SupernodeInfo, N2nError> {
    Manager::new(state.config.lock().await.clone())
        .await?
        .supernodes()
        .await
}

#[tauri::command]
pub async fn edge_timestamps(state: tauri::State<'_, EdgeState>) -> Result<Timestamps, N2nError> {
    Manager::new(state.config.lock().await.clone())
        .await?
        .timestamps()
        .await
}

#[tauri::command]
pub async fn edge_verbose(state: tauri::State<'_, EdgeState>) -> Result<Verbose, N2nError> {
    Manager::new(state.config.lock().await.clone())
        .await?
        .verbose()
        .await
}
