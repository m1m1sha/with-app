<script setup lang="ts">
import { darkTheme, zhCN, dateZhCN } from 'naive-ui'
</script>
<template>
  <n-config-provider :theme="darkTheme" :locale="zhCN" :date-locale="dateZhCN">
    <n-message-provider>
      <n-modal-provider>
        <suspense>
          <RouterView />
          <template #fallback>
            <div w-full h-full flex justify-center>
              <n-spin size="medium">
                <template #description>
                  同步数据中...
                </template>
              </n-spin>
            </div>
          </template>
        </suspense>
      </n-modal-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<style scoped lang="postcss">
:deep(.n-spin-description) {
  --n-bezier: cubic-bezier(.4, 0, .2, 1);
  --n-size: 34px;
  --n-text-color: #63e2b7;
}
</style>
