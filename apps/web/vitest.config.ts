import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vitest/config';
import react from '@vitejs/plugin-react';

// Minimal Vitest configuration. The project's build config lives in
// vite.config.ts; this separate config keeps the test runner isolated from
// the production build while reusing the React plugin for any future
// component tests. Pure utility tests run in the `node` environment.
export default defineConfig({
  plugins: [react()],
  test: {
    // Pin root to this config file's directory so test discovery is
    // independent of the cwd `npm test` is invoked from.
    root: fileURLToPath(new URL('./', import.meta.url)),
    environment: 'node',
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
    // Deterministic, isolated runs: no watch mode, single fork, no caching
    // of cross-run state.
    pool: 'forks',
    poolOptions: { forks: { singleFork: true } },
    reporters: 'default',
  },
});
