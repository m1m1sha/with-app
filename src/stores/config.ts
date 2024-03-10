import { defineStore } from 'pinia'
import {
  BaseDirectory,
  exists,
  readTextFile,
  writeTextFile,
} from '@tauri-apps/api/fs'

export const useConfigStore = defineStore('config', () => {
  const config = ref<config>({ ...DEFAULT_CONFIG })

  async function loadConfig() {
    await existsConfig()

    if (
      await exists(CONFIG_FILE, {
        dir: BaseDirectory.AppConfig,
      })
    ) {
      const content = await readConfig(CONFIG_FILE)

      try {
        const data = JSON.parse(content)
        config.value = data
      }
      catch (e) {}
    }

    saveConfig()
  }

  async function saveConfig() {
    await existsConfig()
    await writeConfig(CONFIG_FILE, JSON.stringify(config.value))
  }

  return {
    config,
    loadConfig,
    saveConfig,
  }
})
