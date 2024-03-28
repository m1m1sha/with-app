import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
  Options,
} from "@tauri-apps/plugin-notification";

const { appConfig } = storeToRefs(usePersistAppStore());
export async function notification(options: Options | string) {
  if (appConfig.value.systemNotification) {
    let permissionGranted = await isPermissionGranted();

    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }

    if (permissionGranted) {
      sendNotification(options);
    }
  }
}
