<script setup lang="ts">
const configStore = useConfigStore()
const { config } = storeToRefs(configStore)
</script>

<template>
  <div h-full w-full flex items-center justify-center>
    <t-space direction="vertical" w-full pr-4>
      <t-form>
        <t-form-item label="服务器">
          <t-input v-model="config.with.server" :maxlength="32" show-limit-number />
        </t-form-item>
        <t-form-item label="Token" help="输入相同的才能进入同一局域网哦">
          <t-input v-model="config.with.token" :maxlength="32" show-limit-number />
        </t-form-item>
        <t-form-item label="密码" help="可以不填, 要填大家得填一样的">
          <t-input v-model="config.with.passwd" type="password" :maxlength="32" show-limit-number />
        </t-form-item>
        <t-form-item>
          <t-button>确定</t-button>
        </t-form-item>
      </t-form>
    </t-space>
  </div>
</template>

<style scoped lang="postcss"></style>
