import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  
  // CRITICAL: Tauri needs relative paths, not absolute
  base: '',
  
  // Build config
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    // Ensure relative paths in output
    assetsDir: 'assets',
  },
  
  // Clear the port for dev
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
  },
})
