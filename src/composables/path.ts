import {
  BaseDirectory,
  createDir,
  exists,
  readTextFile,
  writeTextFile,
} from '@tauri-apps/api/fs'

export const APP_CONFIG_DIR = 'cn.smjb.with'
export const CONFIG_FILE = 'config.json'

export async function existsConfig() {
  if (
    !(await exists(APP_CONFIG_DIR, {
      dir: BaseDirectory.Config,
    }))
  ) {
    await createDir(APP_CONFIG_DIR, {
      dir: BaseDirectory.Config,
    })
  }
}

export async function writeConfig(name: string, value: string) {
  await writeTextFile(name, value, {
    dir: BaseDirectory.AppConfig,
  })
}

export async function readConfig(name: string) {
  return await readTextFile(name, {
    dir: BaseDirectory.AppConfig,
  })
}
