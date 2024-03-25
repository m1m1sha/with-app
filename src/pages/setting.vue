<script setup lang="ts">
const { advancedSetting } = storeToRefs(useTempAppStore())


const EncryptModeOptions = [
    {
        label: "无加密",
        value: 'A1'
    },
    {
        label: "TwoFish",
        value: 'A2'
    },
    {
        label: "AES-CBC",
        value: 'A3'
    },
    {
        label: "ChaCha20",
        value: 'A4'
    },
    {
        label: "Speck-CTR",
        value: 'A15'
    },

]

</script>

<template>
    <div flex>
        <div flex-1>
            <n-form label-placement="left" label-align="right" label-width="auto">
                <n-form-item id="E" label="接受多播 -E" path="inputValue">
                    <n-switch />
                </n-form-item>
                <n-form-item id="x" label="网卡跃点 -x" path="inputValue">
                    <n-radio-group>
                        <n-space>
                            <n-radio :value="0">自动</n-radio>
                            <n-radio :value="1">最高优先</n-radio>
                        </n-space>
                    </n-radio-group>
                </n-form-item>
                <n-form-item id="S" label="中继模式 -S" path="inputValue">
                    <n-radio-group>
                        <n-space>
                            <n-radio :value="0">不使用</n-radio>
                            <n-radio :value="1">UDP中继</n-radio>
                        </n-space>
                    </n-radio-group>
                </n-form-item>
                <n-form-item id="A" label="加密模式 -A" path="inputValue">
                    <n-select :options="EncryptModeOptions" />
                </n-form-item>
                <n-form-item id="J" label="节点密钥 -J" path="inputValue">
                    <n-input type="text" />
                </n-form-item>
                <n-divider dashed :style="{ 'margin-top': 0 }">
                    <n-button text @click="advancedSetting = !advancedSetting">
                        {{ advancedSetting ? '关闭' : '开启' }}高级配置
                    </n-button>
                </n-divider>
                <n-collapse-transition :show="advancedSetting">
                    <n-form label-placement="left" label-align="right" label-width="auto">
                        <n-form-item id="H" label="报头加密 -H" path="inputValue">
                            <n-switch />
                        </n-form-item>
                        <n-form-item id="selectRtt" label="RTT择优" path="inputValue">
                            <n-switch />
                        </n-form-item>
                        <n-form-item id="selectMac" label="MAC择优" path="inputValue">
                            <n-switch />
                        </n-form-item>
                        <n-form-item id="D" label="PMTU -D" path="inputValue">
                            <n-switch />
                        </n-form-item>
                        <n-form-item id="r" label="允许转发 -r" path="inputValue">
                            <n-switch />
                        </n-form-item>
                        <n-form-item id="i" label="打洞间隔 -i" path="inputValue">
                            <n-input-number :min="5" :max="60" />
                        </n-form-item>
                        <n-form-item id="L" label="打洞TTL -L" path="inputValue">
                            <n-input-number :min="0" :max="1000" />
                        </n-form-item>
                        <n-form-item id="M" label="MTU -M" path="inputValue">
                            <n-input-number :min="1000" :max="3000" />
                        </n-form-item>
                        <n-form-item id="p" label="本地端口 -p" path="inputValue">
                            <n-input-number :min="0" :max="65535" />
                        </n-form-item>
                        <n-form-item id="e" label="本地广播 -e" path="inputValue">
                            <n-input type="text" />
                        </n-form-item>
                        <n-form-item id="m" label="MAC地址 -m" path="inputValue">
                            <n-input type="text" />
                        </n-form-item>
                        <n-form-item id="d" label="网卡名称 -d" path="inputValue">
                            <n-input type="text" />
                        </n-form-item>

                        <n-form-item id="P" label="节点公钥 -P" path="inputValue">
                            <n-input type="text" />
                        </n-form-item>
                    </n-form>
                </n-collapse-transition>
            </n-form>
        </div>
        <div flex-none w-100px ml-24px>
            <n-anchor fixed affix :show-background="false" style="z-index: 1" :bound="48">
                <n-anchor-link title="接受多播" href="#E" />
                <n-anchor-link title="网卡跃点" href="#X" />
                <n-anchor-link title="中继模式" href="#S" />
                <n-anchor-link title="加密模式" href="#A" />
                <n-anchor-link title="节点密钥" href="#J" />
                <n-collapse-transition :show="advancedSetting" mt-6.5px>
                    <n-anchor-link title="报头加密" href="#H" />
                    <n-anchor-link title="RTT择优" href="#selectRtt" />
                    <n-anchor-link title="MAC择优" href="#selectMac" />
                    <n-anchor-link title="PMTU" href="#D" />
                    <n-anchor-link title="允许转发" href="#r" />
                    <n-anchor-link title="打洞间隔" href="#i" />
                    <n-anchor-link title="打洞TTL" href="#L" />
                    <n-anchor-link title="MTU" href="#M" />
                    <n-anchor-link title="本地端口" href="#p" />
                    <n-anchor-link title="本地广播IP" href="#e" />
                    <n-anchor-link title="MAC地址" href="#m" />
                    <n-anchor-link title="网卡名称" href="#d" />
                    <n-anchor-link title="节点公钥" href="#P" />

                </n-collapse-transition>
            </n-anchor>
        </div>
    </div>
</template>

<style scoped lang="postcss"></style>