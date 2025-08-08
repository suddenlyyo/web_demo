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

### Axum
[axum_demo](./axum_demo) 目录包含使用 Axum 框架实现的示例。

### Rocket
[rocket_demo](./rocket_demo) 目录包含使用 Rocket 框架实现的示例。

## 快速开始

### 环境要求
- Rust 和 Cargo (推荐使用最新稳定版)
- 安装 rustfmt: `rustup component add rustfmt`

### 构建项目
```bash
cargo build
```

### 运行示例
进入各 demo 目录并运行：

```bash
# 运行 Axum 示例
cd axum_demo && cargo run

# 运行 Rocket 示例
cd rocket_demo && cargo run

# 运行 Actix Web 示例
cd actix_web_demo && cargo run
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
- **工具**: Cargo, rustfmt

## 设计原则

1. **模块化设计**: 分离业务逻辑与框架实现
2. **统一接口**: 使用通用的验证和响应封装模块
3. **易于扩展**: 清晰的架构便于添加新的功能和框架支持
4. **类型安全**: 充分利用 Rust 的类型系统确保代码安全

## 许可证

MIT