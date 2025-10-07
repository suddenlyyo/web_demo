# Rocket Demo

使用 Rocket 框架实现的 Web 服务示例。

## 数据库实现特性

本项目支持多种数据库实现，通过 Rust 的特性（features）系统进行管理：

### 可用特性

- `sqlx_impl` - 使用 SQLx 作为数据库实现
- `diesel_impl` - 使用 Diesel 作为数据库实现
- `seaorm_impl` - 使用 SeaORM 作为数据库实现

### 特性使用方法

```bash
# 使用 SQLx 实现（需要显式指定）
cargo run --features sqlx_impl

# 使用 Diesel 实现
cargo run --features diesel_impl

# 使用 SeaORM 实现
cargo run --features seaorm_impl
```

注意：一次只能启用一种数据库实现特性。

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

## 配置

Rocket框架支持多种配置方式，按照优先级从低到高依次为：

1. 默认配置
2. Rocket.toml 文件
3. 环境变量（以 ROCKET_ 为前缀）

### 配置文件 (Rocket.toml)

项目支持使用 Rocket.toml 文件进行配置。配置文件支持多个配置环境（profiles），包括：
- default - 所有环境的默认配置
- debug - 调试模式配置
- release - 发布模式配置

示例配置文件：
```toml
[default]
address = "0.0.0.0"
port = 8000

[debug]
port = 8000

[release]
port = 9999
```

### 环境变量配置

可以通过设置环境变量来配置服务器，所有环境变量都以 ROCKET_ 为前缀：

```bash
ROCKET_ADDRESS=0.0.0.0
ROCKET_PORT=3000
```

环境变量运行方式：
```bash
# 使用环境变量运行（优先级最高）
ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=8080 cargo run

# 或者先设置环境变量再运行
export ROCKET_ADDRESS=0.0.0.0
export ROCKET_PORT=8080
cargo run
```

## 数据库配置

项目使用 MySQL 数据库，需要设置以下环境变量：

- `DATABASE_URL` - 数据库连接字符串

示例：
```env
DATABASE_URL=mysql://user:password@localhost/database
```

## 运行项目

```
# 设置环境变量并运行（以SQLx实现为例）
export DATABASE_URL=mysql://user:password@localhost/database
cargo run --features sqlx_impl
```

## 测试

项目包含两种类型的测试：

### 集成测试

集成测试位于 `tests/integration_test.rs` 文件中，主要测试各个组件的功能，包括：
- Rocket框架基本功能
- 各种数据库实现（SQLx、Diesel、SeaORM）的集成测试
- 控制器、服务层和数据访问层的单元测试

运行集成测试：
```bash
# 运行所有集成测试
cargo test --test integration_test

# 运行特定实现的测试（需要相应配置数据库）
cargo test --test integration_test --features sqlx_impl
cargo test --test integration_test --no-default-features --features diesel_impl
cargo test --test integration_test --no-default-features --features seaorm_impl
```

### 端到端测试

端到端测试位于 `tests/e2e_test.rs` 文件中，用于模拟真实用户请求，测试完整的HTTP请求处理流程。

运行端到端测试需要先启动服务器：
```bash
# 在一个终端中启动服务器
cargo run

# 在另一个终端中运行端到端测试
cargo test --features seaorm_impl --test e2e_test
```

端到端测试会向运行中的服务器发送真实的HTTP请求，验证以下接口：
- 添加部门接口 (`POST /dept/dept/add`)
- 部门列表接口 (`GET /dept/dept/list`)
- 部门树接口 (`GET /dept/dept/getDeptTree`)
- 编辑部门接口 (`PUT /dept/dept/edit`)
- 修改部门状态接口 (`PUT /dept/dept/editStatus/{id}/{status}`)
- 删除部门接口 (`DELETE /dept/dept/delete/{id}`)

#### 测试执行顺序

端到端测试按照CRUD操作顺序执行，以确保测试数据的一致性和可预测性：
1. Create (创建) - 首先测试添加新部门功能
2. Read (读取) - 然后测试查询部门列表和树结构
3. Update (更新) - 接着测试编辑部门和修改部门状态功能
4. Delete (删除) - 最后测试删除部门功能

这种执行顺序确保了测试数据在测试过程中被正确创建、使用和清理，避免了测试数据在数据库中的长期累积。

## API 响应格式

本项目使用 `common_wrapper` crate 提供的统一响应格式，所有 API 响应都遵循以下格式：

### SingleWrapper<T>

用于包装单个对象的响应：

```json
{
  "code": 200,
  "message": "Success",
  "data": {}
}
```

### ListWrapper<T>

用于包装列表对象的响应：

```json
{
  "code": 200,
  "message": "Success",
  "data": []
}
```

### PageWrapper<T>

用于包装分页对象的响应：

```json
{
  "code": 200,
  "message": "Success",
  "data": [],
  "total": 100,
  "total_page": 10,
  "current_page": 1,
  "page_size": 10
}
```

## API 接口

### 部门管理

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/dept/<id>` | 根据ID获取部门信息 |
| GET | `/dept/list` | 获取部门列表 |
| GET | `/dept/page?<page_num>&<page_size>` | 分页查询部门列表 |
| GET | `/dept/children/<parent_id>` | 根据父部门ID获取子部门列表 |
| GET | `/dept/tree` | 获取部门树结构 |
| POST | `/dept` | 新增部门 |
| PUT | `/dept/<id>` | 修改部门 |
| DELETE | `/dept/<id>` | 删除部门 |

## 数据库表结构

项目使用以下数据库表，表结构定义在 [sql/demo.sql](../sql/demo.sql) 文件中：

### 部门表 (sys_dept)

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | char(32) | 部门id |
| name | varchar(30) | 部门名称 |
| email | varchar(50) | 邮箱 |
| telephone | varchar(11) | 联系电话 |
| address | varchar(200) | 地址 |
| logo | varchar(100) | logo地址 |
| parent_id | char(32) | 父部门id |
| seq_no | int | 显示顺序 |
| status | int | 部门状态(0正常 1停用) |
| create_by | varchar(30) | 创建者 |
| create_time | datetime | 创建时间 |
| update_by | varchar(30) | 更新者 |
| update_time | datetime | 更新时间 |
| remark | varchar(200) | 备注 |

<!-- 最后一行保持空白 -->