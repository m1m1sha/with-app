import { setupLayouts } from 'virtual:generated-layouts'
import type { RouteRecordRaw } from 'vue-router'
import { createRouter, createWebHistory } from 'vue-router/auto'
import App from './App.vue'

import 'tdesign-vue-next/es/style/index.css'
import '@unocss/reset/tailwind.css'
import 'uno.css'
import '~/styles/main.css'

if (import.meta.env.PROD) {
  document.addEventListener('keydown', (event) => {
    if (
      event.key === 'F5'
      || (event.ctrlKey && event.key === 'r')
      || (event.metaKey && event.key === 'r')
    )
      event.preventDefault()
  })

  document.addEventListener('contextmenu', (event) => {
    event.preventDefault()
  })
}

const app = createApp(App)

const router = createRouter({
  history: createWebHistory(),
  extendRoutes: (routes: RouteRecordRaw[]) => setupLayouts(routes),
})

app.use(router)
app.use(createPinia())
app.mount('#app')
