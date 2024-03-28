import { defineStore } from "pinia";
import { AppConfig, DEFAULT_APP_CONFIG } from "~/types/app";

export const usePersistAppStore = defineStore("persist_app", () => {
  const _config = ref<AppConfig>(DEFAULT_APP_CONFIG);
  const appConfig = computed(() => _config.value);

  async function updateAppConfig() {
    await writeConfig("app", JSON.stringify(appConfig.value));
  }

  async function syncAppConfig() {
    const configString = await readConfig("app");
    if (configString) {
      try {
        const config = JSON.parse(configString) as AppConfig;
        _config.value = config;
      } catch {}
    }
  }

  return {
    _config,
    appConfig,
    updateAppConfig,
    syncAppConfig,
  };
});
