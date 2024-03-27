<script setup lang="ts">

const options = [
  {
    label: '分享配置',
    key: 'share config'
  },
]
</script>

<template>
  <div>
    <n-grid :x-gap="12" :y-gap="12" cols="1 400:2">
      <n-gi>
        <n-card title="n2n">
          <template #header-extra>
            <n-switch />
          </template>
          <template #action>
            <n-space align="baseline">
              <n-button size="tiny" secondary strong>
                快捷配置
              </n-button>
              <n-button size="tiny" secondary strong>
                详细配置
              </n-button>
              <n-button size="tiny" secondary strong>
                组成员
              </n-button>
              <n-dropdown placement="bottom-start" trigger="click" size="small" :options="options">
                <n-button size="tiny" secondary strong>其他</n-button>
              </n-dropdown>
            </n-space>
          </template>
          <n-flex>
            <n-tag size="small" type="success">
              虚拟IP: 10.26.0.2
            </n-tag>
            <n-tag size="small" type="warning">
              组名: 114514
            </n-tag>
            <n-tag size="small" type="warning">
              成员数: 12
            </n-tag>
            <n-tag size="small" type="info">
              状态: 已连接
            </n-tag>
          </n-flex>
        </n-card>
      </n-gi>
      <n-gi>
        <n-card title="组网通用优化">
          <template #header-extra>
            <n-switch />
          </template>
          <n-flex>
            <n-tag size="small">
              全局IP广播
            </n-tag>
            <n-tag size="small" type="success">
              游戏通用
            </n-tag>
          </n-flex>
        </n-card>
      </n-gi>
      <n-gi>
        <n-card title="Vnt">
          <template #header-extra>
            <n-switch disabled />
          </template>
          <n-flex>
            <n-tag size="small">
              敬请期待
            </n-tag>
          </n-flex>
        </n-card>
      </n-gi>
      <n-gi>
        <n-card title="Zerotier">
          <template #header-extra>
            <n-switch disabled />
          </template>
          <n-flex>
            <n-tag size="small">
              敬请期待
            </n-tag>
          </n-flex>
        </n-card>
      </n-gi>
    </n-grid>
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

.n-card> :deep(.n-card-header),
.n-card> :deep(.n-card__content),
.n-card> :deep(.n-card__footer),
.n-card> :deep(.n-card__action) {
  padding: 0.25rem 0.5rem;
}
</style>
