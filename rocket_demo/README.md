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

### 角色表 (sys_role)

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | char(32) | 角色ID |
| name | varchar(30) | 角色名称 |
| role_key | varchar(100) | 角色权限字符串 |
| seq_no | int | 显示顺序 |
| status | int | 角色状态(0停用 1正常) |
| create_by | varchar(30) | 创建者 |
| create_time | datetime | 创建时间 |
| update_by | varchar(30) | 更新者 |
| update_time | datetime | 更新时间 |
| remark | varchar(200) | 备注 |

### 用户表 (sys_user)

| 字段名 | 类型 | 说明 |
|--------|------|------|
| id | char(32) | 用户ID |
| dept_id | char(32) | 部门ID |
| name | varchar(30) | 用户账号 |
| email | varchar(50) | 用户邮箱 |
| phone_number | varchar(11) | 手机号码 |
| sex | char(1) | 用户性别(0未知 1男 2女) |
| password | varchar(100) | 密码 |
| avatar | varchar(100) | 头像 |
| status | int | 账号状态(0停用 1正常) |
| login_ip | varchar(128) | 最后登录IP |
| login_time | datetime | 最后登录时间 |
| create_by | varchar(30) | 创建者 |
| create_time | datetime | 创建时间 |
| update_by | varchar(30) | 更新者 |
| update_time | datetime | 更新时间 |
| remark | varchar(200) | 备注 |

### 角色菜单关联表 (sys_role_menu)

| 字段名 | 类型 | 说明 |
|--------|------|------|
| role_id | char(32) | 角色ID |
| menu_id | char(32) | 菜单ID |

### 用户角色关联表 (sys_user_role)

| 字段名 | 类型 | 说明 |
|--------|------|------|
| user_id | char(32) | 用户ID |
| role_id | char(32) | 角色ID |