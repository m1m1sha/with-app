<template>
  <div>
    <t-head-menu>
      <template #operations>
        <div>
          <t-button variant="text" shape="square">
            <template #icon>
              <t-icon name="setting" />
            </template>
          </t-button>
        </div>
      </template>
    </t-head-menu>
    <div px-4 py-6 text="center gray-700 dark:gray-200">
      <RouterView />
    </div>
  </div>
</template>

<style scoped lang="postcss">
.t-head-menu {
  & :deep(.t-head-menu__inner),
  & :deep(.t-menu__operations:not(:empty)) {
    height: var(--td-comp-size-xl);
  }
}
</style>
