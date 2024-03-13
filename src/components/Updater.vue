<script setup lang="ts">
import { checkUpdate } from '@tauri-apps/api/updater';
import { MessagePlugin } from 'tdesign-vue-next';
import pkg from "../../package.json"

const appStore = useAppStore();
const { appUpdaterInfo, appUpdaterVisible, appUpdaterLoading } = storeToRefs(appStore);

async function update() {

    const { shouldUpdate } = await checkUpdate();

    if (shouldUpdate) {
        await winIPBroadcastStop(false)
        if (appUpdaterInfo.value) {
            await openExternal(`https://hub.gitmirror.com/https://github.com/m1m1sha/with-app/releases/download/v${appUpdaterInfo.value!.version}/with_${appUpdaterInfo.value!.version}_x64-setup.exe`);
        }
        await openExternal(`https://github.com/m1m1sha/with-app/releases`);
    }
}

onMounted(async () => {
    try {
        const { manifest } = await checkUpdate();
        appUpdaterInfo.value = manifest
    } catch {
        MessagePlugin.error(`获取版本信息失败`);
    }
})

</script>
<template>
    <t-dialog preventScrollThrough :closeBtn="false" confirmBtn="更新" :onConfirm="update"
        :confirmLoading="appUpdaterLoading" showOverlay showInAttachedElement theme="info"
        :header="`发现新版本${appUpdaterInfo ? ': v' + appUpdaterInfo!.version : ''}`" v-model:visible="appUpdaterVisible">
        <div px-8>
            <t-space direction="vertical" size="small">
                <div>当前自动更新存在bug, 更新将自动打开Github Release页面</div>
                <div> {{ `最新版本号: ${appUpdaterInfo ? 'v' + appUpdaterInfo!.version : ''} ---- 当前(v${pkg.version})` }}
                </div>
                <div>发布日期: {{ appUpdaterInfo?.date }}</div>
                <div>更新简介: {{ appUpdaterInfo?.body }}</div>
            </t-space>
        </div>
    </t-dialog>
</template>

<style scoped lang="postcss">
:global(.t-dialog__footer) {
    @apply p-0;
}
</style>