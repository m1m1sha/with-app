<script setup lang="ts">
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
import { MessagePlugin } from 'tdesign-vue-next';
import pkg from "../../package.json"
import { relaunch } from '@tauri-apps/api/process';

const appStore = useAppStore();
const { appUpdaterInfo, appUpdaterVisible, appUpdaterLoading } = storeToRefs(appStore);

async function update() {

    const { shouldUpdate } = await checkUpdate();

    if (shouldUpdate) {
        await winIPBroadcastStop(false)
        MessagePlugin.error(`准备更新...`);
        // Install the update. This will also restart the app on Windows!
        await installUpdate()

        // On macOS and Linux you will need to restart the app manually.
        // You could use this step to display another confirmation dialog.
        await relaunch()

        // if (appUpdaterInfo.value) {
        //     await openExternal(`https://hub.gitmirror.com/https://github.com/m1m1sha/with-app/releases/download/v${appUpdaterInfo.value!.version}/with_${appUpdaterInfo.value!.version}_x64-setup.exe`);
        // }
        // await openExternal(`https://github.com/m1m1sha/with-app/releases`);
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
                <div>当前自动更新存在bug, 更新将打开Github Release</div>
                <t-link @click="openExternal(`https://github.com/m1m1sha/with-app/releases`)">{{ `最新版本号:
                    ${appUpdaterInfo ? 'v' + appUpdaterInfo!.version : ''} ---- 当前(v${pkg.version})`
                    }}</t-link>
                <div>发布日期: {{ appUpdaterInfo?.date }}</div>
                <div w-full truncate>更新简介: {{ appUpdaterInfo?.body }}...</div>
            </t-space>
        </div>
    </t-dialog>
</template>

<style scoped lang="postcss">
:global(.t-dialog__footer) {
    @apply p-0;
}
</style>