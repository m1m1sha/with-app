<script setup lang="ts">

</script>

<template>
  <n-layout>
    <n-layout-header p-2 data-tauri-drag-region>
      <HeaderBar data-tauri-drag-region />
    </n-layout-header>
    <n-layout-content flex-1 :native-scrollbar="false">
      <div p-2>
        <RouterView />
      </div>
    </n-layout-content>
    <n-watermark v-if="isDev()" :content="`Dev-${PKG().version}`" cross fullscreen :font-size="16" :line-height="16"
      :x-offset="12" :width="384" :height="256" :y-offset="60" :rotate="-15" />
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