#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const { compressWasmBindings } = require('./compress');

function execCommand(command, description) {
  console.log(`🔧 ${description}...`);
  try {
    execSync(command, { 
      stdio: 'inherit',
      env: {
        ...process.env,
        PATH: `${process.env.HOME}/.cargo/bin:${process.env.PATH}`
      }
    });
    console.log(`✅ ${description}完成`);
  } catch (error) {
    console.error(`❌ ${description}失败:`, error.message);
    process.exit(1);
  }
}

function checkPrerequisites() {
  console.log('🔍 检查构建环境...');
  
  // 检查rustup
  try {
    execSync('rustup --version', { stdio: 'pipe' });
  } catch {
    console.error('❌ rustup未安装，请先安装Rust工具链');
    process.exit(1);
  }
  
  // 检查wasm-pack
  try {
    execSync('wasm-pack --version', { stdio: 'pipe' });
  } catch {
    console.error('❌ wasm-pack未安装，请运行: cargo install wasm-pack');
    process.exit(1);
  }
  
  // 检查wasm32目标
  try {
    const targets = execSync('rustup target list --installed', { encoding: 'utf8' });
    if (!targets.includes('wasm32-unknown-unknown')) {
      console.log('📦 安装wasm32目标...');
      execSync('rustup target add wasm32-unknown-unknown');
    }
  } catch {
    console.error('❌ 无法检查或安装wasm32目标');
    process.exit(1);
  }
  
  console.log('✅ 环境检查通过');
}

function getFileSizes() {
  const pkgDir = path.join(__dirname, 'pkg');
  const wasmFile = path.join(pkgDir, 'json_packer_wasm_bg.wasm');
  const jsFile = path.join(pkgDir, 'json_packer_wasm.js');
  
  const sizes = {};
  
  if (fs.existsSync(wasmFile)) {
    sizes.wasm = fs.statSync(wasmFile).size;
  }
  
  if (fs.existsSync(jsFile)) {
    sizes.js = fs.statSync(jsFile).size;
  }
  
  return sizes;
}

async function main() {
  console.log('🚀 开始构建优化的WASM包...');
  console.log('');
  
  // 检查环境
  checkPrerequisites();
  console.log('');
  
  // 构建WASM
  execCommand(
    'wasm-pack build --target web --release',
    '构建WASM包'
  );
  
  // 获取构建后大小
  const beforeSizes = getFileSizes();
  console.log('');
  console.log('📊 构建后文件大小:');
  if (beforeSizes.wasm) {
    console.log(`   WASM: ${(beforeSizes.wasm / 1024).toFixed(1)}KB`);
  }
  if (beforeSizes.js) {
    console.log(`   JS胶水代码: ${(beforeSizes.js / 1024).toFixed(1)}KB`);
  }
  
  console.log('');
  
  // 压缩JS胶水代码
  await compressWasmBindings();
  
  // 获取压缩后大小
  const afterSizes = getFileSizes();
  console.log('');
  console.log('🎉 最终构建结果:');
  if (afterSizes.wasm) {
    console.log(`   WASM: ${(afterSizes.wasm / 1024).toFixed(1)}KB`);
  }
  if (afterSizes.js && beforeSizes.js) {
    const jsReduction = ((beforeSizes.js - afterSizes.js) / beforeSizes.js * 100).toFixed(1);
    console.log(`   JS胶水代码: ${(afterSizes.js / 1024).toFixed(1)}KB (减少${jsReduction}%)`);
  }
  
  const totalBefore = (beforeSizes.wasm || 0) + (beforeSizes.js || 0);
  const totalAfter = (afterSizes.wasm || 0) + (afterSizes.js || 0);
  const totalReduction = ((totalBefore - totalAfter) / totalBefore * 100).toFixed(1);
  
  console.log(`   总大小: ${(totalAfter / 1024).toFixed(1)}KB (减少${totalReduction}%)`);
  console.log('');
}

if (require.main === module) {
  main().catch(error => {
    console.error('❌ 构建失败:', error.message);
    process.exit(1);
  });
}

module.exports = { main };