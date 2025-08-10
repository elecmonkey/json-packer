import { defineConfig } from 'vite'

// 确保 .wasm 文件以正确的 MIME 类型服务
export default defineConfig({
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
    fs: {
      allow: ['..'],
    },
  },
  // 配置 WASM 文件作为静态资源
  assetsInclude: ['**/*.wasm'],
  // 针对 .wasm 文件设置 MIME 类型
  optimizeDeps: {
    exclude: ['json_packer_wasm'],
  },
})