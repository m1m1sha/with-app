import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import type { withConfig } from "./config";
import { NeedRoute, WithLocalInfo, WithRoute, WithStatus } from "~/stores/app";
import { MessagePlugin } from "tdesign-vue-next";

const WITH_START = "with_start";
const WITH_STOP = "with_stop";

const WITH_EVENT_CONNECT = "with_event_connect";
const appStore = useAppStore();

const {
  withStatus,
  withRoutes,
  withLocalInfo,
  withGatewayRoute,
  withTryConnect,
} = storeToRefs(appStore);
export async function withStart(config: withConfig) {
  withStatus.value = WithStatus.Connecting;
  await invoke(WITH_START, { config });
}

export async function withStop() {
  withStatus.value = WithStatus.Stopping;
  await invoke(WITH_STOP, {});
}

interface eventPayload {
  flag: string;
  data: string;
}

interface eventConnecting {
  count: number;
  address: string;
}

enum eventFlag {
  success = "success",
  stop = "stop",
  register = "register",
  route = "route",
  connecting = "connect",
  timeout = "timeout",
}

enum metricType {
  p2p = 1,
  relay = 2,
}

export async function withEventConnect(): Promise<UnlistenFn> {
  return listen<eventPayload>(WITH_EVENT_CONNECT, async (event) => {
    let data = {};

    try {
      data = JSON.parse(event.payload.data);
    } catch (e) {
      data = {};
    }

    switch (event.payload.flag) {
      case eventFlag.success:
        withStatus.value = WithStatus.Connected;
        withTryConnect.value = 0;
        break;
      case eventFlag.stop || eventFlag.timeout:
        withStatus.value = WithStatus.Stopped;
        withGatewayRoute.value = null;
        withLocalInfo.value = null;
        withRoutes.value = [];
        withTryConnect.value = 0;
        break;
      case eventFlag.register:
        withLocalInfo.value = data as WithLocalInfo;
        break;
      case eventFlag.connecting:
        withTryConnect.value = (data as eventConnecting).count;
        if (withTryConnect.value >= 5) {
          MessagePlugin.error({
            content: "连接超时，请检查服务器是否在线后重试",
          });
          await withStop();
        }
        break;
      case eventFlag.route:
        if (withStatus.value !== WithStatus.Connected) {
          withStatus.value = WithStatus.Connected;
        }
        if (!withTryConnect.value) {
          withTryConnect.value = 0;
        }
        let d = data as WithRoute[];
        let arr: NeedRoute[] = [];

        d.forEach((i) => {
          let route = {
            ip: i.ip,
            rt: `${i.routes[0].rt === 999 ? "连接中" : i.routes[0].rt}`,
            channel: `${i.routes[0].is_tcp ? "tcp" : "udp"} / ${i.routes[0].metric === metricType.p2p ? "p2p" : "relay"}`,
          };
          if (i.ip === withLocalInfo.value!.virtual_gateway) {
            withGatewayRoute.value = route;
          } else {
            arr.push(route);
          }
        });
        withRoutes.value = arr;
        break;
    }
    if (event.payload.flag === eventFlag.timeout) {
      MessagePlugin.error({ content: "连接超时，请重试" });
    }
  });
}
