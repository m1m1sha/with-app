<script setup lang="ts">
import { darkTheme, zhCN, dateZhCN } from 'naive-ui'
</script>
<template>
  <n-config-provider :theme="darkTheme" :locale="zhCN" :date-locale="dateZhCN">
    <RouterView />
  </n-config-provider>
</template>
