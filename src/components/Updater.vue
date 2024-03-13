<script setup lang="ts">
import { relaunch } from '@tauri-apps/api/process';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
import { MessagePlugin } from 'tdesign-vue-next';
import pkg from "../../package.json"

const appStore = useAppStore();
const { appUpdaterInfo, appUpdaterVisible, appUpdaterLoading } = storeToRefs(appStore);

async function update() {
    const { shouldUpdate } = await checkUpdate();

    if (shouldUpdate) {
        await winIPBroadcastStop(false)
        MessagePlugin.info(`当前即将开始更新`);
        appUpdaterLoading.value = true;
        // Install the update. This will also restart the app on Windows!
        await installUpdate();
        MessagePlugin.info(`更新完毕, 软件准备重启`);
        // On macOS and Linux you will need to restart the app manually.
        // You could use this step to display another confirmation dialog.
        await relaunch();
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