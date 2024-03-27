<script setup lang="ts">

</script>

<template>
  <n-layout>
    <n-layout-header p-2 data-tauri-drag-region>
      <HeaderBar data-tauri-drag-region />
    </n-layout-header>
    <n-layout-content flex-1 :native-scrollbar="false">
      <n-flex vertical size="large" p-4>
        <n-alert title="发现更新" type="info" :bordered="false" closable>
          <template #icon>
            <n-icon>
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <path fill="currentColor"
                  d="M21 10.12h-6.78l2.74-2.82c-2.73-2.7-7.15-2.8-9.88-.1a6.887 6.887 0 0 0 0 9.8c2.73 2.7 7.15 2.7 9.88 0c1.36-1.35 2.04-2.92 2.04-4.9h2c0 1.98-.88 4.55-2.64 6.29c-3.51 3.48-9.21 3.48-12.72 0c-3.5-3.47-3.53-9.11-.02-12.58a8.987 8.987 0 0 1 12.65 0L21 3zM12.5 8v4.25l3.5 2.08l-.72 1.21L11 13V8z" />
              </svg>
            </n-icon>
          </template>
          <n-button text>
            点击此处进行更新
          </n-button>
        </n-alert>
        <RouterView />
      </n-flex>
    </n-layout-content>
    <n-watermark v-if="isDev()" :content="`${PKG().name} dev-${PKG().version}`" cross fullscreen :font-size="16"
      :line-height="16" :x-offset="12" :width="384" :height="256" :y-offset="60" :rotate="-15" />
  </n-layout>
</template>

<style scoped lang="postcss">
.n-layout {
  & {
    @apply h-full;
  }

  & :deep(.n-layout-scroll-container) {
    @apply w-full flex flex-col;
  }
}
</style>