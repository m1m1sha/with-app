import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import type { withConfig } from "./config";
import { NeedRoute, WithLocalInfo, WithRoute, WithStatus } from "~/stores/app";

const WITH_START = "with_start";
const WITH_STOP = "with_stop";

const WITH_EVENT_CONNECT = "with_event_connect";
const appStore = useAppStore();

const { withStatus, withRoutes, withLocalInfo } = storeToRefs(appStore);
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

export async function withEventConnect(): Promise<UnlistenFn> {
  return listen<eventPayload>(WITH_EVENT_CONNECT, (event) => {
    let data = {};

    try {
      data = JSON.parse(event.payload.data);
    } catch (e) {
      data = {};
    }

    if (event.payload.flag === "success") {
      withStatus.value = WithStatus.Connecting;
    }

    if (event.payload.flag === "stop") {
      withStatus.value = WithStatus.Stopped;
    }

    if (event.payload.flag === "register") {
      withLocalInfo.value = data as WithLocalInfo;
    }

    if (event.payload.flag === "route") {
      if (withStatus.value !== WithStatus.Connected) {
        withStatus.value = WithStatus.Connected;
      }
      let d = data as WithRoute[];
      let arr: NeedRoute[] = [
        {
          ip: withLocalInfo.value!.virtual_ip,
          rt: "0",
          channel: "本机",
        },
      ];

      d.forEach((i) => {
        arr.push({
          ip: i.ip,
          rt: i.routes[0].rt,
          channel:
            (i.routes[0].is_tcp ? "tcp" : "udp") +
            (i.ip === withLocalInfo.value!.virtual_gateway ? "(网关)" : ""),
        });
      });
      withRoutes.value = arr;
    }
  });
}
