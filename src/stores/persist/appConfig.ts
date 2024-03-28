import { defineStore } from "pinia";
import { AppConfig, DEFAULT_APP_CONFIG } from "~/types/app";

export const usePersistAppStore = defineStore("persist_app", () => {
  const appConfig = ref<AppConfig>(DEFAULT_APP_CONFIG);

  async function updateAppConfig() {
    await writeConfig("app", JSON.stringify(appConfig.value));
  }

  async function syncAppConfig() {
    const configString = await readConfig("app");
    if (configString) {
      try {
        const config = JSON.parse(configString) as AppConfig;
        appConfig.value = config;
      } catch {}
    }
  }

  return {
    appConfig,
    updateAppConfig,
    syncAppConfig,
  };
});
