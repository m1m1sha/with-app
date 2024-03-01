import path from "node:path";
import { defineConfig } from "vite";
import Vue from "@vitejs/plugin-vue";
import Pages from "vite-plugin-pages";
import AutoImport from "unplugin-auto-import/vite";
import Components from "unplugin-vue-components/vite";
import Layouts from "vite-plugin-vue-layouts";
import Markdown from "unplugin-vue-markdown/vite";
import VueMacros from "unplugin-vue-macros/vite";
import UnoCSS from "unocss/vite";
import VueDevTools from "vite-plugin-vue-devtools";
import LinkAttributes from "markdown-it-link-attributes";
import Shiki from "markdown-it-shikiji";
import VueRouter from "unplugin-vue-router/vite";
import { VueRouterAutoImports } from "unplugin-vue-router";
import { TDesignResolver } from "unplugin-vue-components/resolvers";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  resolve: {
    alias: {
      "~/": `${path.resolve(__dirname, "src")}/`,
    },
  },
  plugins: [
    VueMacros({
      plugins: {
        vue: Vue({
          include: [/\.vue$/, /\.md$/],
        }),
      },
    }),

    // https://github.com/posva/unplugin-vue-router
    VueRouter({
      routesFolder: "src/pages",
      extensions: [".vue", ".md"],
      dts: "src/typed-router.d.ts",
    }),

    Components({
      resolvers: [
        TDesignResolver({
          library: "vue-next",
        }),
      ],
      // allow auto load markdown components under `./src/components/`
      extensions: ["vue", "md"],
      // allow auto import and register components used in markdown
      include: [/\.vue$/, /\.vue\?vue/, /\.md$/],
      dts: "src/components.d.ts",
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
          "vue-router/auto": ["useLink"],
        },
      ],
      dts: "src/auto-imports.d.ts",
      dirs: ["src/components", "src/composables", "src/contants", "src/stores"],
      vueTemplate: true,
      resolvers: [
        TDesignResolver({
          library: "vue-next",
        }),
      ],
    }),

    // https://github.com/unplugin/unplugin-vue-markdown
    // Don't need this? Try vitesse-lite: https://github.com/antfu/vitesse-lite
    Markdown({
      wrapperClasses: "prose prose-sm m-auto text-left",
      headEnabled: true,
      async markdownItSetup(md) {
        md.use(LinkAttributes, {
          matcher: (link: string) => /^https?:\/\//.test(link),
          attrs: {
            target: "_blank",
            rel: "noopener",
          },
        });
        md.use(
          await Shiki({
            defaultColor: false,
            themes: {
              light: "vitesse-light",
              dark: "vitesse-dark",
            },
          })
        );
      },
    }),

    UnoCSS(),

    Pages({
      extensions: ["vue", "md"],
    }),

    // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
    Layouts(),

    VueDevTools(),
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
      ignored: ["**/src-tauri/**"],
    },
  },
}));
