import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
import { fileURLToPath, URL } from "node:url";

export default defineConfig({
  plugins: [vue(), tailwindcss()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./resources/js", import.meta.url)),
    },
  },
  build: {
    manifest: true,
    outDir: "public/build",
    emptyOutDir: true,
    rollupOptions: {
      input: "resources/js/app.ts",
    },
  },
  server: {
    host: "127.0.0.1",
    port: 5173,
    strictPort: true,
  },
});
