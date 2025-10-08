# Rust Web框架演示项目

本项目演示了如何使用主流的 Rust Web 框架（Axum、Rocket、Actix Web）实现标准的 CRUD 功能，以及如何在多个框架之间共享业务逻辑。

## 项目结构

```
.
├── actix_web_demo     # Actix Web 框架示例
├── axum_demo          # Axum 框架示例
├── common_validation  # 公共参数验证库
├── common_wrapper     # 公共响应封装库
├── rocket_demo        # Rocket 框架示例
├── sql                # 数据库初始化脚本
└── Cargo.toml         # 工作区配置文件
```

## 各框架示例说明

### Axum 示例 (axum_demo)

使用 Axum 框架实现的 Web 服务示例。

- [详细文档](axum_demo/README.md)
- 默认端口: 8000
- 支持多种数据库实现（SQLx、Diesel、SeaORM）
- 配置方式：环境变量（HOST/PORT）

### Actix Web 示例 (actix_web_demo)

使用 Actix Web 框架实现的 Web 服务示例。

- [详细文档](actix_web_demo/README.md)
- 默认端口: 8000
- 支持多种数据库实现（SQLx、Diesel、SeaORM）
- 配置方式：环境变量（HOST/PORT）

### Rocket 示例 (rocket_demo)

使用 Rocket 框架实现的 Web 服务示例。

- [详细文档](rocket_demo/README.md)
- 默认端口: 8000
- 支持多种数据库实现（SQLx、Diesel、SeaORM）
- 配置方式：
  1. 默认配置
  2. Rocket.toml 文件
  3. 环境变量（以 ROCKET_ 为前缀）

## 公共库

### common_validation

提供参数验证功能，包括：
- 非空验证
- 长度验证
- 格式验证
- 范围验证
- 自定义验证规则

[详细文档](common_validation/README.md)

### common_wrapper

提供统一的 API 响应封装，包括：
- 成功响应
- 错误响应
- 分页响应
- 列表响应

[详细文档](common_wrapper/README.md)

## 快速开始

### 环境要求

- Rust 和 Cargo (推荐使用最新稳定版)
- MySQL 数据库

### 数据库配置

项目使用 MySQL 数据库，需要在运行前设置数据库连接：

```bash
export DATABASE_URL=mysql://user:password@localhost/database
```

数据库连接信息也可以通过项目根目录的 `config.toml` 文件配置。

### 运行项目

由于项目没有默认实现，必须明确指定要使用的数据库特性：

```bash
# 进入任一框架目录并运行项目，必须指定特性

# 使用 SQLx 实现
cd axum_demo && cargo run --no-default-features --features sqlx_impl
cd actix_web_demo && cargo run --no-default-features --features sqlx_impl
cd rocket_demo && cargo run --no-default-features --features sqlx_impl

# 使用 Diesel 实现
cd axum_demo && cargo run --no-default-features --features diesel_impl
cd actix_web_demo && cargo run --no-default-features --features diesel_impl
cd rocket_demo && cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cd axum_demo && cargo run --no-default-features --features seaorm_impl
cd actix_web_demo && cargo run --no-default-features --features seaorm_impl
cd rocket_demo && cargo run --no-default-features --features seaorm_impl
```

## 代码质量检查

为了确保代码质量，项目使用 Clippy 进行静态分析。由于项目支持多种数据库实现，需要为每种实现分别运行 Clippy：

```bash
# 为 actix_web_demo 检查不同的特性组合
cargo clippy -p actix_web_demo --features sqlx_impl
cargo clippy -p actix_web_demo --features diesel_impl
cargo clippy -p actix_web_demo --features seaorm_impl

# 为 axum_demo 检查不同的特性组合
cargo clippy -p axum_demo --features sqlx_impl
cargo clippy -p axum_demo --features diesel_impl
cargo clippy -p axum_demo --features seaorm_impl

# 为 rocket_demo 检查不同的特性组合
cargo clippy -p rocket_demo --features sqlx_impl
cargo clippy -p rocket_demo --features diesel_impl
cargo clippy -p rocket_demo --features seaorm_impl
```

