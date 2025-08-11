# json-packer-napi

Node.js N-API 绑定，基于 `json_packer_core` Rust 库。

## 安装

```bash
npm install json-packer-napi
# 或
pnpm add json-packer-napi
```

## API

```typescript
interface CompressOptions {
  enable_value_pool?: boolean;     // 是否启用字符串值池（默认 false）
  pool_min_repeats?: number;       // 计入值池的最小重复次数（默认 3）
  pool_min_string_len?: number;    // 计入值池的最小字符串长度（默认 8）
}

// 压缩为字节数组
export function compressToBytes(jsonString: string, options?: CompressOptions): Buffer;

// 压缩为 Base64 字符串
export function compressToBase64(jsonString: string, options?: CompressOptions): string;

// 从字节数组解压
export function decompressFromBytes(bytes: Buffer): string;

// 从 Base64 字符串解压
export function decompressFromBase64(base64: string): string;
```

## 快速开始

### 基础用法

```javascript
const { compressToBase64, decompressFromBase64 } = require('json-packer-napi');

const data = { name: "Alice", age: 30, active: true };
const jsonStr = JSON.stringify(data);

// 压缩
const compressed = compressToBase64(jsonStr);
console.log('Compressed:', compressed);

// 解压
const decompressed = decompressFromBase64(compressed);
const restored = JSON.parse(decompressed);
console.log('Restored:', restored);
```

### ESM 导入

```javascript
import { compressToBase64, decompressFromBase64 } from 'json-packer-napi';

const data = { items: ["apple", "banana", "cherry"] };
const compressed = compressToBase64(JSON.stringify(data));
const restored = JSON.parse(decompressFromBase64(compressed));
```

### 启用字符串值池

```javascript
const { compressToBytes, decompressFromBytes } = require('json-packer-napi');

const data = {
  users: [
    { status: "connected", message: "connected to server" },
    { status: "connected", message: "connected to server" },
    { status: "connected", message: "connected to server" }
  ]
};

const options = {
  enable_value_pool: true,
  pool_min_repeats: 2,      // 重复 2 次以上进入值池
  pool_min_string_len: 5    // 字符串长度 >= 5 才考虑值池
};

const compressed = compressToBytes(JSON.stringify(data), options);
const restored = JSON.parse(decompressFromBytes(compressed));

console.log('Original size:', Buffer.byteLength(JSON.stringify(data), 'utf8'));
console.log('Compressed size:', compressed.length);
```

## 平台支持

预编译二进制文件支持：
- Windows (x64, ARM64)
- macOS (x64, ARM64) 
- Linux (x64, ARM64, musl)

## 构建

从源码构建（需要 Rust 工具链）：

```bash
git clone <repository>
cd json-packer/napi
pnpm install
pnpm run build
```

## 注意事项

- 输入必须是有效的 JSON 字符串
- 不支持 NaN、Infinity 等非标准 JSON 值
- 压缩结果是确定性的（相同输入+配置 → 相同输出）
- 值池功能对小数据可能增加开销，建议在重复字符串较多时使用

## 许可证

MIT
