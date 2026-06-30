import { defineConfig } from 'vite';
//import { svelte } from '@sveltejs/vite-plugin-svelte';
import { sveltekit } from "@sveltejs/kit/vite";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  // plugins: [svelte()],
  plugins: [sveltekit()],
  clearScreen: false,
  server: {
    host: host || false,
    port: 1420,
    strictPort: true,
    hmr: host
      ? {
          protocol: 'ws',
          host: host,
          port: 1430,
        }
      : undefined,
  },
});