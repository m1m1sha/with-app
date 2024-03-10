<script setup lang="ts">
import { StickyToolProps, TableProps } from 'tdesign-vue-next';

const configStore = useConfigStore()
const { config } = storeToRefs(configStore)

const appStore = useAppStore()
const { withStatus, withRoutes } = storeToRefs(appStore)
const visible = ref(false);

const columns: TableProps['columns'] = [
  {
    colKey: 'ip',
    title: 'ip',
  }, {
    colKey: 'channel',
    title: '类型',
  }, {
    colKey: 'rt',
    title: '延迟(ms)',
  }
];
async function start() {
  const cfg = JSON.parse(JSON.stringify(config.value.with))
  await configStore.saveConfig();
  await withStart(cfg)
}
async function stop() {
  await withStop()
}

const handleClick: StickyToolProps['onClick'] = (context) => {
  if (context.item.popup === "组用户") {
    visible.value = true
  }
};
</script>

<template>
  <div h-full w-full flex items-center justify-center>
    <t-space direction="vertical" w-full pr-4>
      <t-form>
        <t-form-item label="服务器" help="公共服务器请使用复杂Token, 6位以上">
          <t-input v-model="config.with.server" :maxlength="32" show-limit-number />
        </t-form-item>
        <t-form-item label="Token" help="输入相同的才能进入同一局域网哦">
          <t-input v-model="config.with.token" placeholder="组网使用的相同标识" :maxlength="32" show-limit-number />
        </t-form-item>
        <t-form-item label="密码" help="可以不填, 要填大家得填一样的">
          <t-input v-model="config.with.passwd" type="password" placeholder="组网使用的相同密码" :maxlength="32"
            show-limit-number />
        </t-form-item>
      </t-form>
      <div flex items-center justify-center>
        <t-button w-120px @click="start" v-if="withStatus === 'stopped'">
          启动！
        </t-button>
        <t-button theme="danger" w-120px @click="stop" v-else>
          关闭
        </t-button>
      </div>
    </t-space>
    <t-sticky-tool type="compact" v-if="withStatus !== 'connected'" @click="handleClick">
      <t-sticky-item popup="组用户">
        <template #icon>
          <t-icon name="user-list" />
        </template>
      </t-sticky-item>
    </t-sticky-tool>
    <t-dialog :footer="false" :header="false" :closeOnEscKeydown="false" :closeOnOverlayClick="false"
      preventScrollThrough showOverlay showInAttachedElement v-model:visible="visible">
      <t-table :stripe="true" size="small" height="50%" :data="withRoutes" :columns="columns" row-key="ip">
        <template #empty>
          <span
            style="display: flex; align-items: center; justify-content: center; height: 40px; color: var(--td-text-color-placeholder)">
            😊 暂时还未发现其他组网设备
          </span>
        </template>
      </t-table>
    </t-dialog>
  </div>
</template>

<style scoped lang="postcss">
.t-sticky-tool {
  right: 1rem !important;
  bottom: 2.5rem !important;
}

:global(.t-dialog) {
  @apply p-1 mx-4;
}

:global(.t-dialog__body) {
  @apply p-0;
}
</style>
