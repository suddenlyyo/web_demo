# Web Demo

一个使用 Rust 编写的 Web 服务 Demo，旨在演示如何使用流行的 Rust Web 框架实现基本的 CRUD 功能。

## 项目结构

```
.
├── actix_web_demo      # 使用 Actix Web 框架的示例
├── axum_demo           # 使用 Axum 框架的示例
├── common_validation   # 通用验证库
├── common_validation_macros # 验证宏定义
├── common_wrapper      # 通用响应封装库
├── rocket_demo         # 使用 Rocket 框架的示例
├── sql                 # 数据库脚本
└── Cargo.toml          # 工作区配置
```

## 核心模块介绍

### common_validation
[通用验证库](./common_validation/README.md)，提供参数验证功能：
- 多种验证规则（非空、长度、日期格式、数值范围等）
- 灵活的验证规则组合
- 自定义错误类型
- 易于使用的验证器接口

### common_validation_macros
[验证宏库](./common_validation_macros/README.md)，提供派生宏来自动生成验证代码：
- 为结构体自动生成 `Validatable` 实现
- 支持多种验证属性
- 自动处理嵌套结构体的验证

### common_wrapper
[通用响应封装库](./common_wrapper/README.md)，提供统一的API响应格式：
- 统一的响应数据结构
- 支持多种响应类型（对象、列表、分页等）
- 标准化的错误处理

## Web框架示例

### Actix Web
[actix_web_demo](./actix_web_demo) 目录包含使用 Actix Web 框架实现的示例。

Actix Web Demo 支持多种数据库实现，通过 Rust 特性进行管理，默认使用 SQLx 实现。详情请参见 [Actix Web Demo README](./actix_web_demo/README.md)。

Actix Web 支持通过环境变量配置主机和端口：
- `HOST` - 服务器监听的主机地址（默认: 127.0.0.1）
- `PORT` - 服务器监听的端口号（默认: 8000）

当运行应用程序时，控制台会显示实际使用的地址和来源（环境变量或默认值）。

示例：
```bash
# 设置环境变量并运行
export HOST=0.0.0.0
export PORT=3000
cd actix_web_demo && cargo run

# 或者使用内联方式运行
HOST=0.0.0.0 PORT=3000 cd actix_web_demo && cargo run

# 不设置环境变量，使用默认值（127.0.0.1:8000）
cd actix_web_demo && cargo run
```

### Axum
[axum_demo](./axum_demo) 目录包含使用 Axum 框架实现的示例。

### Rocket
[rocket_demo](./rocket_demo) 目录包含使用 Rocket 框架实现的示例。

Rocket Demo 支持多种数据库实现，通过 Rust 特性进行管理，默认使用 SQLx 实现。详情请参见 [Rocket Demo README](./rocket_demo/README.md)。

## 快速开始

### 环境要求
- Rust 和 Cargo (推荐使用最新稳定版)
- 安装 rustfmt: `rustup component add rustfmt`

### 数据库配置
项目使用 MySQL 数据库，需要在运行前设置数据库连接：
```bash
export DATABASE_URL=mysql://user:password@localhost/database
```

### 构建项目
```bash
cargo build
```

### 运行示例
进入各 demo 目录并运行：

```bash
# 运行 Actix Web 示例（默认使用 SQLx）
cd actix_web_demo && cargo run

# 运行 Actix Web 示例（使用 Diesel）
cd actix_web_demo && cargo run --no-default-features --features diesel_impl

# 运行 Actix Web 示例（使用 SeaORM）
cd actix_web_demo && cargo run --no-default-features --features seaorm_impl

# 运行 Axum 示例
cd axum_demo && cargo run

# 运行 Rocket 示例（默认使用 SQLx）
cd rocket_demo && cargo run

# 运行 Rocket 示例（使用 Diesel）
cd rocket_demo && cargo run --no-default-features --features diesel_impl

# 运行 Rocket 示例（使用 SeaORM）
cd rocket_demo && cargo run --no-default-features --features seaorm_impl
```

### 运行测试
```bash
# 运行所有测试
cargo test

# 运行特定包的测试
cargo test -p common_validation
cargo test -p common_validation_macros
cargo test -p common_wrapper
```

## 技术栈

- **语言**: Rust
- **Web框架**: 
  - Axum
  - Rocket
  - Actix Web
- **ORM/数据库访问**:
  - SQLx
  - Diesel
  - SeaORM
- **工具**: Cargo, rustfmt