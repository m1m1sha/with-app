import { BaseDirectory } from "@tauri-apps/plugin-fs";

export async function readConfig(name: string): Promise<string> {
  return await readText(`${name}.wc`, BaseDirectory.AppConfig);
}

export async function writeConfig(name: string, config: string): Promise<void> {
  return await writeText(`${name}.wc`, config, BaseDirectory.AppConfig);
}
