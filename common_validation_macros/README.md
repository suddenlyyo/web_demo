# Common Validation Macros

提供用于自动生成验证代码的派生宏。

## 功能特性

- 为结构体自动生成 `Validatable` 实现
- 支持多种验证属性（not_null、length、date_format、min/max等）
- 支持嵌套结构体验证
- 支持Option和Vec类型的验证
- 支持数值类型验证（i32, f64等）
- 支持自定义验证消息

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
- 可扩展的验证器接口

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
    
    #[validate(min = 0, max = 150, desc = "年龄")]
    age: u8,
    
    #[validate(nested, desc = "地址信息")]
    address: Option<Address>,
    
    #[validate(length_range(min = 1, max = 5), desc = "电话号码")]
    phone_numbers: Vec<String>,
}
```

## 支持的验证属性

### 基本验证属性

- `not_null`: 非空验证
- `length = N`: 固定长度验证
- `length_range(min = N, max = M)`: 长度范围验证
- `exist_length = N`: 存在时的固定长度验证
- `exist_length_range(min = N, max = M)`: 存在时的长度范围验证
- `desc = "描述"`: 字段描述

### 日期验证属性

- `date_format = Format`: 日期格式验证

### 数值验证属性

- `min = N`: 最小值验证
- `max = N`: 最大值验证
- `number_min = N`: 数值最小值验证
- `number_max = N`: 数值最大值验证
- `positive_number`: 正数验证
- `non_negative_number`: 非负数验证
- `integer`: 整数验证
- `decimal_scale = N`: 小数位数验证
- `odd_number`: 奇数验证
- `even_number`: 偶数验证
- `multiple_of = N`: 倍数验证

### 高级验证属性

- `nested`: 嵌套结构体验证（用于标记需要递归验证的结构体字段）
- `custom = "function_name"`: 自定义验证函数

## 使用示例

### 基本示例

```rust
use common_validation::{Validatable, DateTimeFormatEnum};
use common_validation_macros::ValidatableImpl;

#[derive(ValidatableImpl)]
struct User {
    #[validate(not_null, length_range(min = 3, max = 20), desc = "用户名")]
    username: String,
    
    #[validate(not_null, date_format = Year, desc = "生日")]
    birthday: String,
    
    #[validate(min = 0, max = 150, desc = "年龄")]
    age: u8,
    
    #[validate(length_range(min = 1, max = 5), desc = "电话号码")]
    phone_numbers: Vec<String>,
}

let user = User {
    username: "test".to_string(),
    birthday: "1990-01-01".to_string(),
    age: 30,
    phone_numbers: vec!["13800138000".to_string()],
};

match user.validate() {
    Ok(()) => println!("验证通过"),
    Err(e) => println!("验证失败: {}", e),
}
```

### 高级验证示例

```rust
use common_validation::{Validatable, DateTimeFormatEnum};
use common_validation_macros::ValidatableImpl;

#[derive(ValidatableImpl)]
struct Product {
    #[validate(not_null, length_range(min = 1, max = 100), desc = "产品名称")]
    name: String,
    
    #[validate(not_null, positive_number, desc = "产品价格")]
    price: f64,
    
    #[validate(not_null, decimal_scale = 2, desc = "折扣")]
    discount: f64,
    
    #[validate(not_null, integer, min = 1, max = 1000, desc = "库存数量")]
    stock: i32,
    
    #[validate(not_null, even_number, desc = "包装数量")]
    package_quantity: i32,
    
    #[validate(not_null, multiple_of = 5, desc = "最小起订量")]
    min_order_quantity: i32,
}

let product = Product {
    name: "测试产品".to_string(),
    price: 99.99,
    discount: 0.8,
    stock: 100,
    package_quantity: 10,
    min_order_quantity: 5,
};

match product.validate() {
    Ok(()) => println!("验证通过"),
    Err(e) => println!("验证失败: {}", e),
}
```

### 分组验证示例

```rust
use common_validation::{Validatable, DateTimeFormatEnum, CreateGroup, UpdateGroup};
use common_validation_macros::ValidatableImpl;

#[derive(ValidatableImpl)]
#[group_fields(
    create = ["name", "price", "stock"],
    update = ["id", "name", "price"]
)]
struct Product {
    #[validate(not_null, desc = "产品ID")]
    id: String,
    
    #[validate(not_null, length_range(min = 1, max = 100), desc = "产品名称")]
    name: String,
    
    #[validate(not_null, positive_number, desc = "产品价格")]
    price: f64,
    
    #[validate(not_null, non_negative_number, desc = "库存数量")]
    stock: i32,
}

let product = Product {
    id: "1".to_string(),
    name: "测试产品".to_string(),
    price: 99.99,
    stock: 100,
};

// 完整验证
match product.validate() {
    Ok(()) => println!("完整验证通过"),
    Err(e) => println!("完整验证失败: {}", e),
}

// 创建分组验证（只验证create组中的字段）
match product.validate_with_group::<CreateGroup>() {
    Ok(()) => println!("创建验证通过"),
    Err(e) => println!("创建验证失败: {}", e),
}

// 更新分组验证（只验证update组中的字段）
match product.validate_with_group::<UpdateGroup>() {
    Ok(()) => println!("更新验证通过"),
    Err(e) => println!("更新验证失败: {}", e),
}
```