import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";
import { MessagePlugin } from "tdesign-vue-next";

export async function checkForUpdates() {
  const unlisten = await onUpdaterEvent(({ error, status }) => {
    switch (status) {
      case "UPTODATE":
        MessagePlugin.info("当前已是最新版本");
        break;
      case "DONE":
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
    const { shouldUpdate, manifest } = await checkUpdate();

    if (shouldUpdate) {
      MessagePlugin.info(
        `当前即将更新 v${manifest?.version}, ${manifest?.date}`
      );
      // Install the update. This will also restart the app on Windows!
      await installUpdate();
      MessagePlugin.info(`更新完毕, 软件准备重启`);
      // On macOS and Linux you will need to restart the app manually.
      // You could use this step to display another confirmation dialog.
      await relaunch();
    }
  } catch (error) {
    MessagePlugin.error(`更新发生错误: ${error}`);
  }

  // you need to call unlisten if your handler goes out of scope, for example if the component is unmounted.
  unlisten();
}
