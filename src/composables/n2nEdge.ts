import { invoke } from "@tauri-apps/api/core";
import {
  EdgeRespCommunity,
  EdgeRespEdges,
  EdgeRespPacketStats,
  EdgeRespStatus,
  EdgeRespSupernodes,
  EdgeRespTimestamps,
  EdgeRespVerbose,
} from "~/types/n2n";

export async function status(stop: boolean = false) {
  return await invoke<EdgeRespStatus>("edge_status", { stop });
}

export async function community() {
  return await invoke<EdgeRespCommunity>("edge_community");
}

export async function edges() {
  return await invoke<EdgeRespEdges>("edge_edges");
}

export async function packetStats() {
  return await invoke<EdgeRespPacketStats>("edge_packet_stats");
}

export async function supernodes() {
  return await invoke<EdgeRespSupernodes>("edge_supernodes");
}

export async function timestamps() {
  return await invoke<EdgeRespTimestamps>("edge_timestamps");
}

export async function verbose() {
  return await invoke<EdgeRespVerbose>("edge_verbose");
}
