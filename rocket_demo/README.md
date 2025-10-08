# Rocket Demo

使用 Rocket 框架实现的 Web 服务示例。

## 数据库实现特性

本项目支持多种数据库实现，通过 Rust 的特性（features）系统进行管理：

### 默认特性

- `sqlx_impl` - 默认启用，使用 SQLx 作为数据库实现

### 可选特性

- `diesel_impl` - 使用 Diesel 作为数据库实现
- `seaorm_impl` - 使用 SeaORM 作为数据库实现

### 特性使用方法

```bash
# 使用默认的 SQLx 实现
cargo run

# 使用 Diesel 实现
cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cargo run --no-default-features --features seaorm_impl

# 使用 SQLx 实现（显式指定）
cargo run --features sqlx_impl
```

注意：一次只能启用一种数据库实现特性。

## 数据库配置

项目使用 MySQL 数据库，需要设置以下环境变量：

- `DATABASE_URL` - 数据库连接字符串

示例：
```env
DATABASE_URL=mysql://user:password@localhost/database
```

## 运行项目

```bash
# 设置环境变量并运行（以SQLx实现为例）
export DATABASE_URL=mysql://user:password@localhost/database
cargo run --features sqlx_impl
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
    "message": "Welcome to the Rocket Demo"
  }
  ```

### 获取部门列表

- **URL**: `/dept/dept/list`
- **方法**: `GET`
- **描述**: 分页获取部门列表，支持条件查询
- **查询参数**:
  - `deptName`: 部门名称（可选）
  - `status`: 状态（可选）
  - `pageNum`: 页码，默认为1
  - `pageSize`: 每页大小，默认为10
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

- **URL**: `/dept/dept/getDeptTree`
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

- **URL**: `/dept/dept/add`
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

- **URL**: `/dept/dept/edit`
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

- **URL**: `/dept/dept/editStatus/{id}/{status}`
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

- **URL**: `/dept/dept/delete/{id}`
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

## API 响应格式

本项目使用 `common_wrapper` crate 提供的统一响应格式，所有 API 响应都遵循以下格式：

### SingleWrapper<T>

用于包装单个对象的响应：

```json
{
  "code": 1,
  "message": "Success",
  "data": {}
}
```

### ListWrapper<T>

用于包装列表对象的响应：

```json
{
  "code": 1,
  "message": "Success",
  "data": []
}
```

### PageWrapper<T>

用于包装分页对象的响应：

```json
{
  "code": 1,
  "message": "Success",
  "data": [],
  "total": 100,
  "totalPage": 10,
  "currentPage": 1,
  "pageSize": 10
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