//! # Common Validation 通用验证库
//!
//! 一个用于参数验证的通用库，提供各种验证规则和验证器实现。
//!
//! ## 功能特性
//!
//! - 多种验证规则（非空、长度、日期格式、数值范围等）
//! - 灵活的验证规则组合
//! - 自定义错误类型
//! - 易于使用的验证器接口
//!
//! ## 使用示例
//!
//! ```rust
//! use common_validation::{ValidationRule, ValidationRulesEnum, DateTimeFormatEnum, ParameterValidator};
//!
//! let rule = ValidationRule::new("用户名")
//!     .not_null()
//!     .length_range(3, 20);
//!
//! let result = ParameterValidator::validate_value("test_user", &rule);
//! assert!(result.is_ok());
//! ```

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::str::FromStr;

// ====================== 验证规则配置 ======================

/// 参数验证规则配置
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// 参数描述
    pub desc: String,
    /// 验证规则列表
    pub rules: Vec<ValidationRulesEnum>,
}

impl ValidationRule {
    /// 创建新的验证规则
    ///
    /// # 参数
    ///
    /// * `desc` - 参数描述信息
    ///
    /// # 返回值
    ///
    /// 返回一个新的ValidationRule实例
    pub fn new(desc: &str) -> Self {
        ValidationRule { desc: desc.to_string(), rules: Vec::new() }
    }

    /// 添加验证规则
    ///
    /// # 参数
    ///
    /// * `rule` - 要添加的验证规则
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn with_rule(mut self, rule: ValidationRulesEnum) -> Self {
        self.rules.push(rule);
        self
    }

    /// 便捷方法：添加非空验证
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn not_null(self) -> Self {
        self.with_rule(ValidationRulesEnum::NotNull)
    }

    /// 便捷方法：添加固定长度验证
    ///
    /// # 参数
    ///
    /// * `len` - 指定的长度
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn length(self, len: usize) -> Self {
        self.with_rule(ValidationRulesEnum::Length(len))
    }

    /// 便捷方法：添加长度范围验证
    ///
    /// # 参数
    ///
    /// * `min` - 最小长度
    /// * `max` - 最大长度
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn length_range(self, min: usize, max: usize) -> Self {
        self.with_rule(ValidationRulesEnum::LengthRange(min, max))
    }

    /// 便捷方法：添加可选长度验证
    ///
    /// # 参数
    ///
    /// * `len` - 指定的长度
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn exist_length(self, len: usize) -> Self {
        self.with_rule(ValidationRulesEnum::ExistLength(len))
    }

    /// 便捷方法：添加可选长度范围验证
    ///
    /// # 参数
    ///
    /// * `min` - 最小长度
    /// * `max` - 最大长度
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn exist_length_range(self, min: usize, max: usize) -> Self {
        self.with_rule(ValidationRulesEnum::ExistLengthRange(min, max))
    }

    /// 便捷方法：添加日期格式验证
    ///
    /// # 参数
    ///
    /// * `format` - 日期时间格式枚举
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn date_format(self, format: DateTimeFormatEnum) -> Self {
        self.with_rule(ValidationRulesEnum::DateFormat(format))
    }

    /// 便捷方法：添加最小值验证
    ///
    /// # 参数
    ///
    /// * `min` - 最小值
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn min(self, min: i64) -> Self {
        self.with_rule(ValidationRulesEnum::NumberMin(min))
    }

    /// 便捷方法：添加最大值验证
    ///
    /// # 参数
    ///
    /// * `max` - 最大值
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn max(self, max: i64) -> Self {
        self.with_rule(ValidationRulesEnum::NumberMax(max))
    }

    /// 便捷方法：添加数值范围验证
    ///
    /// # 参数
    ///
    /// * `min` - 最小值
    /// * `max` - 最大值
    ///
    /// # 返回值
    ///
    /// 返回Self以支持链式调用
    pub fn number_range(self, min: i64, max: i64) -> Self {
        self.with_rule(ValidationRulesEnum::NumberMin(min))
            .with_rule(ValidationRulesEnum::NumberMax(max))
    }
}

// ====================== 验证器实现 ======================
/// 参数验证器
///
/// 提供静态方法用于验证字符串值是否符合指定规则
pub struct ParameterValidator;

impl ParameterValidator {
    /// 验证值是否符合规则
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的字符串值
    /// * `rule` - 验证规则
    ///
    /// # 返回值
    ///
    /// 如果验证通过返回Ok(())，否则返回相应的验证错误
    pub fn validate_value(value: &str, rule: &ValidationRule) -> Result<(), ValidationErrorEnum> {
        for &rule_type in &rule.rules {
            match rule_type {
                ValidationRulesEnum::NotNull => Self::validate_not_null(value, &rule.desc)?,
                ValidationRulesEnum::Length(len) => Self::validate_length(value, len, &rule.desc)?,
                ValidationRulesEnum::LengthRange(min, max) => Self::validate_length_range(value, min, max, &rule.desc)?,
                ValidationRulesEnum::ExistLength(len) => Self::validate_exist_length(value, len, &rule.desc)?,
                ValidationRulesEnum::ExistLengthRange(min, max) => Self::validate_exist_length_range(value, min, max, &rule.desc)?,
                ValidationRulesEnum::DateFormat(format) => Self::validate_datetime(value, format, &rule.desc)?,
                ValidationRulesEnum::NumberMin(min) => Self::validate_number_min(value, min, &rule.desc)?,
                ValidationRulesEnum::NumberMax(max) => Self::validate_number_max(value, max, &rule.desc)?,
                ValidationRulesEnum::Nested => {
                    // Nested规则不在此处处理，它应该在宏生成的代码中通过直接调用validate()处理
                    // 此处保留空处理以避免编译错误
                },
            }
        }
        Ok(())
    }

