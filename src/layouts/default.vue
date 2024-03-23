<script setup lang="ts">
const { activeMenu } = storeToRefs(useTempAppStore())

const route = useRoute()
const router = useRouter()

if (route.path !== activeMenu.value)
  activeMenu.value = route.path
console.log(route.path)

const handleUpdateValue = (value: string) => {
  router.push(value)
}
</script>

<template>
  <n-layout>
    <n-layout-header flex-none px-4>
      <n-tabs type="line" v-model:value="activeMenu" @update:value="handleUpdateValue">
        <n-tab name="/">
          主页
        </n-tab>
        <n-tab name="/setting">
          设置
        </n-tab>
      </n-tabs>
    </n-layout-header>
    <n-layout-content flex-1 :native-scrollbar="false">
      <div px-4 py-2>
        <RouterView />
      </div>
    </n-layout-content>
    <n-watermark v-if="true" content="Dev-0.2.0.alpha" cross fullscreen :font-size="16" :line-height="16" :x-offset="12"
      :width="384" :height="256" :y-offset="60" :rotate="-15" />
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