<script setup lang="ts">

</script>

<template>
  <div flex flex-col>
    <div flex-1>
      <n-form label-placement="left" label-align="right" label-width="auto">
        <n-form-item id="server" label="服务器" path="inputValue">
          <n-input type="text" placeholder="服务器" />
        </n-form-item>
        <n-form-item id="group" label="组" path="inputValue">
          <n-input-group>
            <n-input type="text" :style="{ width: '60%' }" placeholder="组名" />
            <n-input type="text" :style="{ width: '40%' }" placeholder="组密码" />
          </n-input-group>
        </n-form-item>
        <n-form-item id="personal" label="个人" path="inputValue">
          <n-input-group>
            <n-input type="text" :style="{ width: '40%' }" placeholder="昵称" />
            <n-input type="text" :style="{ width: '35%' }" placeholder="虚拟ip" />
            <n-checkbox class="input-group">自动</n-checkbox>
          </n-input-group>
        </n-form-item>
        <n-form-item>
          <n-button strong secondary>启动</n-button>
        </n-form-item>
      </n-form>
    </div>
  </div>

</template>

<style scoped lang="postcss">
.n-checkbox.input-group {
  & {
    align-items: center !important;
    background-color: #ffffff1a !important;
    width: 25%;
    border-radius: 3px;
    padding-left: 10px;
  }
}
</style>
