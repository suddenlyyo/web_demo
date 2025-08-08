# Common Validation Macros

提供用于自动生成验证代码的派生宏。

## 功能特性

- 为结构体自动生成 `Validatable` 实现
- 支持多种验证属性（not_null、length、date_format等）
- 自动处理嵌套结构体的验证
- 支持Option和Vec类型的验证

## 设计理念

### 1. 宏与验证器分离
本库遵循关注点分离原则，将代码生成（宏）与验证逻辑（验证器）完全分离：
- **宏**：只负责生成验证代码，不包含实际的验证逻辑
- **验证器**：负责执行具体的验证逻辑

这种设计使得：
- 宏的职责单一，代码更加清晰
- 验证逻辑集中管理，便于维护和扩展
- 可以独立优化和测试验证逻辑

### 2. 类型安全与性能
充分利用Rust的类型系统，在编译时进行类型检查，避免运行时类型错误：
- 通过trait约束确保类型安全
- 编译时生成特定类型的验证代码
- 避免运行时类型判断带来的性能开销

### 3. 易用性与灵活性
在保证类型安全的前提下，提供简单易用的接口：
- 通过派生宏自动生成验证代码，减少样板代码
- 支持链式调用的验证规则定义
- 灵活的验证规则组合

### 4. 专注Web API场景
专门针对HTTP请求中的JSON数据验证设计：
- 主要处理字符串和数字类型验证
- 支持嵌套结构体验证
- 符合Web开发的常见需求

## 使用方法

在结构体上添加 `#[derive(ValidatableImpl)]` 属性，并在字段上使用 `#[validate(...)]` 属性来定义验证规则：

```rust
use common_validation::{Validatable, DateTimeFormatEnum};
use common_validation_macros::ValidatableImpl;

#[derive(ValidatableImpl)]
struct User {
    #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
    username: String,
    
    #[validate(not_null, date_format = Year, desc = "生日")]
    birthday: String,
}
```

## 支持的验证属性

### 基本验证属性

- `not_null`: 非空验证
- `length = N`: 固定长度验证
- `length_range(min = N, max = M)`: 长度范围验证
- `date_format = Format`: 日期格式验证
- `min = N`: 最小值验证
- `max = N`: 最大值验证
- `desc = "描述"`: 字段描述

### 其他验证属性

- `nested`: 嵌套结构体验证（用于标记需要递归验证的结构体字段）

## 使用示例

```rust
use common_validation::{Validatable, DateTimeFormatEnum};
use common_validation_macros::ValidatableImpl;

#[derive(ValidatableImpl)]
struct User {
    #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
    username: String,
    
    #[validate(not_null, date_format = Year, desc = "生日")]
    birthday: String,
}

let user = User {
    username: "test".to_string(),
    birthday: "1990-01-01".to_string(),
}

assert!(user.validate().is_ok());
```

## 验证调用

生成的代码会为结构体实现 `Validatable` trait，可以通过调用 `validate()` 方法来执行验证：

```rust
match user.validate() {
    Ok(()) => println!("验证通过"),
    Err(e) => println!("验证失败: {}", e),
}
```