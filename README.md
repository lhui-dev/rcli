## ✨ 项目简介

**rcli** 是一个开箱即用的 Rust CLI项目。

## 功能特性

- **CSV 处理**：支持将 CSV 文件转换为 JSON/YAML 格式，可自定义分隔符、是否包含表头，支持指定输入输出路径。
- **密码生成**：支持生成自定义长度、字符集的随机密码，支持批量生成，可输出到控制台或文件。
- **Base64 编解码**：支持标准与 URL 安全两种格式的 Base64 编码与解码，支持文件或标准输入，正确处理二进制数据。
- **HTTP 文件服务**：快速将本地目录挂载为静态 HTTP 文件服务，可自定义监听地址与端口。

## 快速开始

### 前置条件

- 已安装 Rust 环境（推荐 1.70+）：[Rust 安装指南](https://www.rust-lang.org/tools/install)

### 构建项目

```bash
# 克隆项目
git clone https://github.com/lhui-dev/rcli.git
cd rcli

# 构建项目（debug 模式）
cargo build

# 构建发布版本（性能更优）
cargo build --release
```

构建完成后，可执行文件位于 `target/debug/rcli`（debug 模式）或 `target/release/rcli`（release 模式）。

### 基本使用

#### 查看帮助

```bash
# 查看全局帮助
./rcli --help

# 查看各子命令帮助
./rcli csv --help
./rcli passwd --help
./rcli base64 --help
./rcli http --help
```

## 功能详情

### 1. CSV 格式转换（csv 子命令）

将 CSV 文件转换为 JSON 或 YAML 格式，支持自定义分隔符、表头配置。

#### 命令参数

| 参数            | 简写   | 默认值                    | 说明                    |
|---------------|------|------------------------|-----------------------|
| `--input`     | `-i` | `input.csv`            | 指定输入 CSV 文件路径         |
| `--output`    | -    | 自动生成（output.json/yaml） | 指定输出文件路径              |
| `--delimiter` | `-d` | `,`                    | CSV 文件分隔符（单字节字符）      |
| `--no-header` | -    | `false`                | 指定 CSV 文件无表头          |
| `--format`    | -    | `json`                 | 输出格式，支持 `json`/`yaml` |

#### 使用示例

```bash
# 基础用法：默认输入 input.csv，输出为 output.json
./rcli csv

# 自定义输入文件、输出格式为 YAML
./rcli csv -i data.csv --format yaml

# 自定义分隔符（如制表符 \t）、指定输出路径
./rcli csv -i test.csv -d $'\t' --output result.json

# 处理无表头的 CSV 文件
./rcli csv -i no_header.csv --no-header --format json
```

### 2. 随机密码生成（passwd 子命令）

生成符合自定义规则的随机密码，支持批量生成、指定输出文件。

#### 命令参数

| 参数               | 简写   | 默认值       | 说明                  |
|------------------|------|-----------|---------------------|
| `--length`       | `-l` | `16`      | 密码长度（需大于启用的字符集数量）   |
| `--no-uppercase` | -    | `false`   | 不包含大写字母（A-Z）        |
| `--no-lowercase` | -    | `false`   | 不包含小写字母（a-z）        |
| `--no-numeric`   | -    | `false`   | 不包含数字（1-9）          |
| `--no-symbolic`  | -    | `false`   | 不包含特殊符号（!@#$%^&*_）  |
| `--batch-count`  | `-n` | `1`       | 批量生成密码数量（≥1）        |
| `--output`       | -    | `console` | 输出密码到指定文件（默认打印到控制台） |

#### 使用示例

```bash
# 基础用法：生成 16 位包含所有字符集的密码
./rcli passwd

# 生成 20 位仅包含小写字母和数字的密码（批量 5 个）
./rcli passwd -l 20 --no-uppercase --no-symbolic -n 5

# 生成无特殊符号的密码，输出到文件
./rcli passwd --no-symbolic --output passwords.txt

# 生成 8 位仅包含大写字母和数字的密码
./rcli passwd -l 8 --no-lowercase --no-symbolic
```

### 3. Base64 编解码（base64 子命令）

对文件或标准输入进行 Base64 编码或解码，支持标准与 URL 安全两种格式，正确处理任意二进制数据。

#### 子命令

| 子命令      | 说明               |
|----------|------------------|
| `encode` | 将数据编码为 Base64    |
| `decode` | 将 Base64 解码为原始数据 |

#### 命令参数（encode / decode 共用）

| 参数         | 简写   | 默认值        | 说明                                            |
|------------|------|------------|-----------------------------------------------|
| `--input`  | `-i` | `-`（标准输入）  | 指定输入文件路径，`-` 表示从标准输入读取                        |
| `--format` | -    | `standard` | 编码格式，支持 `standard`（标准）/ `urlsafe`（URL 安全，无填充） |

#### 使用示例

```bash
# 从标准输入编码（standard 格式）
echo -n "hello world" | ./rcli base64 encode
# aGVsbG8gd29ybGQ=

# 编码文件（URL 安全格式）
./rcli base64 encode --input ./data.bin --format urlsafe

# 从标准输入解码
echo "aGVsbG8gd29ybGQ=" | ./rcli base64 decode

# 解码文件，输出重定向（支持二进制）
./rcli base64 decode --input encoded.txt > output.bin

# 编解码往返验证
echo -n "round trip" | ./rcli base64 encode | ./rcli base64 decode
```

> **说明**：解码输出为原始字节，若原始数据为二进制（如图片、压缩包），请使用重定向（`> file`）保存，而非直接打印到终端。

### 4. HTTP 静态文件服务（http 子命令）

将本地目录快速挂载为 HTTP 静态文件服务，适合本地开发调试或局域网文件共享。

#### 子命令

| 子命令     | 说明             |
|---------|----------------|
| `serve` | 启动静态文件 HTTP 服务 |

#### 命令参数

| 参数       | 简写 | 默认值         | 说明         |
|----------|----|-------------|------------|
| `--dir`  | -  | `.`（当前目录）   | 指定要挂载的目录路径 |
| `--ip`   | -  | `127.0.0.1` | 监听 IP 地址   |
| `--port` | -  | `8080`      | 监听端口       |

#### 使用示例

```bash
# 基础用法：将当前目录挂载到 127.0.0.1:8080
./rcli http serve

# 指定目录与端口
./rcli http serve --dir ./public --port 3000

# 监听所有网卡（局域网可访问）
./rcli http serve --dir ./dist --ip 0.0.0.0 --port 8080
```

## 注意事项

1. **CSV 转换**：
    - 分隔符仅支持单字节字符，不可使用空字符（\0）；
    - 若指定输出目录不存在，会直接报错。
2. **密码生成**：
    - 至少需启用一个字符集（大写/小写/数字/符号）；
    - 密码长度需≥启用的字符集数量（确保每个字符集至少出现1次）；
    - 批量生成数量需≥1；
    - 若指定输出文件路径，其父目录必须存在，且路径不能是目录（需是文件）。
3. **Base64 编解码**：
    - `urlsafe` 格式使用 `-` 和 `_` 替代 `+` 和 `/`，且不添加 `=` 填充；
    - 标准格式与 URL 安全格式不可混用（用一种格式编码的数据必须用同一格式解码）；
    - 解码输出为原始字节流，二进制数据请用重定向保存。

4. **HTTP 文件服务**：
    - 指定的目录路径必须存在；
    - 对外暴露时请注意文件访问安全。

## 错误处理

工具会对非法参数、文件不存在、路径错误等场景返回明确的错误提示，例如：

- 密码长度为 0 时：`Password length must be greater than 0`；
- 未启用任何字符集时：`At least one of character set must be enabled（uppercase/lowercase/numeric/symbolic）`；
- CSV 文件不存在时：`failed to open file xxx.csv`。
- Base64 解码失败时：`Invalid byte in base64 input`

## 测试

项目包含单元测试，可通过以下命令运行：

```bash
# 运行所有测试
cargo test

# 运行指定模块测试
cargo test base64
cargo test csv

# 显示测试输出
cargo test -- --nocapture
```

测试覆盖 CSV 转换、密码生成、Base64 编解码的核心逻辑，以及异常场景（如文件不存在、参数非法、二进制数据往返等）。