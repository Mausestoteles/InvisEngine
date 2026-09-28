import { defineConfig } from "vite";

// Fixed port so tauri.conf.json's devUrl always matches.
export default defineConfig({
  clearScreen: false,
  server: { port: 1420, strictPort: true },
});
