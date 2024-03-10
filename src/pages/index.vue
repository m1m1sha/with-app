<script setup lang="ts">
const configStore = useConfigStore()
const { config } = storeToRefs(configStore)
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
          <t-input
            v-model="config.with.passwd" type="password" placeholder="组网使用的相同密码" :maxlength="32"
            show-limit-number
          />
        </t-form-item>
      </t-form>
      <div flex items-center justify-center>
        <t-button w-120px>
          启动！
        </t-button>
      </div>
    </t-space>
  </div>
</template>

<style scoped lang="postcss"></style>
