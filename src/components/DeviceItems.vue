<script setup lang="ts">
import { TableProps } from 'tdesign-vue-next';
import { DeviceItem } from '~/composables/with';
const appStore = useAppStore();
const { withDeviceItems } = storeToRefs(appStore);
const columns: TableProps['columns'] = [
    {
        colKey: 'name',
        title: '设备名称',
        width: 100,
    }, {
        colKey: 'ip',
        title: '虚拟IP',
    }, {
        colKey: 'channel',
        title: '连接 / 通道',
    }, {
        colKey: 'rt',
        title: '延迟(ms)',
    }
];

interface DeviceItemColumn {
    name: String;
    ip: String;

    channel: string
    rt: string;
    online: boolean;
    nat: string;
    public_ips: string[];
    local_ip: string;
    ipv6: string;

    same_secret: boolean;
}

const tableData = computed(() => {
    let list: DeviceItemColumn[] = [];
    withDeviceItems.value.forEach((item: DeviceItem) => {
        let metric = "P2P";
        if (item.metric === DeviceMetric.ClientRelay) {
            metric = "CRelay";
        } else if (item.metric === DeviceMetric.ServerRelay) {
            metric = "SRelay";
        }

        let nat = "Cone";
        if (item.nat_type === NatType.Symmetric) {
            nat = "Symmetric";
        }

        let rt = '连接中...'
        if (item.rt < 999 && item.rt > 0) {
            rt = `${item.rt}`
        } else if (item.rt >= 999) {
            rt = '连接中...'
        } else {
            rt = '网络波动'
        }

        let itemColumn: DeviceItemColumn = {
            channel: `${item.tcp ? "TCP" : "UDP"} / ${metric}`,
            nat,
            ...item,
            ip: item.virtual_ip,
            rt,
        }
        if (item.same_secret && item.online) {
            list.push(itemColumn);
        }
    })
    return list;
})


</script>
<template>
    <t-table :stripe="true" size="small" headerAffixedTop maxHeight="50%" :data="tableData" :columns="columns"
        row-key="ip">
        <template #empty>
            <span
                style="display: flex; align-items: center; justify-content: center; height: 38px; color: var(--td-text-color-placeholder)">
                😊 暂时还未发现其他组网设备
            </span>
        </template>
        <template #name="{ row }">
            <div w-full truncate>{{ row.name }}</div>
        </template>
        <template #ip="{ row }">
            <t-popup trigger="hover">
                <div>{{ row.ip }}</div>
                <template #content>
                    <p>设备网络信息：</p>
                    <p>nat: {{ row.nat }}</p>
                    <p>ip: {{ row.local_ip }}</p>
                    <p>ipv6: {{ row.ipv6 }}</p>
                </template>
            </t-popup>
        </template>
        <template #rt="{ row }">
            <t-tag v-if="row.online" :theme="row.rt >= 80 || row.rt < 1 ? 'warning' : 'success'">{{ row.rt }}</t-tag>
            <t-tag v-else>离线</t-tag>
        </template>
    </t-table>
</template>

<style scoped lang="postcss"></style>