/// <reference types="vite/client" />
/* eslint-disable */
/* prettier-ignore */
declare module '*.vue' {
  import type { DefineComponent } from 'vue'

  const component: DefineComponent<{}, {}, any>
  export default component
}