    // =============== 具体验证方法 ===============
    /// 验证非空
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::NotNull(desc) 如果值为空或为 "null" 或 "undefined"
    ///
    /// # 注意
    ///
    /// 这里的 "null" 和 "undefined" 是字符串形式的检查
    /// 例如: validate_not_null("undefined", "用户名") 返回 Err(ValidationErrorEnum::NotNull("用户名".to_string()))
    fn validate_not_null(value: &str, desc: &str) -> Result<(), ValidationErrorEnum> {
        if value.trim().is_empty() || value.eq_ignore_ascii_case("null") || value.eq_ignore_ascii_case("undefined") {
            Err(ValidationErrorEnum::NotNull(desc.to_string()))
        } else {
            Ok(())
        }
    }

    /// 验证存在时的固定长度（只有当值不为空时才验证长度）
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `expected_len` - 期望的长度
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::Length(desc, format!("必须为 {expected_len} 个字符")) 如果长度不符合预期
    fn validate_exist_length(value: &str, expected_len: usize, desc: &str) -> Result<(), ValidationErrorEnum> {
        if !value.is_empty() { Self::validate_length(value, expected_len, desc) } else { Ok(()) }
    }

    /// 验证存在时的长度范围（只有当值不为空时才验证长度范围）
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `min` - 最小长度
    /// * `max` - 最大长度
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::Length(desc, format!("必须在 {min}~{max} 个字符之间"))
    fn validate_exist_length_range(value: &str, min: usize, max: usize, desc: &str) -> Result<(), ValidationErrorEnum> {
        if !value.is_empty() { Self::validate_length_range(value, min, max, desc) } else { Ok(()) }
    }

    /// 验证固定长度
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `expected_len` - 期望的长度
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::Length(desc, format!("必须为 {expected_len} 个字符")) 如果长度不符合预期
    ///
    /// # 示例
    ///
    /// validate_length("abc", 3, "用户名") 返回 Ok(())
    ///
    /// # 注意
    ///
    /// 如果 value 的长度不等于 expected_len，则返回错误
    fn validate_length(value: &str, expected_len: usize, desc: &str) -> Result<(), ValidationErrorEnum> {
        let value_len = value.len();
        if value_len != expected_len {
            return Err(ValidationErrorEnum::Length(desc.to_string(), format!("必须为 {expected_len} 个字符")));
        }
        Ok(())
    }

    /// 验证长度范围
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `min` - 最小长度
    /// * `max` - 最大长度
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::Length(desc, format!("必须在 {min}~{max} 个字符之间"))
    fn validate_length_range(value: &str, min: usize, max: usize, desc: &str) -> Result<(), ValidationErrorEnum> {
        let value_len = value.len();
        if value_len < min || value_len > max {
            return Err(ValidationErrorEnum::Length(desc.to_string(), format!("必须在 {min}~{max} 个字符之间")));
        }
        Ok(())
    }

    /// 验证日期时间格式
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `format` - 日期时间格式枚举
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::Format(desc) 如果格式不正确
    fn validate_datetime(value: &str, format: DateTimeFormatEnum, desc: &str) -> Result<(), ValidationErrorEnum> {
        match format {
            DateTimeFormatEnum::Time => {
                NaiveTime::parse_from_str(value, format.pattern().unwrap()).map_err(|_| ValidationErrorEnum::Format(desc.to_string()))?;
            },
            DateTimeFormatEnum::Year | DateTimeFormatEnum::YearNoSplit => {
                NaiveDate::parse_from_str(value, format.pattern().unwrap()).map_err(|_| ValidationErrorEnum::Format(desc.to_string()))?;
            },
            _ => {
                NaiveDateTime::parse_from_str(value, format.pattern().unwrap()).map_err(|_| ValidationErrorEnum::Format(desc.to_string()))?;
            },
        }
        Ok(())
    }

    /// 验证数值最小值
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `min` - 最小值
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::NumberMin(desc, min) 如果数值小于最小值
    /// ValidationErrorEnum::NumberFormatError 如果不是有效数字
    fn validate_number_min(value: &str, min: i64, desc: &str) -> Result<(), ValidationErrorEnum> {
        let num = i64::from_str(value).map_err(|_| ValidationErrorEnum::NumberFormatError)?;

        if num < min {
            return Err(ValidationErrorEnum::NumberMin(desc.to_string(), min));
        }
        Ok(())
    }

    /// 验证数值最大值
    ///
    /// # 参数
    ///
    /// * `value` - 要验证的值
    /// * `max` - 最大值
    /// * `desc` - 参数描述
    ///
    /// # 返回值
    ///
    /// Ok(()) 如果验证通过，否则返回 ValidationErrorEnum
    ///
    /// # 错误
    ///
    /// ValidationErrorEnum::NumberMax(desc, max) 如果数值大于最大值
    /// ValidationErrorEnum::NumberFormatError 如果不是有效数字
    fn validate_number_max(value: &str, max: i64, desc: &str) -> Result<(), ValidationErrorEnum> {
        let num = i64::from_str(value).map_err(|_| ValidationErrorEnum::NumberFormatError)?;

        if num > max {
            return Err(ValidationErrorEnum::NumberMax(desc.to_string(), max));
        }
        Ok(())
    }
}

// ====================== 结构体验证接口 ======================
/// 可验证 trait
///
/// 为需要验证的结构体提供统一的验证接口
pub trait Validatable {
    /// 验证结构体
    ///
    /// # 返回值
    ///
    /// 如果验证通过返回Ok(())，否则返回相应的验证错误
    fn validate(&self) -> Result<(), ValidationErrorEnum>;
}

mod enums;
// ====================== 模块导出 ======================
pub use enums::*;
