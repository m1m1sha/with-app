import { checkUpdate } from "@tauri-apps/api/updater";
import { MessagePlugin } from "tdesign-vue-next";

const appStore = useAppStore();
const { appUpdaterVisible } = storeToRefs(appStore);

export async function checkForUpdates() {
  try {
    const { shouldUpdate } = await checkUpdate();

    if (shouldUpdate) {
      appUpdaterVisible.value = true;
    } else {
      MessagePlugin.info(`当前已是最新版本`);
    }
  } catch (error) {
    MessagePlugin.error(`获取更新信息失败`);
  }
}
