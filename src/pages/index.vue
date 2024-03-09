<script setup lang="ts">

</script>

<template>
  <div h-full w-full flex items-center justify-center>
    <t-space direction="vertical">
      <t-form>
        <t-form-item label="服务器">
          <t-select>
            <t-option key="apple" label="Apple" value="apple" />
            <t-option key="orange" label="Orange" value="orange">
              OrangeContentSlot
            </t-option>
            <t-option key="banana" label="Banana" value="banana" />
          </t-select>
        </t-form-item>
        <t-form-item label="Token" help="输入相同的才能进入同一局域网哦">
          <t-input />
        </t-form-item>
        <t-form-item label="密码" help="可以不填, 要填大家得填一样的">
          <t-input />
        </t-form-item>
        <t-form-item>
          <t-button>确定</t-button>
        </t-form-item>
      </t-form>
    </t-space>
  </div>
</template>

<style scoped lang="postcss"></style>
