import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from "path";

// Tauri 期望固定端口，dev 时失败则报错而非随机切换
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? { protocol: "ws", host, port: 1421 }
      : undefined,
    watch: {
      // tauri 源码目录不触发前端热更新
      ignored: ["**/src-tauri/**"],
    },
  },
  // 多页面：主界面 + 桌面悬浮层 两个独立入口
  build: {
    target: "es2021",
    minify: "esbuild",
    sourcemap: false,
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        desktop: resolve(__dirname, "desktop.html"),
      },
    },
  },
});
