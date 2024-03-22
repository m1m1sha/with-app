import { setupLayouts } from "virtual:generated-layouts";
import { createRouter, createWebHistory } from "vue-router/auto";
import { RouteRecordRaw } from "vue-router";

import App from "./App.vue";

import "@unocss/reset/tailwind.css";
import "uno.css";
import "~/styles/main.css";

const app = createApp(App);

const router = createRouter({
  history: createWebHistory(),
  extendRoutes: (routes: RouteRecordRaw[]) => setupLayouts(routes),
});

app.use(router);
app.use(createPinia());
app.mount("#app");
