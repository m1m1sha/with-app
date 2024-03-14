<script setup lang="ts">
import { StickyToolProps } from 'tdesign-vue-next';
import { WithStatus } from '~/stores/app';
import { writeText } from '@tauri-apps/api/clipboard';
import { MessagePlugin } from "tdesign-vue-next";

const configStore = useConfigStore()
const { config } = storeToRefs(configStore)

const appStore = useAppStore()
const { withStatus, withLocalInfo, withTryConnect } = storeToRefs(appStore)
const visible = ref(false);

async function start() {
  const cfg = JSON.parse(JSON.stringify(config.value.with))
  await configStore.saveConfig();
  await withStart(cfg)
}
async function stop() {
  await withStop()
}

const handleClick: StickyToolProps['onClick'] = async (context) => {


  switch (context.item.popup) {
    case "组用户":
      visible.value = true
      break;
    case "分享":
      let share = shareForDeeplink();
      console.log(share.length)
      await writeText(share);
      MessagePlugin.success("分享链接已复制到剪贴板!");
      break;
  }
};
</script>

<template>
  <div h-full w-full flex items-center justify-center>
    <t-space direction="vertical" w-full pr-4>
      <t-form>
        <t-form-item label="服务器" help="公共服务器请使用复杂Token, 6位以上">
          <t-input v-model="config.with.server" :maxlength="32" show-limit-number
            :disabled="withStatus != WithStatus.Stopped" />
        </t-form-item>
        <t-form-item label="Token" help="输入相同的才能进入同一局域网哦">
          <t-input v-model="config.with.token" placeholder="组网使用的相同标识" :maxlength="32" show-limit-number
            :disabled="withStatus != WithStatus.Stopped" />
        </t-form-item>
        <t-form-item label="密码" help="可以不填, 要填大家得填一样的">
          <t-input v-model="config.with.passwd" type="password" placeholder="组网使用的相同密码" :maxlength="32"
            show-limit-number :disabled="withStatus != WithStatus.Stopped" />
        </t-form-item>
      </t-form>
      <div w-full flex items-center justify-center>
        <t-button class="action start" @click="start" v-if="withStatus !== WithStatus.Connected"
          :loading="withStatus === WithStatus.Connecting">
          {{ withStatus === WithStatus.Connecting ? withTryConnect > 1 ? `正在尝试第${withTryConnect - 1}次重连` : '启动中...' :
            "启动" }}
        </t-button>
        <div v-if="(withStatus === WithStatus.Connecting && withTryConnect >= 1)" mx-2></div>
        <t-button class="action stop" theme="danger" @click="stop"
          v-if="withStatus === WithStatus.Connected || (withStatus === WithStatus.Connecting && withTryConnect >= 1)">
          关闭
        </t-button>
      </div>
    </t-space>
    <t-sticky-tool type="compact" @click="handleClick" v-show="withStatus === WithStatus.Connected">
      <t-sticky-item popup="分享">
        <template #icon>
          <t-icon name="share" />
        </template>
      </t-sticky-item>
      <t-sticky-item popup="组用户">
        <template #icon>
          <t-icon name="user-list" />
        </template>
      </t-sticky-item>
    </t-sticky-tool>
    <t-dialog :footer="false" :header="false" :closeBtn="false" preventScrollThrough showOverlay showInAttachedElement
      v-model:visible="visible">
      <div flex justify-between>
        <div>{{ withLocalInfo ? `本机：${withLocalInfo!.virtual_ip}` : '本机信息获取中' }}</div>
        <div>{{ `本机名称：${config.with.name ? config.with.name : '设备识别码'}` }}</div>
      </div>
      <DeviceItems />
    </t-dialog>
    <updater />
  </div>
</template>

<style scoped lang="postcss">
.t-sticky-tool {
  right: 0.5rem !important;
  bottom: 2rem !important;
}

:global(.t-dialog) {
  @apply p-1 mx-4;
}

:global(.t-dialog__body) {
  @apply p-0;
}

:global(.t-table__empty) {
  min-height: auto;
}

:global(.t-sticky-tool .t-sticky-item) {
  @apply w-32px h-32px flex items-center justify-center;
}

:global(.t-sticky-tool .t-sticky-item .t-icon) {
  @apply m-0;
}

.action {
  will-change: filter;
  transition: filter 300ms;
}

.start:hover {
  filter: drop-shadow(0 0 2em #699ef5aa);
}

.stop:hover {
  filter: drop-shadow(0 0 2em #de6670aa);
}
</style>
