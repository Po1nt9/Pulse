/// <reference types="vitest" />
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["../tauri/**"],
    },
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
  },
  test: {
    // Pure-logic unit tests; no DOM needed. Keep runs deterministic & isolated.
    environment: 'node',
    include: ['src/**/*.test.ts'],
  },
})
