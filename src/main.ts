import { setupLayouts } from "virtual:generated-layouts";
import { createRouter, createWebHistory } from "vue-router/auto";
import App from "./App.vue";

import "tdesign-vue-next/es/style/index.css";
import "@unocss/reset/tailwind.css";
import "uno.css";
import "~/styles/main.css";

const app = createApp(App);

const router = createRouter({
  history: createWebHistory(),
  extendRoutes: (routes) => setupLayouts(routes),
});

app.use(router);
app.use(createPinia());
app.mount("#app");
