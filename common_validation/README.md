# Common Validation

一个用于参数验证的通用库，提供各种验证规则和验证器实现。

## 功能特性

- 多种验证规则（非空、长度、日期格式、数值范围等）
- 灵活的验证规则组合
- 自定义错误类型
- 易于使用的验证器接口

## 设计理念

### 1. 职责分离
本库采用职责分离的设计原则，将验证逻辑与代码生成完全分离：
- **验证器（ParameterValidator）**：负责执行具体的验证逻辑
- **验证trait（Validatable）**：为结构体提供统一的验证接口
- **宏（在common_validation_macros中）**：负责生成验证代码

这种设计使得各部分职责明确，便于维护和扩展。

### 2. 类型安全
充分利用Rust的类型系统确保验证过程的安全性：
- 通过枚举定义验证规则，避免字符串类型的规则定义错误
- 使用Result类型处理验证结果，强制调用者处理验证失败的情况
- 编译时检查确保类型安全

### 3. 易用性
在保证类型安全的前提下，提供简单易用的接口：
- 链式调用的验证规则定义
- 清晰的错误信息
- 统一的验证接口

### 4. 可扩展性
设计上考虑了可扩展性：
- 可以轻松添加新的验证规则
- 支持自定义错误类型
- 可以手动实现Validatable trait以处理复杂验证逻辑

### 5. 专注Web API场景
专门针对HTTP请求中的JSON数据验证设计：
- 主要处理字符串和数字类型验证
- 支持嵌套结构体验证
- 符合Web开发的常见需求

## 主要组件

### 验证规则 (ValidationRule)
用于定义字段的验证规则，支持链式调用：

```rust
use common_validation::{ValidationRule, ValidationRulesEnum, DateTimeFormatEnum};

let rule = ValidationRule::new("用户名")
    .not_null()
    .length_range(3, 20);
```

### 验证器 (ParameterValidator)
提供静态方法用于验证字符串值是否符合指定规则：

```rust
use common_validation::ParameterValidator;

let result = ParameterValidator::validate_value("test_user", &rule);
```

### 可验证 trait (Validatable)
为需要验证的结构体提供统一的验证接口：

```rust
use common_validation::Validatable;

impl Validatable for MyStruct {
    fn validate(&self) -> Result<(), ValidationErrorEnum> {
        // 验证逻辑
    }
}
```

## 支持的验证规则

- `not_null`: 非空验证
- `length`: 固定长度验证
- `length_range`: 长度范围验证
- `exist_length`: 存在时的固定长度验证
- `exist_length_range`: 存在时的长度范围验证
- `date_format`: 日期格式验证
- `min`/`max`: 数值范围验证
- `number_min`/`number_max`: 数值最小/最大值验证

## 错误类型

- `NotNull`: 字段不能为空
- `Length`: 字段长度不符合要求
- `Format`: 字段格式不正确
- `NumberMin`: 数值不能小于指定最小值
- `NumberMax`: 数值不能大于指定最大值
- `LengthRangeError`: 长度区间设置错误
- `DateTimeFormatNotSet`: 日期时间格式未设置
- `NumberFormatError`: 数字格式错误
- `UnsupportedType`: 类型不支持错误

## 使用示例

```rust
use common_validation::{ValidationRule, ValidationRulesEnum, DateTimeFormatEnum, ParameterValidator};

let rule = ValidationRule::new("用户名")
    .not_null()
    .length_range(3, 20);

let result = ParameterValidator::validate_value("test_user", &rule);
assert!(result.is_ok());
```