// vite.config.ts
import path from "node:path";
import { defineConfig } from "file:///D:/coder/with/with-app/node_modules/.pnpm/vite@5.1.5_@types+node@20.11.25/node_modules/vite/dist/node/index.js";
import Vue from "file:///D:/coder/with/with-app/node_modules/.pnpm/@vitejs+plugin-vue@5.0.4_vite@5.1.5_vue@3.4.21/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import Pages from "file:///D:/coder/with/with-app/node_modules/.pnpm/vite-plugin-pages@0.32.0_@vue+compiler-sfc@3.4.21_vite@5.1.5/node_modules/vite-plugin-pages/dist/index.js";
import AutoImport from "file:///D:/coder/with/with-app/node_modules/.pnpm/unplugin-auto-import@0.17.5_@vueuse+core@10.9.0/node_modules/unplugin-auto-import/dist/vite.js";
import Components from "file:///D:/coder/with/with-app/node_modules/.pnpm/unplugin-vue-components@0.26.0_vue@3.4.21/node_modules/unplugin-vue-components/dist/vite.js";
import Layouts from "file:///D:/coder/with/with-app/node_modules/.pnpm/vite-plugin-vue-layouts@0.11.0_vite@5.1.5_vue-router@4.3.0_vue@3.4.21/node_modules/vite-plugin-vue-layouts/dist/index.mjs";
import VueMacros from "file:///D:/coder/with/with-app/node_modules/.pnpm/unplugin-vue-macros@2.7.10_@vueuse+core@10.9.0_typescript@5.4.2_vite@5.1.5_vue@3.4.21/node_modules/unplugin-vue-macros/dist/vite.mjs";
import UnoCSS from "file:///D:/coder/with/with-app/node_modules/.pnpm/unocss@0.58.5_postcss@8.4.35_vite@5.1.5/node_modules/unocss/dist/vite.mjs";
import VueDevTools from "file:///D:/coder/with/with-app/node_modules/.pnpm/vite-plugin-vue-devtools@7.0.16_vite@5.1.5_vue@3.4.21/node_modules/vite-plugin-vue-devtools/dist/vite.mjs";
import VueRouter from "file:///D:/coder/with/with-app/node_modules/.pnpm/unplugin-vue-router@0.8.4_vue-router@4.3.0_vue@3.4.21/node_modules/unplugin-vue-router/dist/vite.mjs";
import { VueRouterAutoImports } from "file:///D:/coder/with/with-app/node_modules/.pnpm/unplugin-vue-router@0.8.4_vue-router@4.3.0_vue@3.4.21/node_modules/unplugin-vue-router/dist/index.mjs";
import { TDesignResolver } from "file:///D:/coder/with/with-app/node_modules/.pnpm/unplugin-vue-components@0.26.0_vue@3.4.21/node_modules/unplugin-vue-components/dist/resolvers.js";
var __vite_injected_original_dirname = "D:\\coder\\with\\with-app";
var vite_config_default = defineConfig(async () => ({
  resolve: {
    alias: {
      "~/": `${path.resolve(__vite_injected_original_dirname, "src")}/`
    }
  },
  plugins: [
    VueMacros({
      plugins: {
        vue: Vue({
          include: [/\.vue$/, /\.md$/]
        })
      }
    }),
    // https://github.com/posva/unplugin-vue-router
    VueRouter({
      routesFolder: "src/pages",
      extensions: [".vue", ".md"],
      dts: "src/typed-router.d.ts"
    }),
    Components({
      resolvers: [
        TDesignResolver({
          library: "vue-next"
        })
      ],
      // allow auto load markdown components under `./src/components/`
      extensions: ["vue", "md"],
      // allow auto import and register components used in markdown
      include: [/\.vue$/, /\.vue\?vue/],
      dts: "src/components.d.ts"
    }),
    AutoImport({
      // targets to transform
      include: [/\.[tj]sx?$/, /\.vue$/, /\.vue\?vue/],
      imports: [
        "vue",
        "pinia",
        VueRouterAutoImports,
        {
          // add any other imports you were relying on
          "vue-router/auto": ["useLink"]
        }
      ],
      dts: "src/auto-imports.d.ts",
      dirs: ["src/components", "src/composables", "src/contants", "src/stores"],
      vueTemplate: true,
      resolvers: [
        TDesignResolver({
          library: "vue-next"
        })
      ]
    }),
    UnoCSS(),
    Pages({
      extensions: ["vue", "md"]
    }),
    // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
    Layouts(),
    VueDevTools()
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"]
    }
  }
}));
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxjb2RlclxcXFx3aXRoXFxcXHdpdGgtYXBwXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCJEOlxcXFxjb2RlclxcXFx3aXRoXFxcXHdpdGgtYXBwXFxcXHZpdGUuY29uZmlnLnRzXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ltcG9ydF9tZXRhX3VybCA9IFwiZmlsZTovLy9EOi9jb2Rlci93aXRoL3dpdGgtYXBwL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHBhdGggZnJvbSBcIm5vZGU6cGF0aFwiO1xyXG5pbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tIFwidml0ZVwiO1xyXG5pbXBvcnQgVnVlIGZyb20gXCJAdml0ZWpzL3BsdWdpbi12dWVcIjtcclxuaW1wb3J0IFBhZ2VzIGZyb20gXCJ2aXRlLXBsdWdpbi1wYWdlc1wiO1xyXG5pbXBvcnQgQXV0b0ltcG9ydCBmcm9tIFwidW5wbHVnaW4tYXV0by1pbXBvcnQvdml0ZVwiO1xyXG5pbXBvcnQgQ29tcG9uZW50cyBmcm9tIFwidW5wbHVnaW4tdnVlLWNvbXBvbmVudHMvdml0ZVwiO1xyXG5pbXBvcnQgTGF5b3V0cyBmcm9tIFwidml0ZS1wbHVnaW4tdnVlLWxheW91dHNcIjtcclxuaW1wb3J0IFZ1ZU1hY3JvcyBmcm9tIFwidW5wbHVnaW4tdnVlLW1hY3Jvcy92aXRlXCI7XHJcbmltcG9ydCBVbm9DU1MgZnJvbSBcInVub2Nzcy92aXRlXCI7XHJcbmltcG9ydCBWdWVEZXZUb29scyBmcm9tIFwidml0ZS1wbHVnaW4tdnVlLWRldnRvb2xzXCI7XHJcbmltcG9ydCBWdWVSb3V0ZXIgZnJvbSBcInVucGx1Z2luLXZ1ZS1yb3V0ZXIvdml0ZVwiO1xyXG5pbXBvcnQgeyBWdWVSb3V0ZXJBdXRvSW1wb3J0cyB9IGZyb20gXCJ1bnBsdWdpbi12dWUtcm91dGVyXCI7XHJcbmltcG9ydCB7IFREZXNpZ25SZXNvbHZlciB9IGZyb20gXCJ1bnBsdWdpbi12dWUtY29tcG9uZW50cy9yZXNvbHZlcnNcIjtcclxuXHJcbi8vIGh0dHBzOi8vdml0ZWpzLmRldi9jb25maWcvXHJcbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyhhc3luYyAoKSA9PiAoe1xyXG4gIHJlc29sdmU6IHtcclxuICAgIGFsaWFzOiB7XHJcbiAgICAgIFwifi9cIjogYCR7cGF0aC5yZXNvbHZlKF9fZGlybmFtZSwgXCJzcmNcIil9L2AsXHJcbiAgICB9LFxyXG4gIH0sXHJcbiAgcGx1Z2luczogW1xyXG4gICAgVnVlTWFjcm9zKHtcclxuICAgICAgcGx1Z2luczoge1xyXG4gICAgICAgIHZ1ZTogVnVlKHtcclxuICAgICAgICAgIGluY2x1ZGU6IFsvXFwudnVlJC8sIC9cXC5tZCQvXSxcclxuICAgICAgICB9KSxcclxuICAgICAgfSxcclxuICAgIH0pLFxyXG5cclxuICAgIC8vIGh0dHBzOi8vZ2l0aHViLmNvbS9wb3N2YS91bnBsdWdpbi12dWUtcm91dGVyXHJcbiAgICBWdWVSb3V0ZXIoe1xyXG4gICAgICByb3V0ZXNGb2xkZXI6IFwic3JjL3BhZ2VzXCIsXHJcbiAgICAgIGV4dGVuc2lvbnM6IFtcIi52dWVcIiwgXCIubWRcIl0sXHJcbiAgICAgIGR0czogXCJzcmMvdHlwZWQtcm91dGVyLmQudHNcIixcclxuICAgIH0pLFxyXG5cclxuICAgIENvbXBvbmVudHMoe1xyXG4gICAgICByZXNvbHZlcnM6IFtcclxuICAgICAgICBURGVzaWduUmVzb2x2ZXIoe1xyXG4gICAgICAgICAgbGlicmFyeTogXCJ2dWUtbmV4dFwiLFxyXG4gICAgICAgIH0pLFxyXG4gICAgICBdLFxyXG4gICAgICAvLyBhbGxvdyBhdXRvIGxvYWQgbWFya2Rvd24gY29tcG9uZW50cyB1bmRlciBgLi9zcmMvY29tcG9uZW50cy9gXHJcbiAgICAgIGV4dGVuc2lvbnM6IFtcInZ1ZVwiLCBcIm1kXCJdLFxyXG4gICAgICAvLyBhbGxvdyBhdXRvIGltcG9ydCBhbmQgcmVnaXN0ZXIgY29tcG9uZW50cyB1c2VkIGluIG1hcmtkb3duXHJcbiAgICAgIGluY2x1ZGU6IFsvXFwudnVlJC8sIC9cXC52dWVcXD92dWUvXSxcclxuICAgICAgZHRzOiBcInNyYy9jb21wb25lbnRzLmQudHNcIixcclxuICAgIH0pLFxyXG5cclxuICAgIEF1dG9JbXBvcnQoe1xyXG4gICAgICAvLyB0YXJnZXRzIHRvIHRyYW5zZm9ybVxyXG4gICAgICBpbmNsdWRlOiBbL1xcLlt0al1zeD8kLywgL1xcLnZ1ZSQvLCAvXFwudnVlXFw/dnVlL10sXHJcbiAgICAgIGltcG9ydHM6IFtcclxuICAgICAgICBcInZ1ZVwiLFxyXG4gICAgICAgIFwicGluaWFcIixcclxuICAgICAgICBWdWVSb3V0ZXJBdXRvSW1wb3J0cyxcclxuICAgICAgICB7XHJcbiAgICAgICAgICAvLyBhZGQgYW55IG90aGVyIGltcG9ydHMgeW91IHdlcmUgcmVseWluZyBvblxyXG4gICAgICAgICAgXCJ2dWUtcm91dGVyL2F1dG9cIjogW1widXNlTGlua1wiXSxcclxuICAgICAgICB9LFxyXG4gICAgICBdLFxyXG4gICAgICBkdHM6IFwic3JjL2F1dG8taW1wb3J0cy5kLnRzXCIsXHJcbiAgICAgIGRpcnM6IFtcInNyYy9jb21wb25lbnRzXCIsIFwic3JjL2NvbXBvc2FibGVzXCIsIFwic3JjL2NvbnRhbnRzXCIsIFwic3JjL3N0b3Jlc1wiXSxcclxuICAgICAgdnVlVGVtcGxhdGU6IHRydWUsXHJcbiAgICAgIHJlc29sdmVyczogW1xyXG4gICAgICAgIFREZXNpZ25SZXNvbHZlcih7XHJcbiAgICAgICAgICBsaWJyYXJ5OiBcInZ1ZS1uZXh0XCIsXHJcbiAgICAgICAgfSksXHJcbiAgICAgIF0sXHJcbiAgICB9KSxcclxuXHJcbiAgICBVbm9DU1MoKSxcclxuXHJcbiAgICBQYWdlcyh7XHJcbiAgICAgIGV4dGVuc2lvbnM6IFtcInZ1ZVwiLCBcIm1kXCJdLFxyXG4gICAgfSksXHJcblxyXG4gICAgLy8gaHR0cHM6Ly9naXRodWIuY29tL0pvaG5DYW1waW9uSnIvdml0ZS1wbHVnaW4tdnVlLWxheW91dHNcclxuICAgIExheW91dHMoKSxcclxuXHJcbiAgICBWdWVEZXZUb29scygpLFxyXG4gIF0sXHJcblxyXG4gIC8vIFZpdGUgb3B0aW9ucyB0YWlsb3JlZCBmb3IgVGF1cmkgZGV2ZWxvcG1lbnQgYW5kIG9ubHkgYXBwbGllZCBpbiBgdGF1cmkgZGV2YCBvciBgdGF1cmkgYnVpbGRgXHJcbiAgLy9cclxuICAvLyAxLiBwcmV2ZW50IHZpdGUgZnJvbSBvYnNjdXJpbmcgcnVzdCBlcnJvcnNcclxuICBjbGVhclNjcmVlbjogZmFsc2UsXHJcbiAgLy8gMi4gdGF1cmkgZXhwZWN0cyBhIGZpeGVkIHBvcnQsIGZhaWwgaWYgdGhhdCBwb3J0IGlzIG5vdCBhdmFpbGFibGVcclxuICBzZXJ2ZXI6IHtcclxuICAgIHBvcnQ6IDE0MjAsXHJcbiAgICBzdHJpY3RQb3J0OiB0cnVlLFxyXG4gICAgd2F0Y2g6IHtcclxuICAgICAgLy8gMy4gdGVsbCB2aXRlIHRvIGlnbm9yZSB3YXRjaGluZyBgc3JjLXRhdXJpYFxyXG4gICAgICBpZ25vcmVkOiBbXCIqKi9zcmMtdGF1cmkvKipcIl0sXHJcbiAgICB9LFxyXG4gIH0sXHJcbn0pKTtcclxuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUE0UCxPQUFPLFVBQVU7QUFDN1EsU0FBUyxvQkFBb0I7QUFDN0IsT0FBTyxTQUFTO0FBQ2hCLE9BQU8sV0FBVztBQUNsQixPQUFPLGdCQUFnQjtBQUN2QixPQUFPLGdCQUFnQjtBQUN2QixPQUFPLGFBQWE7QUFDcEIsT0FBTyxlQUFlO0FBQ3RCLE9BQU8sWUFBWTtBQUNuQixPQUFPLGlCQUFpQjtBQUN4QixPQUFPLGVBQWU7QUFDdEIsU0FBUyw0QkFBNEI7QUFDckMsU0FBUyx1QkFBdUI7QUFaaEMsSUFBTSxtQ0FBbUM7QUFlekMsSUFBTyxzQkFBUSxhQUFhLGFBQWE7QUFBQSxFQUN2QyxTQUFTO0FBQUEsSUFDUCxPQUFPO0FBQUEsTUFDTCxNQUFNLEdBQUcsS0FBSyxRQUFRLGtDQUFXLEtBQUssQ0FBQztBQUFBLElBQ3pDO0FBQUEsRUFDRjtBQUFBLEVBQ0EsU0FBUztBQUFBLElBQ1AsVUFBVTtBQUFBLE1BQ1IsU0FBUztBQUFBLFFBQ1AsS0FBSyxJQUFJO0FBQUEsVUFDUCxTQUFTLENBQUMsVUFBVSxPQUFPO0FBQUEsUUFDN0IsQ0FBQztBQUFBLE1BQ0g7QUFBQSxJQUNGLENBQUM7QUFBQTtBQUFBLElBR0QsVUFBVTtBQUFBLE1BQ1IsY0FBYztBQUFBLE1BQ2QsWUFBWSxDQUFDLFFBQVEsS0FBSztBQUFBLE1BQzFCLEtBQUs7QUFBQSxJQUNQLENBQUM7QUFBQSxJQUVELFdBQVc7QUFBQSxNQUNULFdBQVc7QUFBQSxRQUNULGdCQUFnQjtBQUFBLFVBQ2QsU0FBUztBQUFBLFFBQ1gsQ0FBQztBQUFBLE1BQ0g7QUFBQTtBQUFBLE1BRUEsWUFBWSxDQUFDLE9BQU8sSUFBSTtBQUFBO0FBQUEsTUFFeEIsU0FBUyxDQUFDLFVBQVUsWUFBWTtBQUFBLE1BQ2hDLEtBQUs7QUFBQSxJQUNQLENBQUM7QUFBQSxJQUVELFdBQVc7QUFBQTtBQUFBLE1BRVQsU0FBUyxDQUFDLGNBQWMsVUFBVSxZQUFZO0FBQUEsTUFDOUMsU0FBUztBQUFBLFFBQ1A7QUFBQSxRQUNBO0FBQUEsUUFDQTtBQUFBLFFBQ0E7QUFBQTtBQUFBLFVBRUUsbUJBQW1CLENBQUMsU0FBUztBQUFBLFFBQy9CO0FBQUEsTUFDRjtBQUFBLE1BQ0EsS0FBSztBQUFBLE1BQ0wsTUFBTSxDQUFDLGtCQUFrQixtQkFBbUIsZ0JBQWdCLFlBQVk7QUFBQSxNQUN4RSxhQUFhO0FBQUEsTUFDYixXQUFXO0FBQUEsUUFDVCxnQkFBZ0I7QUFBQSxVQUNkLFNBQVM7QUFBQSxRQUNYLENBQUM7QUFBQSxNQUNIO0FBQUEsSUFDRixDQUFDO0FBQUEsSUFFRCxPQUFPO0FBQUEsSUFFUCxNQUFNO0FBQUEsTUFDSixZQUFZLENBQUMsT0FBTyxJQUFJO0FBQUEsSUFDMUIsQ0FBQztBQUFBO0FBQUEsSUFHRCxRQUFRO0FBQUEsSUFFUixZQUFZO0FBQUEsRUFDZDtBQUFBO0FBQUE7QUFBQTtBQUFBLEVBS0EsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixNQUFNO0FBQUEsSUFDTixZQUFZO0FBQUEsSUFDWixPQUFPO0FBQUE7QUFBQSxNQUVMLFNBQVMsQ0FBQyxpQkFBaUI7QUFBQSxJQUM3QjtBQUFBLEVBQ0Y7QUFDRixFQUFFOyIsCiAgIm5hbWVzIjogW10KfQo=
