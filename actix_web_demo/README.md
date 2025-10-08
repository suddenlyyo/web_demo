# Actix Web Demo

本项目是基于 Actix Web 框架实现的 Web 服务示例，展示了如何使用 Actix Web 构建 RESTful API 服务。

## 功能特性

- 基于 Actix Web 框架构建
- 支持多种数据库访问方式（SQLx、Diesel、SeaORM）
- 实现部门管理的 CRUD 操作
- 统一的参数验证和响应封装
- 支持通过环境变量配置服务地址和端口

## Diesel CLI 工具

如果需要使用 Diesel 的数据库迁移功能，需要安装 `diesel_cli` 工具。

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

在本项目目录中，可以使用以下命令操作数据库：

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
# 进入项目目录
cd actix_web_demo

# 使用 SQLx 实现运行（sqlx_impl 是默认特性）
cargo run --features sqlx_impl

# 使用 Diesel 实现运行
cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现运行
cargo run --no-default-features --features seaorm_impl
```

### 配置服务地址和端口

Actix Web Demo 支持通过环境变量配置服务监听的地址和端口：

- `HOST` - 服务器监听的主机地址（默认: 127.0.0.1）
- `PORT` - 服务器监听的端口号（默认: 8000）

示例：
```bash
# 设置环境变量并运行
export HOST=0.0.0.0
export PORT=3000
cargo run

# 或者使用内联方式运行
HOST=0.0.0.0 PORT=3000 cargo run
```

当运行应用程序时，控制台会显示实际使用的地址和来源（环境变量或默认值）。

## 项目结构

```
src/
├── config.rs              # 配置文件解析
├── main.rs                # 程序入口
├── controllers/           # 控制器层
│   ├── dept/              # 部门相关控制器
│   └── index/             # 首页控制器
├── models/                # 数据模型
├── params/                # 请求参数
├── repositories/          # 数据访问层
│   └── dept/              # 部门数据访问
│       ├── diesel_impl/   # Diesel 实现
│       ├── seaorm_impl/   # SeaORM 实现
│       ├── sqlx_impl/     # SQLx 实现
│       └── dept_repository.rs  # 数据访问接口
├── services/              # 服务层
│   └── dept/              # 部门服务
└── views/                 # 视图模型
```

## API 接口

项目提供部门管理相关的 RESTful API 接口：

- `GET /dept/tree` - 获取部门树形结构
- `GET /dept/list` - 获取部门列表
- `GET /dept/{id}` - 根据 ID 获取部门详情
- `POST /dept` - 创建部门
- `PUT /dept` - 更新部门
- `DELETE /dept/{id}` - 根据 ID 删除部门

所有接口都返回统一格式的 JSON 响应。