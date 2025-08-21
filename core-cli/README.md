# JSON Packer CLI

`json-packer` core 包的命令行工具。

## 安装

```bash
cd core-cli
cargo build --release
```

## 使用方法

### 基本用法

#### 压缩 JSON 文件
```bash
# 压缩到 Base64 格式
json-packer-cli compress data.json

# 压缩到文件
json-packer-cli compress data.json compressed.jcp

# 启用字符串值池优化
json-packer-cli compress --enable-pool data.json

# 显示压缩统计信息
json-packer-cli compress --stats data.json
```

#### 解压文件
```bash
# 解压 Base64 格式
json-packer-cli decompress compressed.jcp

# 美化输出
json-packer-cli decompress --pretty compressed.jcp

# 解压到文件
json-packer-cli decompress compressed.jcp restored.json
```

#### 查看文件信息
```bash
# 基本信息
json-packer-cli info compressed.jcp

# 详细信息
json-packer-cli info --detailed compressed.jcp
```

#### 批量处理
```bash
# 批量压缩 JSON 文件
json-packer-cli batch compress "*.json"

# 批量解压，启用值池
json-packer-cli batch compress --enable-pool --recursive "data/"

# 批量解压
json-packer-cli batch decompress "*.jcp" --output-dir results/
```

### 命令参考

#### `compress` 命令
压缩 JSON 文件或数据

**选项**:
- `--format <FORMAT>`: 输出格式 [base64|bytes] (默认: base64)
- `--enable-pool`: 启用字符串值池 (v2 格式)
- `--pool-min-repeats <NUM>`: 值池最小重复次数 (默认: 3)
- `--pool-min-string-len <LEN>`: 值池最小字符串长度 (默认: 8)
- `--pretty`: 美化输出 (仅适用于 base64 格式)
- `--stats`: 显示压缩统计信息

#### `decompress` 命令
解压 JSON 数据

**选项**:
- `--format <FORMAT>`: 输入格式 [auto|base64|bytes] (默认: auto)
- `--pretty`: 美化 JSON 输出
- `--compact`: 紧凑 JSON 输出

#### `info` 命令
查看压缩文件信息

**选项**:
- `--format <FORMAT>`: 输入格式 [auto|base64|bytes] (默认: auto)
- `--detailed`: 显示详细信息

#### `batch` 命令
批量处理多个文件

**选项**:
- `--output-dir <DIR>`: 输出目录
- `--output-suffix <SUFFIX>`: 输出文件后缀 (默认: .jcp)
- `--parallel <NUM>`: 并行处理数量 (默认: CPU 核心数)
- `--recursive`: 递归处理子目录
- `--enable-pool`: 启用字符串值池

### 全局选项

- `-v, --verbose`: 显示详细输出
- `-q, --quiet`: 静默模式
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

## 示例

### 基本压缩和解压
```bash
# 创建测试 JSON
echo '{"name": "John", "age": 30, "city": "New York"}' > test.json

# 压缩
json-packer-cli compress test.json compressed.jcp

# 查看信息
json-packer-cli info compressed.jcp

# 解压
json-packer-cli decompress compressed.jcp restored.json

# 验证
diff test.json restored.json
```

### 启用值池优化
```bash
# 对于包含重复字符串的大型 JSON
json-packer-cli compress --enable-pool --stats large_data.json
```

### 批量处理
```bash
# 批量压缩当前目录所有 JSON 文件
json-packer-cli batch compress "*.json" --output-dir compressed/

# 递归批量压缩
json-packer-cli batch compress --recursive "data/" --enable-pool
```

## 错误码

- 0: 成功
- 2: IO 错误
- 3: JSON 解析错误  
- 4: 压缩/解压错误
- 5: 无效格式
- 6: 文件未找到
- 7: 无效命令行参数
- 8: 批处理错误

## 开发

```bash
# 开发模式运行
cargo run -- compress test.json

# 运行测试
cargo test

# 检查代码
cargo check
cargo clippy
```