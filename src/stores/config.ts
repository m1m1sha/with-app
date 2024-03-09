import { defineStore } from 'pinia'
import {
  BaseDirectory,
  mkdir,
  readTextFile,
  writeTextFile,
} from '@tauri-apps/plugin-fs'
import { APP_CONFIG_DIR } from '~/composables/path'
import { DEFAULT_CONFIG } from '~/composables/config'

export const useConfigStore = defineStore('config', () => {
  const config = ref<config>({ ...DEFAULT_CONFIG })

  async function loadConfig() {
    await mkdir(APP_CONFIG_DIR, {
      baseDir: BaseDirectory.Config,
      recursive: true,
    })

    const content = await readTextFile(CONFIG_FILE, {
      baseDir: BaseDirectory.AppConfig,
    })

    try {
      const data = JSON.parse(content)
      config.value = data
    }
    catch (e) {}

    saveConfig()
  }

  async function saveConfig() {
    await writeTextFile(CONFIG_FILE, JSON.stringify(config.value), {
      baseDir: BaseDirectory.AppConfig,
    })
  }

  return {
    config,
    loadConfig,
    saveConfig,
  }
})
