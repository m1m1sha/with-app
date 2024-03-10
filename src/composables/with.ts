import { invoke } from "@tauri-apps/api";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import type { withConfig } from "./config";

const WITH_START = "with_start";
const WITH_STOP = "with_stop";

const WITH_EVENT_CONNECT = "with_event_connect";

export async function withStart(config: withConfig) {
  await invoke(WITH_START, { config });
}

export async function withStop() {
  await invoke(WITH_STOP, {});
}

interface eventPayload {
  flag: string;
  data: string;
}

export async function withEventConnect(): Promise<UnlistenFn> {
  return listen<eventPayload>(WITH_EVENT_CONNECT, (event) => {
    const _data = JSON.parse(event.payload.data);
  });
}
