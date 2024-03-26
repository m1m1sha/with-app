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

export async function status(): Promise<{
  Status: EdgeRespStatus;
}> {
  return await invoke<{
    Status: EdgeRespStatus;
  }>("edge_action", { flag: EdgeFlag.Status });
}
export async function stop() {
  return await invoke<{
    Stop: EdgeRespStatus;
  }>("edge_action", { flag: EdgeFlag.Stop });
}

export async function community(): Promise<{
  Community: EdgeRespCommunity;
}> {
  return await invoke<{
    Community: EdgeRespCommunity;
  }>("edge_action", { flag: EdgeFlag.Community });
}

export async function edges(): Promise<{
  EdgeInfo: EdgeRespEdges;
}> {
  return await invoke<{
    EdgeInfo: EdgeRespEdges;
  }>("edge_action", { flag: EdgeFlag.EdgeInfo });
}

export async function packetStats(): Promise<{
  PacketStats: EdgeRespPacketStats;
}> {
  return await invoke<{
    PacketStats: EdgeRespPacketStats;
  }>("edge_action", { flag: EdgeFlag.PacketStats });
}

export async function supernodes(): Promise<{
  SupernodeInfo: EdgeRespSupernodes;
}> {
  return await invoke<{
    SupernodeInfo: EdgeRespSupernodes;
  }>("edge_action", { flag: EdgeFlag.SupernodeInfo });
}

export async function timestamps(): Promise<{
  Timestamps: EdgeRespTimestamps;
}> {
  return await invoke<{
    Timestamps: EdgeRespTimestamps;
  }>("edge_action", { flag: EdgeFlag.Timestamps });
}

export async function verbose(): Promise<{
  Verbose: EdgeRespVerbose;
}> {
  return await invoke<{
    Verbose: EdgeRespVerbose;
  }>("edge_action", { flag: EdgeFlag.Verbose });
}

export async function start(): Promise<null> {
  return await invoke<null>("edge_start", {
    args: { c: "110110", l: "101.34.37.43:7654", t: 5644 },
  });
}
