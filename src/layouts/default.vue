<script setup lang="ts">
import pkg from "../../package.json"
import { UnlistenFn } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window'
import { checkUpdate } from '@tauri-apps/api/updater';
import type { MenuProps } from 'tdesign-vue-next'
import { winIPBroadcastStart } from '~/composables/tool';
const router = useRouter()
const route = useRoute()
const appStore = useAppStore()
const configStore = useConfigStore()
const { menu, appUpdaterInfo, appUpdaterVisible } = storeToRefs(appStore)

if (route.path !== menu.value)
  menu.value = route.path

await configStore.loadConfig()

const changeHandler: MenuProps['onChange'] = (active) => {
  router.push(active.toString())
}
let unlisten: null | UnlistenFn = null
let unlistenDeeplink: null | UnlistenFn = null

onMounted(async () => {
  winIPBroadcastStart(false);
  unlisten = await withEventConnect()
  unlistenDeeplink = await listenForDeeplink()
})

onUnmounted(async () => {
  if (unlisten !== null)
    unlisten()
  if (unlistenDeeplink !== null)
    unlistenDeeplink()

  try {
    const { shouldUpdate, manifest } = await checkUpdate();
    appUpdaterInfo.value = manifest
    if (shouldUpdate)
      appUpdaterVisible.value = true
  } catch { }
})
</script>

<template>
  <t-layout h-full>
    <t-header data-tauri-drag-region>
      <t-head-menu v-model="menu" :collapsed="true" @change="changeHandler">
        <t-menu-item value="/">
          <template #icon>
            <t-icon name="precise-monitor" />
          </template>
          启动
        </t-menu-item>
        <t-menu-item value="/setting">
          <template #icon>
            <t-icon name="setting" />
          </template>
          设置
        </t-menu-item>
        <template #operations>
          <t-button variant="text" shape="square" @click="appWindow.minimize()">
            <template #icon><t-icon name="minus" /></template>
          </t-button>
          <t-button variant="text" shape="square" @click="appWindow.close()">
            <template #icon><t-icon name="close" /></template>
          </t-button>
        </template>
      </t-head-menu>
    </t-header>
    <t-content data-tauri-drag-region>
      <RouterView />
    </t-content>
    <t-footer data-tauri-drag-region>
      <t-space>
        <t-link @click="checkForUpdates">
          当前版本: v{{ pkg.version }}
        </t-link>
        <t-link @click="openExternal('https://github.com/m1m1sha/with-app/issues')">
          有Bug或想法？请提交Issue来帮助完善
        </t-link>
      </t-space>
    </t-footer>
  </t-layout>
</template>

<style scoped lang="postcss">
.t-layout {
  & {
    background: none;
  }

  & :deep(.t-layout__content) {
    @apply py-2 px-4;
  }

  & :deep(.t-layout__footer) {
    @apply p-2 text-center;
  }

  & :deep(.t-layout__header) {
    background: none;
    height: auto;
  }

  & :deep(.t-head-menu__inner),
  :global(.t-menu__operations) {
    height: var(--td-comp-size-l);
    margin: 4px 0;
  }

}
</style>
