<script setup lang="ts">

</script>

<template>
  <t-tabs :default-value="0" h-full>
    <t-tab-panel :value="0" label="基本">
      <t-form>
        <t-form-item label="加密模式">
          <t-select default-value="4">
            <t-option key="aes-gcm" label="aes-gcm(一般选这个)" value="0" />
            <t-option key="aes-cbc" label="aes-cbc" value="1" />
            <t-option key="aes-ecb" label="aes-ecb" value="2" />
            <t-option key="sm4-cbc" label="sm4-cbc" value="3" />
            <t-option key="none" label="无加密" value="4" />
          </t-select>
        </t-form-item>
        <t-form-item label="打洞模式">
          <t-select default-value="2">
            <t-option key="ipv4" label="仅ipv4" value="0" />
            <t-option key="ipv6" label="仅ipv6" value="1" />
            <t-option key="all" label="ipv4或ipv6" value="2" />
          </t-select>
        </t-form-item>
        <t-form-item label="通道模式">
          <t-select default-value="2">
            <t-option key="relay" label="仅中转" value="0" />
            <t-option key="p2p" label="仅p2p" value="1" />
            <t-option key="all" label="中转或p2p" value="2" />
          </t-select>
        </t-form-item>
      </t-form>
    </t-tab-panel>
    <t-tab-panel :value="1" label="高级">
      <t-form>
        <t-form-item label="服务端加密" help="需要服务端支持">
          <t-switch />
        </t-form-item>
        <t-form-item label="强制TCP" help="仅建议UDP丢包验证无法使用情况">
          <t-switch />
        </t-form-item>
        <t-form-item label="指纹校验">
          <t-switch />
        </t-form-item>
        <t-form-item label="延迟优先" help="中转或p2p寻找低延迟模式">
          <t-switch />
        </t-form-item>
      </t-form>
    </t-tab-panel>
    <t-tab-panel :value="2" label="服务器">
      <div />
    </t-tab-panel>
  </t-tabs>
</template>

<style scoped lang="postcss">
.t-tab-panel {
  @apply p-4;
}
</style>