同样，也可以使用 cargo check 来快速检查代码是否能编译通过：

```bash
# 为 actix_web_demo 检查不同的特性组合
cargo check -p actix_web_demo --features sqlx_impl
cargo check -p actix_web_demo --features diesel_impl
cargo check -p actix_web_demo --features seaorm_impl

# 为 axum_demo 检查不同的特性组合
cargo check -p axum_demo --features sqlx_impl
cargo check -p axum_demo --features diesel_impl
cargo check -p axum_demo --features seaorm_impl

# 为 rocket_demo 检查不同的特性组合
cargo check -p rocket_demo --features sqlx_impl
cargo check -p rocket_demo --features diesel_impl
cargo check -p rocket_demo --features seaorm_impl
```

## SeaORM CLI 工具

本项目支持使用 SeaORM ORM，如果需要使用 SeaORM 的代码生成功能，需要安装 `sea-orm-cli` 工具。

### 安装 SeaORM CLI

```bash
# 安装最新版 sea-orm-cli
cargo install sea-orm-cli

# 或者安装特定版本
cargo install sea-orm-cli@^2.0.0-rc
```

### 使用 SeaORM CLI

在启用了 `seaorm_impl` 特性的项目目录中，可以使用以下命令：

```bash
# 设置数据库 URL 环境变量
export DATABASE_URL=mysql://user:password@localhost/database

# 生成实体文件
sea-orm-cli generate entity -u mysql://user:password@localhost/database -o src/entities

# 查看所有可用命令
sea-orm-cli -h

# 查看 generate 子命令帮助
sea-orm-cli generate -h

# 查看生成实体文件的详细选项
sea-orm-cli generate entity -h
```

常用选项：
- `-u` / `--database-url`: 数据库 URL (默认: 环境变量 DATABASE_URL)
- `-s` / `--database-schema`: 数据库模式 (PostgreSQL 有效，默认: public)
- `-o` / `--output-dir`: 实体文件输出目录 (默认: 当前目录)
- `--expanded-format`: 生成展开格式的实体文件 (默认: 紧凑格式)
- `--with-serde`: 为实体自动派生 serde Serialize/Deserialize trait (none, serialize, deserialize, both)

## Diesel CLI 工具

本项目支持使用 Diesel ORM，如果需要使用 Diesel 的数据库迁移功能，需要安装 `diesel_cli` 工具。

### 安装 Diesel CLI

```bash
# 安装支持所有数据库后端的 diesel_cli（需要安装对应数据库的客户端库）
cargo install diesel_cli

# 如果只需要 MySQL 支持
cargo install diesel_cli --no-default-features --features mysql

# 如果只需要 PostgreSQL 支持
cargo install diesel_cli --no-default-features --features postgres

# 如果只需要 SQLite 支持
cargo install diesel_cli --no-default-features --features sqlite
```

注意：安装前需要确保系统已安装对应数据库的客户端开发库。

### 使用 Diesel CLI

在启用了 `diesel_impl` 特性的项目目录中，可以使用以下命令：

```bash
# 设置数据库 URL 环境变量
export DATABASE_URL=mysql://user:password@localhost/database

# 运行迁移
diesel migration run

# 创建新迁移
diesel migration generate migration_name

# 回滚迁移
diesel migration revert

# 重新运行迁移
diesel migration redo
```

### 从现有数据库表生成 Diesel 代码

如果已有数据库表结构，可以通过以下步骤生成 Diesel 相关代码：

1. 首先确保已安装 diesel_cli 工具（参考上面的安装说明）

2. 设置数据库连接环境变量：
   ```bash
   export DATABASE_URL=mysql://user:password@localhost/database
   ```

3. 在项目根目录下初始化 Diesel：
   ```bash
   diesel setup
   ```

4. 从现有数据库表生成模型代码：
   ```bash
   diesel print-schema > src/schema.rs
   ```

5. 根据生成的 schema.rs 文件手动创建对应的模型结构体和实现。
   
   注意：Diesel 不像 SeaORM 那样提供完整的实体代码生成工具，需要手动创建模型结构体并实现相关 trait。