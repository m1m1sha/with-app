import { checkUpdate, onUpdaterEvent } from "@tauri-apps/api/updater";
import { MessagePlugin } from "tdesign-vue-next";

const appStore = useAppStore();
const { appUpdaterVisible } = storeToRefs(appStore);

export async function checkForUpdates() {
  const unlisten = await onUpdaterEvent(({ error, status }) => {
    switch (status) {
      case "UPTODATE":
        MessagePlugin.info("当前已是最新版本");
        break;
      case "DONE":
        MessagePlugin.info("更新完成");
        break;
      case "PENDING":
        MessagePlugin.info(`正在等待更新中...`);
        break;
      case "ERROR":
        MessagePlugin.error(`检查更新发生错误: ${error}`);
        break;
    }
  });

  try {
    const { shouldUpdate } = await checkUpdate();

    if (shouldUpdate) {
      appUpdaterVisible.value = true;
    }
  } catch (error) {
    MessagePlugin.error(`获取更新信息失败`);
  }
  unlisten();
}
