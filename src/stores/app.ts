import { UpdateManifest } from "@tauri-apps/api/updater";
import { defineStore } from "pinia";
import { DeviceItem } from "~/composables/with";

export interface WithRoute {
  ip: string;
  routes: Route[];
}

export interface Route {
  is_tcp: boolean;
  index: number;
  addr: string;
  metric: number;
  rt: number;
}

export interface NeedRoute {
  ip: string;
  rt: string;
  channel: string;
}

export enum WithStatus {
  Stopping = "stopping",
  Stopped = "stopped",
  Connecting = "connecting",
  Connected = "connected",
}

export interface WithLocalInfo {
  virtual_gateway: string;
  virtual_ip: string;
  virtual_netmask: string;
}

export const useAppStore = defineStore("app", () => {
  const menu = ref("/");
  const settingTab = ref("basic");

  const appUpdaterVisible = ref(false);
  const appUpdaterInfo = ref<UpdateManifest | undefined>();
  const appUpdaterLoading = ref(false);

  const withStatus = ref<WithStatus>(WithStatus.Stopped);

  const withDeviceItems = ref<DeviceItem[]>([]);
  const withTryConnect = ref(0);

  const withLocalInfo = ref<WithLocalInfo | null>();
  const winIPBroadcastStatus = ref(false);

  return {
    menu,
    settingTab,
    withStatus,
    withDeviceItems,
    withLocalInfo,
    withTryConnect,
    winIPBroadcastStatus,
    appUpdaterVisible,
    appUpdaterInfo,
    appUpdaterLoading,
  };
});
