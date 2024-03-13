import { invoke } from "@tauri-apps/api";
import { MessagePlugin } from "tdesign-vue-next";
const appStore = useAppStore();

const { winIPBroadcastStatus } = storeToRefs(appStore);

export async function winIPBroadcastStart(successShow: boolean) {
  let rep = await invoke("win_ip_broadcast_start");
  if (rep !== null) {
    MessagePlugin.warning(`winIPBroadcast 启动失败: ${rep}`);
    return;
  }
  winIPBroadcastStatus.value = true;
  if (successShow) MessagePlugin.success("winIPBroadcast 启动成功");
}

export async function winIPBroadcastStop(successShow: boolean) {
  await invoke("win_ip_broadcast_stop");
  winIPBroadcastStatus.value = false;
  if (successShow) MessagePlugin.success("winIPBroadcast 关闭成功");
}
