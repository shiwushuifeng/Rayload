import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite'
import { resolve } from 'path'

const host = process.env.TAURI_DEV_HOST

const demoBuild = process.env.VITE_RAYLOAD_DEMO === '1'

export default defineConfig(async () => ({
  plugins: [
    vue(),
    VueI18nPlugin({
      include: resolve(__dirname, 'src/shared/locales/*/messages.json'),
      runtimeOnly: true,
    }),
  ],
  resolve: {
    alias: [
      { find: '@tauri-core-real', replacement: resolve(__dirname, 'node_modules/@tauri-apps/api/core.js') },
      ...(demoBuild
        ? [
            { find: /^@tauri-apps\/api\/core$/, replacement: resolve(__dirname, 'src/demo/core.ts') },
            { find: /^@tauri-apps\/api\/event$/, replacement: resolve(__dirname, 'src/demo/event.ts') },
          ]
        : []),
      { find: '@shared', replacement: resolve(__dirname, 'src/shared') },
      { find: '@', replacement: resolve(__dirname, 'src') },
      { find: 'path', replacement: 'path-browserify' },
    ],
  },
  clearScreen: false,
  build: {
    minify: 'esbuild',
    cssMinify: true,
    reportCompressedSize: false,
    sourcemap: false,
    modulePreload: { polyfill: false },
    esbuild: {
      drop: ['console', 'debugger'],
    },
    rollupOptions: {
      treeshake: 'smallest',
      input: {
        main: resolve(__dirname, 'index.html'),
      },
      output: {
        manualChunks: {
          'naive-ui': ['naive-ui'],
          'tauri-api': [
            '@tauri-apps/api',
            '@tauri-apps/plugin-shell',
            '@tauri-apps/plugin-dialog',
            '@tauri-apps/plugin-fs',
            '@tauri-apps/plugin-clipboard-manager',
            '@tauri-apps/plugin-updater',
          ],
          'vue-vendor': ['vue', 'vue-router', 'pinia', 'vue-i18n'],
        },
      },
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}))
