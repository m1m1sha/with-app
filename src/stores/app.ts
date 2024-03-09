import { defineStore } from 'pinia'

export const useAppStore = defineStore('app', () => {
  const menu = ref('/')
  const settingTab = ref('basic')

  return {
    menu,
    settingTab,
  }
})
