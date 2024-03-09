<script setup lang="ts">
import type { MenuProps } from 'tdesign-vue-next'

const router = useRouter()

const changeHandler: MenuProps['onChange'] = (active) => {
  router.push(active)
}

const menu = ref('/')
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
        Copyright @ 2024-{{ new Date().getFullYear() }} m1m1sha. All Rights
        Reserved
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
    @apply px-2 py-4;
  }

  & :deep(.t-layout__footer) {
    @apply p-2 text-center;
  }
}
</style>
