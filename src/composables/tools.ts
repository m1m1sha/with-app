import { invoke } from "@tauri-apps/api/core";

export async function broadcast_start(force: boolean = false) {
  return await invoke("win_ip_broadcast_start", { force });
}

export async function broadcast_stop() {
  return await invoke("win_ip_broadcast_stop");
}

export async function tap_install(silent: boolean = false) {
  return await invoke("tap_install", { silent });
}
