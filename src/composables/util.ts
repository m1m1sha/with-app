import { shell } from "@tauri-apps/api";

export async function openExternal(
  path: string,
  openWith?: string | undefined
) {
  return await shell.open(path, openWith);
}
