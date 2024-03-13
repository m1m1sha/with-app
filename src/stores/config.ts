import { defineStore } from "pinia";
import { BaseDirectory, exists } from "@tauri-apps/api/fs";

export const useConfigStore = defineStore("config", () => {
  const config = ref<config>({ ...DEFAULT_CONFIG });

  async function loadConfig() {
    await existsConfig();

    if (
      await exists(CONFIG_FILE, {
        dir: BaseDirectory.AppConfig,
      })
    ) {
      const content = await readConfig(CONFIG_FILE);

      try {
        let data: config = JSON.parse(content);
        if (!data.with.metric || data.with.metric < 0) {
          data.with.metric = 0;
        }
        config.value = data;
      } catch (e) {}
    }

    saveConfig();
  }

  async function saveConfig() {
    await existsConfig();
    await writeConfig(CONFIG_FILE, JSON.stringify(config.value));
  }

  return {
    config,
    loadConfig,
    saveConfig,
  };
});
