import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from 'path';
import svgLoader from 'vite-svg-loader';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(), svgLoader()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'), // 这里配置 @ 为 src 目录
    },
  },
  server: {
    port: 0,
    strictPort: false,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
