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

## 公共库

### common_validation

提供参数验证功能，包括：
- 非空验证
- 长度验证
- 格式验证
- 范围验证
- 自定义验证规则

### common_wrapper

提供统一的 API 响应封装，包括：
- 成功响应
- 错误响应
- 分页响应
- 列表响应

## 各框架示例说明

### Rocket 示例 (rocket_demo)

使用 Rocket 框架实现的 Web 服务示例。

支持多种数据库实现：
- SQLx（默认）
- Diesel
- SeaORM

配置方式：
1. 默认配置
2. Rocket.toml 文件
3. 环境变量（以 ROCKET_ 为前缀）

运行方式：
```bash
# 使用默认的 SQLx 实现
cd rocket_demo && cargo run

# 使用环境变量配置运行（优先级最高）
cd rocket_demo && ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=8080 cargo run

# 使用 Diesel 实现
cd rocket_demo && cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cd rocket_demo && cargo run --no-default-features --features seaorm_impl
```

### Axum 示例 (axum_demo)

使用 Axum 框架实现的 Web 服务示例。

支持多种数据库实现：
- SQLx（默认）
- Diesel
- SeaORM

配置方式：
- 通过环境变量配置（HOST, PORT）

运行方式：
```bash
# 使用默认的 SQLx 实现
cd axum_demo && HOST=0.0.0.0 PORT=3000 cargo run

# 使用 Diesel 实现
cd axum_demo && HOST=0.0.0.0 PORT=3000 cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cd axum_demo && HOST=0.0.0.0 PORT=3000 cargo run --no-default-features --features seaorm_impl
```

### Actix Web 示例 (actix_web_demo)

使用 Actix Web 框架实现的 Web 服务示例。

支持多种数据库实现：
- SQLx（默认）
- Diesel
- SeaORM

配置方式：
- 通过环境变量配置（HOST, PORT）

运行方式：
```bash
# 使用默认的 SQLx 实现
cd actix_web_demo && HOST=0.0.0.0 PORT=3000 cargo run

# 使用 Diesel 实现
cd actix_web_demo && HOST=0.0.0.0 PORT=3000 cargo run --no-default-features --features diesel_impl

# 使用 SeaORM 实现
cd actix_web_demo && HOST=0.0.0.0 PORT=3000 cargo run --no-default-features --features seaorm_impl
```

## 数据库配置

所有示例都需要配置数据库连接，通过设置 DATABASE_URL 环境变量：

```bash
export DATABASE_URL=mysql://user:password@localhost/database
```

数据库初始化脚本位于 [sql/demo.sql](sql/demo.sql)。

## 测试

每个框架示例都包含集成测试和端到端测试：

```bash
# 运行集成测试
cargo test --test integration_test

# 运行端到端测试（需要先启动服务器）
cargo test --test e2e_test
```