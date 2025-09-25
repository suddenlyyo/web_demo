# Axum Demo

本项目演示了如何使用 [Axum](https://github.com/tokio-rs/axum) 框架实现标准的 CRUD 功能，以及如何在多个框架之间共享业务逻辑。

## 项目结构

```
axum_demo/
├── src/
│   ├── controllers/     # 控制器层，处理HTTP请求
│   ├── models/          # 数据模型
│   ├── params/          # 请求参数
│   ├── repositories/    # 数据访问层
│   ├── services/        # 业务逻辑层
│   ├── views/           # 视图模型
│   ├── config.rs        # 配置文件解析
│   └── main.rs          # 程序入口
├── tests/               # 测试文件
├── Cargo.toml           # 项目配置文件
└── README.md            # 项目说明文件
```

## 功能特性

- 基于 RESTful 风格的 CRUD 接口示例
- 参数校验（非空、长度、格式、范围等）
- 统一 API 响应结构封装
- 多数据库实现切换（SQLx、Diesel、SeaORM）
- 支持环境变量配置

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

## 配置说明

Axum 项目支持通过环境变量进行配置：

### 环境变量配置

```bash
# 设置主机和端口
export HOST=0.0.0.0
export PORT=3000
cd axum_demo && cargo run
```

或者使用内联方式：

```bash
HOST=0.0.0.0 PORT=3000 cd axum_demo && cargo run
```

默认配置：
- 主机：127.0.0.1
- 端口：8000

## 运行项目

```bash
# 使用默认的 SQLx 实现
cd axum_demo && cargo run

# 使用环境变量配置运行
cd axum_demo && HOST=0.0.0.0 PORT=3000 cargo run

# 使用 Diesel 实现
cd axum_demo && cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cd axum_demo && cargo run --no-default-features --features seaorm_impl
```

## API 接口

- `GET /` - 首页，返回欢迎信息
- `GET /dept/tree` - 获取部门树形结构
- `GET /dept/list` - 获取部门列表
- `GET /dept/{id}` - 根据ID获取部门详情
- `POST /dept` - 创建部门
- `PUT /dept` - 更新部门
- `DELETE /dept/{id}` - 根据ID删除部门

## 数据库配置

数据库连接信息通过环境变量 `DATABASE_URL` 配置：

```bash
export DATABASE_URL=mysql://user:password@localhost:3306/database
```

也可以在项目根目录的 `config.toml` 文件中配置：

```toml
[database]
url = "mysql://user:password@localhost:3306/database"
```