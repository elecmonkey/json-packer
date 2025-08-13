#!/usr/bin/env node

const { build } = require('esbuild');
const fs = require('fs');
const path = require('path');

async function compressWasmBindings() {
  const pkgDir = path.join(__dirname, 'pkg');
  const jsFile = path.join(pkgDir, 'json_packer_wasm.js');
  const compressedFile = path.join(pkgDir, 'json_packer_wasm.min.js');
  
  if (!fs.existsSync(jsFile)) {
    console.error('❌ WASM JS文件不存在，请先运行 pnpm build:wasm');
    process.exit(1);
  }

  console.log('🔧 开始压缩WASM胶水代码...');
  
  try {
    // 读取原始文件大小
    const originalSize = fs.statSync(jsFile).size;
    
    // 使用esbuild压缩
    await build({
      entryPoints: [jsFile],
      outfile: compressedFile,
      bundle: false,
      minify: true,
      target: 'es2020',
      format: 'esm',
      write: true,
      legalComments: 'none'
    });
    
    // 读取压缩后文件大小
    const compressedSize = fs.statSync(compressedFile).size;
    const reduction = ((originalSize - compressedSize) / originalSize * 100).toFixed(1);
    
    console.log(`📊 压缩结果:`);
    console.log(`   原始大小: ${(originalSize / 1024).toFixed(1)}KB`);
    console.log(`   压缩后: ${(compressedSize / 1024).toFixed(1)}KB`);
    console.log(`   减少: ${reduction}%`);
    
    // 替换原文件
    fs.copyFileSync(compressedFile, jsFile);
    fs.unlinkSync(compressedFile);
    
    console.log('✅ 压缩完成，已替换原文件');
    
  } catch (error) {
    console.error('❌ 压缩失败:', error.message);
    process.exit(1);
  }
}

if (require.main === module) {
  compressWasmBindings();
}

module.exports = { compressWasmBindings };