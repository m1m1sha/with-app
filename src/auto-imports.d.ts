/* eslint-disable */
/* prettier-ignore */
// @ts-nocheck
// noinspection JSUnusedGlobalSymbols
// Generated by unplugin-auto-import
export {}
declare global {
  const APP_CONFIG_DIR: typeof import('./composables/path')['APP_CONFIG_DIR']
  const CONFIG_FILE: typeof import('./composables/path')['CONFIG_FILE']
  const DEFAULT_CONFIG: typeof import('./composables/config')['DEFAULT_CONFIG']
  const EffectScope: typeof import('vue')['EffectScope']
  const WITH_EVENT_CONNECT: typeof import('./composables/with')['WITH_EVENT_CONNECT']
  const WithStatus: typeof import('./stores/app')['WithStatus']
  const acceptHMRUpdate: typeof import('pinia')['acceptHMRUpdate']
  const channelMode: typeof import('./composables/config')['channelMode']
  const checkForUpdates: typeof import('./composables/updater')['checkForUpdates']
  const cipherMode: typeof import('./composables/config')['cipherMode']
  const computed: typeof import('vue')['computed']
  const config: typeof import('./composables/path')['config']
  const createApp: typeof import('vue')['createApp']
  const createPinia: typeof import('pinia')['createPinia']
  const customRef: typeof import('vue')['customRef']
  const defineAsyncComponent: typeof import('vue')['defineAsyncComponent']
  const defineComponent: typeof import('vue')['defineComponent']
  const defineLoader: typeof import('vue-router/auto')['defineLoader']
  const definePage: typeof import('unplugin-vue-router/runtime')['definePage']
  const defineStore: typeof import('pinia')['defineStore']
  const effectScope: typeof import('vue')['effectScope']
  const existsConfig: typeof import('./composables/path')['existsConfig']
  const getActivePinia: typeof import('pinia')['getActivePinia']
  const getCurrentInstance: typeof import('vue')['getCurrentInstance']
  const getCurrentScope: typeof import('vue')['getCurrentScope']
  const h: typeof import('vue')['h']
  const inject: typeof import('vue')['inject']
  const isProxy: typeof import('vue')['isProxy']
  const isReactive: typeof import('vue')['isReactive']
  const isReadonly: typeof import('vue')['isReadonly']
  const isRef: typeof import('vue')['isRef']
  const listenForDeeplink: typeof import('./composables/deeplink')['listenForDeeplink']
  const mapActions: typeof import('pinia')['mapActions']
  const mapGetters: typeof import('pinia')['mapGetters']
  const mapState: typeof import('pinia')['mapState']
  const mapStores: typeof import('pinia')['mapStores']
  const mapWritableState: typeof import('pinia')['mapWritableState']
  const markRaw: typeof import('vue')['markRaw']
  const nextTick: typeof import('vue')['nextTick']
  const onActivated: typeof import('vue')['onActivated']
  const onBeforeMount: typeof import('vue')['onBeforeMount']
  const onBeforeRouteLeave: typeof import('vue-router/auto')['onBeforeRouteLeave']
  const onBeforeRouteUpdate: typeof import('vue-router/auto')['onBeforeRouteUpdate']
  const onBeforeUnmount: typeof import('vue')['onBeforeUnmount']
  const onBeforeUpdate: typeof import('vue')['onBeforeUpdate']
  const onDeactivated: typeof import('vue')['onDeactivated']
  const onErrorCaptured: typeof import('vue')['onErrorCaptured']
  const onMounted: typeof import('vue')['onMounted']
  const onRenderTracked: typeof import('vue')['onRenderTracked']
  const onRenderTriggered: typeof import('vue')['onRenderTriggered']
  const onScopeDispose: typeof import('vue')['onScopeDispose']
  const onServerPrefetch: typeof import('vue')['onServerPrefetch']
  const onUnmounted: typeof import('vue')['onUnmounted']
  const onUpdated: typeof import('vue')['onUpdated']
  const openExternal: typeof import('./composables/util')['openExternal']
  const provide: typeof import('vue')['provide']
  const punchMode: typeof import('./composables/config')['punchMode']
  const reactive: typeof import('vue')['reactive']
  const readConfig: typeof import('./composables/path')['readConfig']
  const readonly: typeof import('vue')['readonly']
  const ref: typeof import('vue')['ref']
  const resolveComponent: typeof import('vue')['resolveComponent']
  const setActivePinia: typeof import('pinia')['setActivePinia']
  const setMapStoreSuffix: typeof import('pinia')['setMapStoreSuffix']
  const shallowReactive: typeof import('vue')['shallowReactive']
  const shallowReadonly: typeof import('vue')['shallowReadonly']
  const shallowRef: typeof import('vue')['shallowRef']
  const shareForDeeplink: typeof import('./composables/deeplink')['shareForDeeplink']
  const storeToRefs: typeof import('pinia')['storeToRefs']
  const toRaw: typeof import('vue')['toRaw']
  const toRef: typeof import('vue')['toRef']
  const toRefs: typeof import('vue')['toRefs']
  const toValue: typeof import('vue')['toValue']
  const triggerRef: typeof import('vue')['triggerRef']
  const unref: typeof import('vue')['unref']
  const useAppStore: typeof import('./stores/app')['useAppStore']
  const useAttrs: typeof import('vue')['useAttrs']
  const useConfigStore: typeof import('./stores/config')['useConfigStore']
  const useCssModule: typeof import('vue')['useCssModule']
  const useCssVars: typeof import('vue')['useCssVars']
  const useLink: typeof import('vue-router/auto')['useLink']
  const useRoute: typeof import('vue-router/auto')['useRoute']
  const useRouter: typeof import('vue-router/auto')['useRouter']
  const useSlots: typeof import('vue')['useSlots']
  const watch: typeof import('vue')['watch']
  const watchEffect: typeof import('vue')['watchEffect']
  const watchPostEffect: typeof import('vue')['watchPostEffect']
  const watchSyncEffect: typeof import('vue')['watchSyncEffect']
  const withEventConnect: typeof import('./composables/with')['withEventConnect']
  const withStart: typeof import('./composables/with')['withStart']
  const withStop: typeof import('./composables/with')['withStop']
  const writeConfig: typeof import('./composables/path')['writeConfig']
  const writeText: typeof import('./composables/path')['writeText']
}
// for type re-export
declare global {
  // @ts-ignore
  export type { Component, ComponentPublicInstance, ComputedRef, ExtractDefaultPropTypes, ExtractPropTypes, ExtractPublicPropTypes, InjectionKey, PropType, Ref, VNode, WritableComputedRef } from 'vue'
  import('vue')
}
// for vue template auto import
import { UnwrapRef } from 'vue'
declare module 'vue' {
  interface GlobalComponents {}
  interface ComponentCustomProperties {
    readonly APP_CONFIG_DIR: UnwrapRef<typeof import('./composables/path')['APP_CONFIG_DIR']>
    readonly CONFIG_FILE: UnwrapRef<typeof import('./composables/path')['CONFIG_FILE']>
    readonly DEFAULT_CONFIG: UnwrapRef<typeof import('./composables/config')['DEFAULT_CONFIG']>
    readonly EffectScope: UnwrapRef<typeof import('vue')['EffectScope']>
    readonly WithStatus: UnwrapRef<typeof import('./stores/app')['WithStatus']>
    readonly acceptHMRUpdate: UnwrapRef<typeof import('pinia')['acceptHMRUpdate']>
    readonly channelMode: UnwrapRef<typeof import('./composables/config')['channelMode']>
    readonly checkForUpdates: UnwrapRef<typeof import('./composables/updater')['checkForUpdates']>
    readonly cipherMode: UnwrapRef<typeof import('./composables/config')['cipherMode']>
    readonly computed: UnwrapRef<typeof import('vue')['computed']>
    readonly createApp: UnwrapRef<typeof import('vue')['createApp']>
    readonly createPinia: UnwrapRef<typeof import('pinia')['createPinia']>
    readonly customRef: UnwrapRef<typeof import('vue')['customRef']>
    readonly defineAsyncComponent: UnwrapRef<typeof import('vue')['defineAsyncComponent']>
    readonly defineComponent: UnwrapRef<typeof import('vue')['defineComponent']>
    readonly definePage: UnwrapRef<typeof import('unplugin-vue-router/runtime')['definePage']>
    readonly defineStore: UnwrapRef<typeof import('pinia')['defineStore']>
    readonly effectScope: UnwrapRef<typeof import('vue')['effectScope']>
    readonly existsConfig: UnwrapRef<typeof import('./composables/path')['existsConfig']>
    readonly getActivePinia: UnwrapRef<typeof import('pinia')['getActivePinia']>
    readonly getCurrentInstance: UnwrapRef<typeof import('vue')['getCurrentInstance']>
    readonly getCurrentScope: UnwrapRef<typeof import('vue')['getCurrentScope']>
    readonly h: UnwrapRef<typeof import('vue')['h']>
    readonly inject: UnwrapRef<typeof import('vue')['inject']>
    readonly isProxy: UnwrapRef<typeof import('vue')['isProxy']>
    readonly isReactive: UnwrapRef<typeof import('vue')['isReactive']>
    readonly isReadonly: UnwrapRef<typeof import('vue')['isReadonly']>
    readonly isRef: UnwrapRef<typeof import('vue')['isRef']>
    readonly listenForDeeplink: UnwrapRef<typeof import('./composables/deeplink')['listenForDeeplink']>
    readonly mapActions: UnwrapRef<typeof import('pinia')['mapActions']>
    readonly mapGetters: UnwrapRef<typeof import('pinia')['mapGetters']>
    readonly mapState: UnwrapRef<typeof import('pinia')['mapState']>
    readonly mapStores: UnwrapRef<typeof import('pinia')['mapStores']>
    readonly mapWritableState: UnwrapRef<typeof import('pinia')['mapWritableState']>
    readonly markRaw: UnwrapRef<typeof import('vue')['markRaw']>
    readonly nextTick: UnwrapRef<typeof import('vue')['nextTick']>
    readonly onActivated: UnwrapRef<typeof import('vue')['onActivated']>
    readonly onBeforeMount: UnwrapRef<typeof import('vue')['onBeforeMount']>
    readonly onBeforeRouteLeave: UnwrapRef<typeof import('vue-router/auto')['onBeforeRouteLeave']>
    readonly onBeforeRouteUpdate: UnwrapRef<typeof import('vue-router/auto')['onBeforeRouteUpdate']>
    readonly onBeforeUnmount: UnwrapRef<typeof import('vue')['onBeforeUnmount']>
    readonly onBeforeUpdate: UnwrapRef<typeof import('vue')['onBeforeUpdate']>
    readonly onDeactivated: UnwrapRef<typeof import('vue')['onDeactivated']>
    readonly onErrorCaptured: UnwrapRef<typeof import('vue')['onErrorCaptured']>
    readonly onMounted: UnwrapRef<typeof import('vue')['onMounted']>
    readonly onRenderTracked: UnwrapRef<typeof import('vue')['onRenderTracked']>
    readonly onRenderTriggered: UnwrapRef<typeof import('vue')['onRenderTriggered']>
    readonly onScopeDispose: UnwrapRef<typeof import('vue')['onScopeDispose']>
    readonly onServerPrefetch: UnwrapRef<typeof import('vue')['onServerPrefetch']>
    readonly onUnmounted: UnwrapRef<typeof import('vue')['onUnmounted']>
    readonly onUpdated: UnwrapRef<typeof import('vue')['onUpdated']>
    readonly openExternal: UnwrapRef<typeof import('./composables/util')['openExternal']>
    readonly provide: UnwrapRef<typeof import('vue')['provide']>
    readonly punchMode: UnwrapRef<typeof import('./composables/config')['punchMode']>
    readonly reactive: UnwrapRef<typeof import('vue')['reactive']>
    readonly readConfig: UnwrapRef<typeof import('./composables/path')['readConfig']>
    readonly readonly: UnwrapRef<typeof import('vue')['readonly']>
    readonly ref: UnwrapRef<typeof import('vue')['ref']>
    readonly resolveComponent: UnwrapRef<typeof import('vue')['resolveComponent']>
    readonly setActivePinia: UnwrapRef<typeof import('pinia')['setActivePinia']>
    readonly setMapStoreSuffix: UnwrapRef<typeof import('pinia')['setMapStoreSuffix']>
    readonly shallowReactive: UnwrapRef<typeof import('vue')['shallowReactive']>
    readonly shallowReadonly: UnwrapRef<typeof import('vue')['shallowReadonly']>
    readonly shallowRef: UnwrapRef<typeof import('vue')['shallowRef']>
    readonly shareForDeeplink: UnwrapRef<typeof import('./composables/deeplink')['shareForDeeplink']>
    readonly storeToRefs: UnwrapRef<typeof import('pinia')['storeToRefs']>
    readonly toRaw: UnwrapRef<typeof import('vue')['toRaw']>
    readonly toRef: UnwrapRef<typeof import('vue')['toRef']>
    readonly toRefs: UnwrapRef<typeof import('vue')['toRefs']>
    readonly toValue: UnwrapRef<typeof import('vue')['toValue']>
    readonly triggerRef: UnwrapRef<typeof import('vue')['triggerRef']>
    readonly unref: UnwrapRef<typeof import('vue')['unref']>
    readonly useAppStore: UnwrapRef<typeof import('./stores/app')['useAppStore']>
    readonly useAttrs: UnwrapRef<typeof import('vue')['useAttrs']>
    readonly useConfigStore: UnwrapRef<typeof import('./stores/config')['useConfigStore']>
    readonly useCssModule: UnwrapRef<typeof import('vue')['useCssModule']>
    readonly useCssVars: UnwrapRef<typeof import('vue')['useCssVars']>
    readonly useLink: UnwrapRef<typeof import('vue-router/auto')['useLink']>
    readonly useRoute: UnwrapRef<typeof import('vue-router/auto')['useRoute']>
    readonly useRouter: UnwrapRef<typeof import('vue-router/auto')['useRouter']>
    readonly useSlots: UnwrapRef<typeof import('vue')['useSlots']>
    readonly watch: UnwrapRef<typeof import('vue')['watch']>
    readonly watchEffect: UnwrapRef<typeof import('vue')['watchEffect']>
    readonly watchPostEffect: UnwrapRef<typeof import('vue')['watchPostEffect']>
    readonly watchSyncEffect: UnwrapRef<typeof import('vue')['watchSyncEffect']>
    readonly withEventConnect: UnwrapRef<typeof import('./composables/with')['withEventConnect']>
    readonly withStart: UnwrapRef<typeof import('./composables/with')['withStart']>
    readonly withStop: UnwrapRef<typeof import('./composables/with')['withStop']>
    readonly writeConfig: UnwrapRef<typeof import('./composables/path')['writeConfig']>
  }
}
declare module '@vue/runtime-core' {
  interface GlobalComponents {}
  interface ComponentCustomProperties {
    readonly APP_CONFIG_DIR: UnwrapRef<typeof import('./composables/path')['APP_CONFIG_DIR']>
    readonly CONFIG_FILE: UnwrapRef<typeof import('./composables/path')['CONFIG_FILE']>
    readonly DEFAULT_CONFIG: UnwrapRef<typeof import('./composables/config')['DEFAULT_CONFIG']>
    readonly EffectScope: UnwrapRef<typeof import('vue')['EffectScope']>
    readonly WithStatus: UnwrapRef<typeof import('./stores/app')['WithStatus']>
    readonly acceptHMRUpdate: UnwrapRef<typeof import('pinia')['acceptHMRUpdate']>
    readonly channelMode: UnwrapRef<typeof import('./composables/config')['channelMode']>
    readonly checkForUpdates: UnwrapRef<typeof import('./composables/updater')['checkForUpdates']>
    readonly cipherMode: UnwrapRef<typeof import('./composables/config')['cipherMode']>
    readonly computed: UnwrapRef<typeof import('vue')['computed']>
    readonly createApp: UnwrapRef<typeof import('vue')['createApp']>
    readonly createPinia: UnwrapRef<typeof import('pinia')['createPinia']>
    readonly customRef: UnwrapRef<typeof import('vue')['customRef']>
    readonly defineAsyncComponent: UnwrapRef<typeof import('vue')['defineAsyncComponent']>
    readonly defineComponent: UnwrapRef<typeof import('vue')['defineComponent']>
    readonly definePage: UnwrapRef<typeof import('unplugin-vue-router/runtime')['definePage']>
    readonly defineStore: UnwrapRef<typeof import('pinia')['defineStore']>
    readonly effectScope: UnwrapRef<typeof import('vue')['effectScope']>
    readonly existsConfig: UnwrapRef<typeof import('./composables/path')['existsConfig']>
    readonly getActivePinia: UnwrapRef<typeof import('pinia')['getActivePinia']>
    readonly getCurrentInstance: UnwrapRef<typeof import('vue')['getCurrentInstance']>
    readonly getCurrentScope: UnwrapRef<typeof import('vue')['getCurrentScope']>
    readonly h: UnwrapRef<typeof import('vue')['h']>
    readonly inject: UnwrapRef<typeof import('vue')['inject']>
    readonly isProxy: UnwrapRef<typeof import('vue')['isProxy']>
    readonly isReactive: UnwrapRef<typeof import('vue')['isReactive']>
    readonly isReadonly: UnwrapRef<typeof import('vue')['isReadonly']>
    readonly isRef: UnwrapRef<typeof import('vue')['isRef']>
    readonly listenForDeeplink: UnwrapRef<typeof import('./composables/deeplink')['listenForDeeplink']>
    readonly mapActions: UnwrapRef<typeof import('pinia')['mapActions']>
    readonly mapGetters: UnwrapRef<typeof import('pinia')['mapGetters']>
    readonly mapState: UnwrapRef<typeof import('pinia')['mapState']>
    readonly mapStores: UnwrapRef<typeof import('pinia')['mapStores']>
    readonly mapWritableState: UnwrapRef<typeof import('pinia')['mapWritableState']>
    readonly markRaw: UnwrapRef<typeof import('vue')['markRaw']>
    readonly nextTick: UnwrapRef<typeof import('vue')['nextTick']>
    readonly onActivated: UnwrapRef<typeof import('vue')['onActivated']>
    readonly onBeforeMount: UnwrapRef<typeof import('vue')['onBeforeMount']>
    readonly onBeforeRouteLeave: UnwrapRef<typeof import('vue-router/auto')['onBeforeRouteLeave']>
    readonly onBeforeRouteUpdate: UnwrapRef<typeof import('vue-router/auto')['onBeforeRouteUpdate']>
    readonly onBeforeUnmount: UnwrapRef<typeof import('vue')['onBeforeUnmount']>
    readonly onBeforeUpdate: UnwrapRef<typeof import('vue')['onBeforeUpdate']>
    readonly onDeactivated: UnwrapRef<typeof import('vue')['onDeactivated']>
    readonly onErrorCaptured: UnwrapRef<typeof import('vue')['onErrorCaptured']>
    readonly onMounted: UnwrapRef<typeof import('vue')['onMounted']>
    readonly onRenderTracked: UnwrapRef<typeof import('vue')['onRenderTracked']>
    readonly onRenderTriggered: UnwrapRef<typeof import('vue')['onRenderTriggered']>
    readonly onScopeDispose: UnwrapRef<typeof import('vue')['onScopeDispose']>
    readonly onServerPrefetch: UnwrapRef<typeof import('vue')['onServerPrefetch']>
    readonly onUnmounted: UnwrapRef<typeof import('vue')['onUnmounted']>
    readonly onUpdated: UnwrapRef<typeof import('vue')['onUpdated']>
    readonly openExternal: UnwrapRef<typeof import('./composables/util')['openExternal']>
    readonly provide: UnwrapRef<typeof import('vue')['provide']>
    readonly punchMode: UnwrapRef<typeof import('./composables/config')['punchMode']>
    readonly reactive: UnwrapRef<typeof import('vue')['reactive']>
    readonly readConfig: UnwrapRef<typeof import('./composables/path')['readConfig']>
    readonly readonly: UnwrapRef<typeof import('vue')['readonly']>
    readonly ref: UnwrapRef<typeof import('vue')['ref']>
    readonly resolveComponent: UnwrapRef<typeof import('vue')['resolveComponent']>
    readonly setActivePinia: UnwrapRef<typeof import('pinia')['setActivePinia']>
    readonly setMapStoreSuffix: UnwrapRef<typeof import('pinia')['setMapStoreSuffix']>
    readonly shallowReactive: UnwrapRef<typeof import('vue')['shallowReactive']>
    readonly shallowReadonly: UnwrapRef<typeof import('vue')['shallowReadonly']>
    readonly shallowRef: UnwrapRef<typeof import('vue')['shallowRef']>
    readonly shareForDeeplink: UnwrapRef<typeof import('./composables/deeplink')['shareForDeeplink']>
    readonly storeToRefs: UnwrapRef<typeof import('pinia')['storeToRefs']>
    readonly toRaw: UnwrapRef<typeof import('vue')['toRaw']>
    readonly toRef: UnwrapRef<typeof import('vue')['toRef']>
    readonly toRefs: UnwrapRef<typeof import('vue')['toRefs']>
    readonly toValue: UnwrapRef<typeof import('vue')['toValue']>
    readonly triggerRef: UnwrapRef<typeof import('vue')['triggerRef']>
    readonly unref: UnwrapRef<typeof import('vue')['unref']>
    readonly useAppStore: UnwrapRef<typeof import('./stores/app')['useAppStore']>
    readonly useAttrs: UnwrapRef<typeof import('vue')['useAttrs']>
    readonly useConfigStore: UnwrapRef<typeof import('./stores/config')['useConfigStore']>
    readonly useCssModule: UnwrapRef<typeof import('vue')['useCssModule']>
    readonly useCssVars: UnwrapRef<typeof import('vue')['useCssVars']>
    readonly useLink: UnwrapRef<typeof import('vue-router/auto')['useLink']>
    readonly useRoute: UnwrapRef<typeof import('vue-router/auto')['useRoute']>
    readonly useRouter: UnwrapRef<typeof import('vue-router/auto')['useRouter']>
    readonly useSlots: UnwrapRef<typeof import('vue')['useSlots']>
    readonly watch: UnwrapRef<typeof import('vue')['watch']>
    readonly watchEffect: UnwrapRef<typeof import('vue')['watchEffect']>
    readonly watchPostEffect: UnwrapRef<typeof import('vue')['watchPostEffect']>
    readonly watchSyncEffect: UnwrapRef<typeof import('vue')['watchSyncEffect']>
    readonly withEventConnect: UnwrapRef<typeof import('./composables/with')['withEventConnect']>
    readonly withStart: UnwrapRef<typeof import('./composables/with')['withStart']>
    readonly withStop: UnwrapRef<typeof import('./composables/with')['withStop']>
    readonly writeConfig: UnwrapRef<typeof import('./composables/path')['writeConfig']>
  }
}
