<script setup lang="ts">
import type { TabValue, TabsProps } from 'tdesign-vue-next'

const configStore = useConfigStore()
const appStore = useAppStore()
const { settingTab, winIPBroadcastStatus } = storeToRefs(appStore)
const { config } = storeToRefs(configStore)
const changeHandler: TabsProps['onChange'] = (val: TabValue) => {
  settingTab.value = val.toString()
}

const toggleIPBroadcast = async () => {
  if (!winIPBroadcastStatus.value) {
    await winIPBroadcastStart();
  } else {
    await winIPBroadcastStop();
  }
}

onUnmounted(async () => {
  await configStore.saveConfig()
})
</script>

<template>
  <t-tabs v-model="settingTab" h-full @change="changeHandler">
    <t-tab-panel value="basic" label="基本">
      <t-form>
        <t-form-item label="网卡跃点" help="默认为0(自动), 1(最高优先)">
          <t-input-number v-model="config.with.metric" theme="column"></t-input-number>
        </t-form-item>
        <t-form-item label="强制TCP" help="建议仅在UDP丢包严重时启用">
          <t-switch v-model="config.with.tcp" />
        </t-form-item>
        <t-form-item label="延迟优先" help="寻找低延迟通道">
          <t-switch v-model="config.with.latency" />
        </t-form-item>
      </t-form>
    </t-tab-panel>
    <t-tab-panel value="mode" label="模式">
      <t-form>
        <t-form-item label="加密模式" help="需要选择相同加密模式">
          <t-select v-model="config.with.cipher">
            <t-option key="aes-gcm" label="aes-gcm" value="AesGcm" />
            <t-option key="aes-cbc" label="aes-cbc" value="AesCbc" />
            <t-option key="aes-ecb" label="aes-ecb" value="AesEcb" />
            <t-option key="sm4-cbc" label="sm4-cbc" value="Sm4Cbc" />
            <t-option key="none" label="无加密" value="None" />
          </t-select>
        </t-form-item>
        <t-form-item label="打洞模式" help="按实际情况选择">
          <t-select v-model="config.with.punch">
            <t-option key="ipv4" label="ipv4" value="Ipv4" />
            <t-option key="ipv6" label="ipv6" value="Ipv6" />
            <t-option key="all" label="ipv4/6" value="All" />
          </t-select>
        </t-form-item>
        <t-form-item label="通道模式" help="按实际情况选择">
          <t-select v-model="config.with.channel">
            <t-option key="relay" label="中转" value="Relay" />
            <t-option key="p2p" label="p2p" value="P2p" />
            <t-option key="all" label="中转/p2p" value="All" />
          </t-select>
        </t-form-item>
      </t-form>
    </t-tab-panel>
    <t-tab-panel value="tool" label="工具">
      <t-form>
        <t-form-item label="IP广播" help="winIPBroadcast, 默认启用">
          <t-tag @click="toggleIPBroadcast" :theme="winIPBroadcastStatus ? 'danger' : 'success'">{{ winIPBroadcastStatus
    ? '停用' : '启用'
            }}</t-tag>
        </t-form-item>
        <t-form-item label="强制IP绑定" help="forceBindIP, 强制绑定IP到某个进程" v-if="false">
          <t-tag @click="toggleIPBroadcast" :theme="winIPBroadcastStatus ? 'danger' : 'success'">{{ winIPBroadcastStatus
    ? '停用' : '启用'
            }}</t-tag>
        </t-form-item>
      </t-form>
    </t-tab-panel>
  </t-tabs>
</template>

<style scoped lang="postcss">
.t-tab-panel {
  @apply p-4;
}
</style>
