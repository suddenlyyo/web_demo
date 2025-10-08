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

# 使用 SQLx 实现运行
cargo run --no-default-features --features sqlx_impl

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
export PORT=8080
cargo run --no-default-features --features sqlx_impl
```

## API 接口文档

### 首页接口

- **URL**: `/`
- **方法**: `GET`
- **描述**: 健康检查接口，返回欢迎信息
- **成功响应**:
  ```json
  {
    "code": 1,
    "message": "Welcome to the Actix Web Demo"
  }
  ```

### 获取部门列表

- **URL**: `/dept/list`
- **方法**: `POST`
- **描述**: 分页获取部门列表，支持条件查询
- **请求体**:
  ```json
  {
    "deptName": "部门名称（可选）",
    "status": "状态（可选）",
    "pageNum": 1,
    "pageSize": 10
  }
  ```
- **成功响应**:
  ```json
  {
    "code": 1,
    "message": "Success",
    "data": [
      {
        "id": "1",
        "parentId": "0",
        "deptName": "研发部",
        "orderNum": 1,
        "status": 1,
        "createdAt": "2023-01-01T00:00:00+08:00",
        "updatedAt": "2023-01-01T00:00:00+08:00"
      }
    ],
    "total": 100,
    "totalPage": 10,
    "currentPage": 1,
    "pageSize": 10
  }
  ```

### 获取部门树结构

- **URL**: `/dept/getDeptTree`
- **方法**: `GET`
- **描述**: 获取完整的部门树结构
- **成功响应**:
  ```json
  {
    "code": 1,
    "message": "Success",
    "data": [
      {
        "id": "1",
        "parentId": "0",
        "deptName": "总公司",
        "orderNum": 1,
        "status": 1,
        "createdAt": "2023-01-01T00:00:00+08:00",
        "updatedAt": "2023-01-01T00:00:00+08:00",
        "children": [
          {
            "id": "2",
            "parentId": "1",
            "deptName": "研发部",
            "orderNum": 1,
            "status": 1,
            "createdAt": "2023-01-01T00:00:00+08:00",
            "updatedAt": "2023-01-01T00:00:00+08:00",
            "children": []
          }
        ]
      }
    ]
  }
  ```

### 添加部门

- **URL**: `/dept/add`
- **方法**: `POST`
- **描述**: 添加新部门
- **请求体**:
  ```json
  {
    "parentId": "0",
    "deptName": "新部门",
    "orderNum": 1,
    "status": 1
  }
  ```
- **成功响应**:
  ```json
  {
    "code": 1,
    "message": "操作成功"
  }
  ```

### 编辑部门

- **URL**: `/dept/edit`
- **方法**: `PUT`
- **描述**: 编辑部门信息
- **请求体**:
  ```json
  {
    "id": "1",
    "parentId": "0",
    "deptName": "更新后的部门名",
    "orderNum": 2,
    "status": 1
  }
  ```
- **成功响应**:
  ```json
  {
    "code": 1,
    "message": "操作成功"
  }
  ```

### 修改部门状态

- **URL**: `/dept/editStatus/{id}/{status}`
- **方法**: `PUT`
- **描述**: 修改部门状态（启用/禁用）
- **路径参数**:
  - `id`: 部门ID
  - `status`: 状态值（0-禁用，1-启用）
- **成功响应**:
  ```json
  {
    "code": 1,
    "message": "操作成功"
  }
  ```

### 删除部门

- **URL**: `/dept/delete/{id}`
- **方法**: `DELETE`
- **描述**: 根据ID删除部门
- **路径参数**:
  - `id`: 部门ID
- **成功响应**:
  ```json
  {
    "code": 1,
    "message": "操作成功"
  }
  ```

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

tests/
├── e2e_test.rs            # 端到端测试
└── integration_test.rs    # 集成测试
```

## 测试

项目包含两种类型的测试：

### 集成测试 (Integration Tests)
测试多个组件之间的交互，使用 Actix Web 的测试工具。

运行集成测试：
```bash
cargo test --test integration_test
```

### 端到端测试 (End-to-End Tests)
测试完整的应用程序，需要启动实际的服务。

1. 在一个终端启动服务：
```bash
cargo run
```

2. 在另一个终端运行端到端测试：
```bash
cargo test --test e2e_test
```