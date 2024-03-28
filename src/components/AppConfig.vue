<script setup lang="ts">

const { appConfigVisible } = storeToRefs(useTempAppStore());
const { _config } = storeToRefs(usePersistAppStore());

async function changeConfig() {
  await usePersistAppStore().updateAppConfig()
}

</script>
<template>
  <n-modal v-model:show="appConfigVisible">
    <n-card title="设置" closable @close="appConfigVisible = false">
      <n-form label-placement="left" label-width="auto" size="small">
        <n-form-item label="开机启动">
          <n-switch v-model:value="_config.autoStart" @update:value="changeConfig" />
        </n-form-item>
        <n-form-item label="启动最小化">
          <n-switch v-model:value="_config.startMinimize" @update:value="changeConfig" />
        </n-form-item>
        <n-form-item label="自动检测更新">
          <n-switch v-model:value="_config.autoUpdate" @update:value="changeConfig" />
        </n-form-item>
        <n-form-item label="系统消息通知">
          <n-switch v-model:value="_config.systemNotification" @update:value="changeConfig" />
        </n-form-item>
      </n-form>
    </n-card>
  </n-modal>

</template>
<style scoped lang="postcss">
.n-card {
  @apply w-90%;
}

.n-card> :deep(.n-card-header),
.n-card> :deep(.n-card__content),
.n-card> :deep(.n-card__footer),
.n-card> :deep(.n-card__action) {
  @apply p-2;
}
</style>