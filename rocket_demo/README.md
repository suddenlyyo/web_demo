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
# 设置环境变量并运行
export DATABASE_URL=mysql://user:password@localhost/database
cargo run
```

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

### 用户管理

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/user/<id>` | 根据ID获取用户信息 |
| GET | `/user/list` | 获取用户列表 |
| POST | `/user/query` | 分页查询用户列表 |
| POST | `/user` | 新增用户 |
| PUT | `/user/<id>` | 修改用户 |
| DELETE | `/user/<id>` | 删除用户 |

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

### 角色管理

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/role/<id>` | 根据ID获取角色信息 |
| GET | `/role/list` | 获取角色列表 |
| GET | `/role/page?<page_num>&<page_size>` | 分页查询角色列表 |
| POST | `/role` | 新增角色 |
| PUT | `/role/<id>` | 修改角色 |
| DELETE | `/role/<id>` | 删除角色 |
| PUT | `/role/<id>/status/<status>` | 修改角色状态 |

### 菜单管理

| 方法 | 路径 | 描述 |
|------|------|------|
| GET | `/menu/<id>` | 根据ID获取菜单信息 |
| GET | `/menu/list` | 获取菜单列表 |
| GET | `/menu/page?<page_num>&<page_size>` | 分页查询菜单列表 |
| POST | `/menu` | 新增菜单 |
| PUT | `/menu/<id>` | 修改菜单 |
| DELETE | `/menu/<id>` | 删除菜单 |

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
| dept_level | varchar(50) | 部门层级 |
| seq_no | int | 显示顺序 |
| status | int | 部门状态(0正常 1停用) |
| create_by | varchar(30) | 创建者 |
| create_time | datetime | 创建时间 |
| update_by | varchar(30) | 更新者 |
| update_time | datetime | 更新时间 |
| remark | varchar(200) | 备注 |

### 菜单表 (sys_menu)

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | char(32) | 菜单ID |
| name | varchar(30) | 菜单名称 |
| parent_id | char(32) | 父菜单ID |
| seq_no | int | 显示顺序 |
| menu_type | char(1) | 菜单类型（D目录 M菜单 B按钮） |
| url | varchar(200) | 请求地址 |
| perms | varchar(100) | 权限标识 |
| status | int | 菜单状态(0停用 1正常) |
| hidden | int | 是否在侧边栏隐藏(0显示 1隐藏) |
| always_show | int | 是否始终显示根菜单(0隐藏 1显示) |
| redirect | varchar(200) | 重定向地址 |
| component | varchar(200) | 当前路由外层包裹的组件信息 |
| href | varchar(200) | 外部链接地址 |
| icon | varchar(200) | 侧边栏中显示的图标 |
| no_cache | int | 不缓存页面(0缓存 1不缓存) |
| affix | int | 页面附加在标签视图中(0不附加 1附加) |
| breadcrumb | int | 该项目将隐藏在breadcrumb中(0隐藏 1显示) |
| active_menu | varchar(200) | 侧边栏会突出显示的路径 |
| create_by | varchar(30) | 创建者 |
| create_time | datetime | 创建时间 |
| update_by | varchar(30) | 更新者 |
| update_time | datetime | 更新时间 |
| remark | varchar(200) | 备注 |