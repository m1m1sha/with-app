import { listen } from "@tauri-apps/api/event";
import { MessagePlugin } from "tdesign-vue-next";
import { withConfig } from "./config";
const configStore = useConfigStore();
const { config } = storeToRefs(configStore);

const appStore = useAppStore();
const { withStatus } = storeToRefs(appStore);

interface share {
  a: string; // server,
  b: string; // token,
  c: string; // passwd,
  d: string; // cipher,
  e: string; // punch,
  f: string; // channel,
}

export async function listenForDeeplink() {
  return await listen<string>("deeplink", (event) => {
    console.log(event.payload);

    // withapp://join/base64
    try {
      let action = event.payload.replaceAll("withapp://", "").split("/");
      let flag = action[0];
      let base64 = action[1];

      if (flag === "join") {
        let json = JSON.parse(atob(base64)) as share;
        if (withStatus.value !== WithStatus.Stopped) {
          MessagePlugin.warning("请停止组网后, 再使用分享链接", 3000);
          return;
        }
        config.value.with.server = json.a;
        config.value.with.token = json.b;
        config.value.with.passwd = json.c;
        config.value.with.cipher = json.d;
        config.value.with.punch = json.e;
        config.value.with.channel = json.f;
        MessagePlugin.success("导入分享链接成功!", 3000);
      }
    } catch {
      return;
    }
  });
}

export function shareForDeeplink() {
  let cfg = config.value.with as withConfig;
  let json: share = {
    a: cfg.server,
    b: cfg.token,
    c: cfg.passwd,
    d: cfg.cipher,
    e: cfg.punch,
    f: cfg.channel,
  };
  return `withApp://join/${btoa(JSON.stringify(json))}`;
}
