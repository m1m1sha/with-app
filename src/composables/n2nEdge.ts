import { invoke } from "@tauri-apps/api/core";
import {
  EdgeFlag,
  EdgeRespCommunity,
  EdgeRespEdges,
  EdgeRespPacketStats,
  EdgeRespStatus,
  EdgeRespSupernodes,
  EdgeRespTimestamps,
  EdgeRespVerbose,
} from "~/types/n2n";

export async function status() {
  return await invoke<{
    Status: EdgeRespStatus;
    Error: string;
  }>("edge_action", { flag: EdgeFlag.Status });
}
export async function stop() {
  return await invoke<{
    Stop: EdgeRespStatus;
    Error: string;
  }>("edge_action", { flag: EdgeFlag.Stop });
}

export async function community() {
  return await invoke<{
    Community: EdgeRespCommunity;
    Error: string;
  }>("edge_action", { flag: EdgeFlag.Community });
}

export async function edges() {
  return await invoke<{
    EdgeInfo: EdgeRespEdges | undefined;
    Error: string | undefined;
  }>("edge_action", { flag: EdgeFlag.EdgeInfo });
}

export async function packetStats() {
  return await invoke<{
    PacketStats: EdgeRespPacketStats | undefined;
    Error: string | undefined;
  }>("edge_action", { flag: EdgeFlag.PacketStats });
}

export async function supernodes() {
  return await invoke<{
    SupernodeInfo: EdgeRespSupernodes | undefined;
    Error: string | undefined;
  }>("edge_action", { flag: EdgeFlag.SupernodeInfo });
}

export async function timestamps() {
  return await invoke<{
    Timestamps: EdgeRespTimestamps | undefined;
    Error: string | undefined;
  }>("edge_action", { flag: EdgeFlag.Timestamps });
}

export async function verbose() {
  return await invoke<{
    Verbose: EdgeRespVerbose | undefined;
    Error: string | undefined;
  }>("edge_action", { flag: EdgeFlag.Verbose });
}
