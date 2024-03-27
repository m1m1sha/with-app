<script setup lang="ts">
import { darkTheme, zhCN, dateZhCN } from 'naive-ui'
</script>
<template>
  <n-config-provider :theme="darkTheme" :locale="zhCN" :date-locale="dateZhCN">
    <n-message-provider>
      <n-modal-provider>
        <RouterView />
      </n-modal-provider>
    </n-message-provider>
  </n-config-provider>
</template>
