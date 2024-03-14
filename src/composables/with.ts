import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import type { withConfig } from "./config";
import { WithLocalInfo, WithStatus } from "~/stores/app";
import { MessagePlugin } from "tdesign-vue-next";

const WITH_START = "with_start";
const WITH_STOP = "with_stop";

const WITH_EVENT_CONNECT = "with_event_connect";
const appStore = useAppStore();

const { withStatus, withDeviceItems, withLocalInfo, withTryConnect } =
  storeToRefs(appStore);
export async function withStart(config: withConfig) {
  withStatus.value = WithStatus.Connecting;
  await invoke(WITH_START, { config });
}

export async function withStop() {
  withStatus.value = WithStatus.Stopping;
  await invoke(WITH_STOP, {});
}

export interface EventPayload {
  flag: string;
  data: string;
}

export enum NatType {
  Symmetric = "Symmetric",
  Cone = "Cone",
}

export enum DeviceMetric {
  P2p = "p2p",
  ClientRelay = "ClientRelay",
  ServerRelay = "ServerRelay",
}

export interface DeviceItem {
  name: String;
  virtual_ip: String;

  tcp: boolean;
  metric: DeviceMetric;

  rt: number;
  online: boolean;

  nat_type: NatType;
  public_ips: string[];
  local_ip: string;
  ipv6: string;

  same_secret: boolean;
}

interface EventConnecting {
  count: number;
  address: string;
}

enum EventFlag {
  success = "success",
  stop = "stop",
  register = "register",
  route = "route",
  connecting = "connect",
  timeout = "timeout",
}

export async function withEventConnect(): Promise<UnlistenFn> {
  return listen<EventPayload>(WITH_EVENT_CONNECT, async (event) => {
    let data = {};

    try {
      data = JSON.parse(event.payload.data);
    } catch (e) {
      data = {};
    }

    switch (event.payload.flag) {
      case EventFlag.success:
        withStatus.value = WithStatus.Connected;
        withTryConnect.value = 0;
        break;
      case EventFlag.stop || EventFlag.timeout:
        withStatus.value = WithStatus.Stopped;
        withLocalInfo.value = null;
        withDeviceItems.value = [];
        withTryConnect.value = 0;
        break;
      case EventFlag.register:
        withLocalInfo.value = data as WithLocalInfo;
        break;
      case EventFlag.connecting:
        withTryConnect.value = (data as EventConnecting).count;
        if (withTryConnect.value >= 5) {
          MessagePlugin.error({
            content: "连接超时，请检查服务器是否在线后重试",
          });
          await withStop();
        }
        break;
      case EventFlag.route:
        if (withStatus.value !== WithStatus.Connected) {
          withStatus.value = WithStatus.Connected;
        }
        if (!withTryConnect.value) {
          withTryConnect.value = 0;
        }
        withDeviceItems.value = data as DeviceItem[];
        break;
    }
    if (event.payload.flag === EventFlag.timeout) {
      MessagePlugin.error({ content: "连接超时，请重试" });
    }
  });
}
