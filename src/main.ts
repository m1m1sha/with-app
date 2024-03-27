import { setupLayouts } from "virtual:generated-layouts";
import { createRouter, createWebHistory } from "vue-router/auto";
import { RouteRecordRaw } from "vue-router";

import App from "~/App.vue";

import "@unocss/reset/tailwind.css";
import "uno.css";
import "vfonts/FiraCode.css";
import "~/styles/main.css";

if (isProd()) {
  document.addEventListener("keydown", (event) => {
    if (
      event.key === "F5" ||
      (event.ctrlKey && event.key === "r") ||
      (event.metaKey && event.key === "r")
    )
      event.preventDefault();
  });

  document.addEventListener("contextmenu", (event) => {
    event.preventDefault();
  });
}

const app = createApp(App);

const router = createRouter({
  history: createWebHistory(),
  extendRoutes: (routes: RouteRecordRaw[]) => setupLayouts(routes),
});

app.use(router);
app.use(createPinia());

const meta = document.createElement("meta");
meta.name = "naive-ui-style";
document.head.appendChild(meta);

app.mount("#app");
