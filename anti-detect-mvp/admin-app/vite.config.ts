import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  
  // Tauri expects a relative base path
  base: './',
  
  // Build config
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  },
  
  // Clear the port for dev
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
})
