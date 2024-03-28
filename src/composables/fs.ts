import {
  exists,
  writeTextFile,
  readTextFile,
  BaseDirectory,
} from "@tauri-apps/plugin-fs";

export async function writeText(
  name: string,
  text: string,
  baseDir: BaseDirectory
) {
  return await writeTextFile(name, text, { baseDir });
}

export async function readText(name: string, baseDir: BaseDirectory) {
  if (!(await exists(name, { baseDir }))) {
    await writeText(name, "", baseDir);
  }
  return await readTextFile(name, { baseDir });
}
