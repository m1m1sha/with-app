<script setup lang="ts">
import { UnlistenFn } from '@tauri-apps/api/event';
import type { MenuProps } from 'tdesign-vue-next'
import pkg from "../../package.json"
import { winIPBroadcastStart } from '~/composables/tool';
import { checkUpdate } from '@tauri-apps/api/updater';
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
    const { manifest } = await checkUpdate();
    appUpdaterInfo.value = manifest
    appUpdaterVisible.value = true
  } catch { }
})
</script>

<template>
  <t-layout h-full>
    <t-aside width="auto">
      <t-menu v-model="menu" :collapsed="true" @change="changeHandler">
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
      </t-menu>
    </t-aside>
    <t-layout>
      <t-content>
        <RouterView />
      </t-content>
      <t-footer>
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
  </t-layout>
</template>

<style scoped lang="postcss">
.t-layout {
  & {
    background: none;
  }

  & :deep(.t-layout__content) {
    @apply px-2 pt-4;
  }

  & :deep(.t-layout__footer) {
    @apply p-2 text-center;
  }

  & :deep(.t-layout__sider) {
    background: none;
  }

  & :deep(.t-default-menu) {
    background: none;
  }
}
</style>
