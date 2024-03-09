<script setup lang="ts">

</script>

<template>
  <div>
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
        <t-form-item label="Token">
          <t-input />
        </t-form-item>
        <t-form-item label="密码">
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
