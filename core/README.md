# json-packer

Rust 核心库：轻量、可逆的 JSON 二进制压缩/解压工具。

## 特性

- **无损压缩**：保留原始数值精度（整数、浮点分离编码）
- **确定性输出**：相同输入+配置 → 相同二进制结果
- **键名优化**：Canonical Huffman 编码，高效压缩对象键
- **可选值池**：重复字符串去重（v2 格式，可选）
- **跨平台**：纯 Rust，支持 N-API 和 WASM 绑定
- **Base64 就绪**：直接支持 Base64 编解码

## 安装

```toml
[dependencies]
json-packer = "0.1.0"
# 或本地开发：
# json-packer = { path = "../core" }
```

## 公共 API

```rust
// 压缩 / 解压（字节）
pub fn compress_to_bytes(value: &serde_json::Value, opts: &CompressOptions) -> Result<Vec<u8>, Error>;
pub fn decompress_from_bytes(bytes: &[u8]) -> Result<serde_json::Value, Error>;

// 压缩 / 解压（Base64）
pub fn compress_to_base64(value: &serde_json::Value, opts: &CompressOptions) -> Result<String, Error>;
pub fn decompress_from_base64(s: &str) -> Result<serde_json::Value, Error>;

// 压缩可选项（无状态，按调用传入）
#[derive(Clone, Debug)]
pub struct CompressOptions {
  pub enable_value_pool: bool,     // 是否启用字符串值池（默认 false）
  pub pool_min_repeats: u32,       // 计入值池的最小重复次数（默认 3）
  pub pool_min_string_len: usize,  // 计入值池的最小字符串长度（默认 8）
}

// 通过 options 压缩的函数（示例，用户可在自己代码中调用 encode::compress_with_options）
// pub fn encode::compress_with_options(v: &Value, opt: &CompressOptions) -> Result<Vec<u8>, Error>;
```

## 快速上手

```rust
use json_packer::{compress_to_base64, decompress_from_base64, CompressOptions};
use serde_json::json;

let v = json!({
  "ok": true,
  "count": 42,
  "name": "Alice"
});

let b64 = compress_to_base64(&v, &CompressOptions::default())?;
let out = decompress_from_base64(&b64)?;
assert_eq!(v, out);
# Ok::<(), Box<dyn std::error::Error>>(())
```

### 启用字符串值池（无状态调用）

```rust
use json_packer::{CompressOptions};
use json_packer::encode::compress_with_options; // 按调用传入，无全局状态
use serde_json::json;

let opt = CompressOptions{
  enable_value_pool: true,       // 打开字符串值池（输出 v2 格式）
  pool_min_repeats: 3,
  pool_min_string_len: 8,
};

let v = json!({
  "items": [
    {"status":"connected", "msg":"connected to server"},
    {"status":"connected", "msg":"connected to server"},
    {"status":"connected", "msg":"connected to server"}
  ]
});
let bytes = compress_with_options(&v, &opt)?; // 启用后会写入值池区
# Ok::<(), Box<dyn std::error::Error>>(())
```

## 数据格式与版本
- v1（默认）：无值池。头部 `VERSION=0x01`。
- v2：启用字符串值池（传入 `enable_value_pool=true` 时自动使用）。头部 `VERSION=0x02`，写入 `POOL_LEN` 与“值池区”；string 在数据区写 `is_pool_ref(1b)` 决定引用 ID 或内联。
- 确定性：同一输入与相同配置下，输出字节完全一致。

## 错误类型（节选）
- `BadMagic`/`BadVersion`：头部不合法
- `BitstreamOutOfBounds`/`VarintError`：位流或变长整数读写越界
- `IllegalFloat`：浮点为 NaN/±Inf（JSON 不允许）
- `HuffmanError`：霍夫曼构建/解码失败
- `PoolMissing`/`PoolIdOutOfRange`：值池引用缺失或越界

## 注意事项
- 遵循 JSON 规范：不支持 NaN/±Inf。
- Base64 接口既可解码无填充（默认）也兼容标准填充格式。

## Demo
仓库包含 `demo/` 二进制示例：读取 `demo/test_large.json`，输出压缩结果与统计。运行：
```bash
cd demo && cargo run --release
```

## 许可证
MIT
