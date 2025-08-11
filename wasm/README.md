# json-packer-wasm

WebAssembly 绑定，基于 `json-packer` Rust 库，使用 `wasm-bindgen`。

## 安装

```bash
npm install json-packer-wasm
# 或
pnpm add json-packer-wasm
```

## API

```typescript
// 选项类别
export class Options {
  constructor(
    enable_value_pool: boolean,
    pool_min_repeats: number,
    pool_min_string_len: number
  );
}

// 压缩为字节数组
export function compress_to_bytes(jsonString: string, options: Options): Uint8Array;

// 压缩为 Base64 字符串
export function compress_to_base64(jsonString: string, options: Options): string;

// 从字节数组解压
export function decompress_from_bytes(bytes: Uint8Array): string;

// 从 Base64 字符串解压
export function decompress_from_base64(base64: string): string;
```

## 快速开始

### 浏览器（ES Modules）

```javascript
import init, { Options, compress_to_base64, decompress_from_base64 } from 'json-packer-wasm';

async function example() {
  // 初始化 WASM 模块
  await init();

  const data = { name: "Alice", age: 30, active: true };
  const jsonStr = JSON.stringify(data);

  // 创建压缩选项
  const options = new Options(false, 3, 8); // 不启用值池

  // 压缩
  const compressed = compress_to_base64(jsonStr, options);
  console.log('Compressed:', compressed);

  // 解压
  const decompressed = decompress_from_base64(compressed);
  const restored = JSON.parse(decompressed);
  console.log('Restored:', restored);
}

example();
```

### 启用字符串值池

```javascript
import init, { Options, compress_to_bytes, decompress_from_bytes } from 'json-packer-wasm';

await init();

const data = {
  logs: [
    { level: "info", message: "server started successfully" },
    { level: "info", message: "server started successfully" },
    { level: "error", message: "connection timeout occurred" },
    { level: "error", message: "connection timeout occurred" }
  ]
};

// 启用值池：重复 2 次以上，长度 >= 6 的字符串进入值池
const options = new Options(true, 2, 6);

const compressed = compress_to_bytes(JSON.stringify(data), options);
const restored = JSON.parse(decompress_from_bytes(compressed));

console.log('Original size:', new TextEncoder().encode(JSON.stringify(data)).length);
console.log('Compressed size:', compressed.length);
```

### Node.js（CommonJS）

```javascript
const init = require('json-packer-wasm');
const { Options, compress_to_base64, decompress_from_base64 } = init;

async function nodeExample() {
  await init();
  
  const options = new Options(false, 3, 8);
  const data = { items: ["apple", "banana", "cherry"] };
  
  const compressed = compress_to_base64(JSON.stringify(data), options);
  const restored = JSON.parse(decompress_from_base64(compressed));
  
  console.log('Success:', JSON.stringify(data) === JSON.stringify(restored));
}

nodeExample();
```

## 构建

从源码构建（需要 Rust 工具链和 `wasm-pack`）：

```bash
# 安装 wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 克隆并构建
git clone <repository>
cd json-packer/wasm
wasm-pack build --target web

# 生成的包在 pkg/ 目录
ls pkg/  # 包含 .wasm、.js、.d.ts、package.json
```

## 打包器集成

### Webpack

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true,
  },
  // ...
};
```

### Vite

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ['json-packer-wasm']
  }
};
```

### Rollup

需要 `@rollup/plugin-wasm` 插件处理 WASM 文件。

## 平台支持

- 所有支持 WebAssembly 的现代浏览器
- Node.js (v14+)
- Deno
- 其他支持 WASM 的 JavaScript 运行时

## 注意事项

- 必须先调用 `init()` 初始化 WASM 模块
- 输入必须是有效的 JSON 字符串
- 不支持 NaN、Infinity 等非标准 JSON 值
- 压缩结果是确定性的（相同输入+配置 → 相同输出）
- WASM 文件大小约 100KB，首次加载需要网络传输

## 许可证

MIT
