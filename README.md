# JSON Packer

一个用 Rust 编写的 JSON 二进制压缩库，支持 Node.js 和 WebAssembly 浏览器平台。

## 项目结构

- **[core](core)** - 实现所有压缩/解压缩逻辑的主 Rust 库
- **[wasm](wasm)** - 用于浏览器的 WebAssembly 绑定
- **[napi](napi)** - 使用 N-API 的 Node.js 原生绑定
- **[core-demo](core-demo)** - 命令行演示核心库
- **[wasm-demo](wasm-demo)** - 带交互式 UI 的 Web 演示
- **[napi-demo](napi-demo)** - Node.js 演示

## 许可证

MIT