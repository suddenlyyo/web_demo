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
    pub fn new(desc: &str) -> Self {
        ValidationRule { desc: desc.to_string(), rules: Vec::new() }
    }

    /// 添加验证规则
    pub fn with_rule(mut self, rule: ValidationRulesEnum) -> Self {
        self.rules.push(rule);
        self
    }

    /// 便捷方法：添加非空验证
    pub fn not_null(self) -> Self {
        self.with_rule(ValidationRulesEnum::NotNull)
    }

    /// 便捷方法：添加固定长度验证
    pub fn length(self, len: usize) -> Self {
        self.with_rule(ValidationRulesEnum::Length(len))
    }

    /// 便捷方法：添加长度范围验证
    pub fn length_range(self, min: usize, max: usize) -> Self {
        self.with_rule(ValidationRulesEnum::LengthRange(min, max))
    }

    /// 便捷方法：添加可选长度验证
    pub fn exist_length(self, len: usize) -> Self {
        self.with_rule(ValidationRulesEnum::ExistLength(len))
    }

    /// 便捷方法：添加可选长度范围验证
    pub fn exist_length_range(self, min: usize, max: usize) -> Self {
        self.with_rule(ValidationRulesEnum::ExistLengthRange(min, max))
    }

    /// 便捷方法：添加日期格式验证
    pub fn date_format(self, format: DateTimeFormatEnum) -> Self {
        self.with_rule(ValidationRulesEnum::DateFormat(format))
    }

    /// 便捷方法：添加最小值验证
    pub fn min(self, min: i64) -> Self {
        self.with_rule(ValidationRulesEnum::NumberMin(min))
    }

    /// 便捷方法：添加最大值验证
    pub fn max(self, max: i64) -> Self {
        self.with_rule(ValidationRulesEnum::NumberMax(max))
    }

    /// 便捷方法：添加数值范围验证
    pub fn number_range(self, min: i64, max: i64) -> Self {
        self.with_rule(ValidationRulesEnum::NumberMin(min))
            .with_rule(ValidationRulesEnum::NumberMax(max))
    }
}

// ====================== 验证器实现 ======================
pub struct ParameterValidator;

impl ParameterValidator {
    pub fn validate_value(value: &str, rule: &ValidationRule) -> Result<(), ValidationErrorEnum> {
        for &rule_type in &rule.rules {
            match rule_type {
                ValidationRulesEnum::NotNull => Self::validate_not_null(value, &rule.desc)?,
                ValidationRulesEnum::Length(len) => Self::validate_length(value, len, &rule.desc)?,
                ValidationRulesEnum::LengthRange(min, max) => Self::validate_length_range(value, min, max, &rule.desc)?,
                ValidationRulesEnum::ExistLength(len) => {
                    if !value.is_empty() {
                        Self::validate_length(value, len, &rule.desc)?
                    }
                },
                ValidationRulesEnum::ExistLengthRange(min, max) => {
                    if !value.is_empty() {
                        Self::validate_length_range(value, min, max, &rule.desc)?
                    }
                },
                ValidationRulesEnum::DateFormat(format) => Self::validate_datetime(value, format, &rule.desc)?,
                ValidationRulesEnum::NumberMin(min) => Self::validate_number_min(value, min, &rule.desc)?,
                ValidationRulesEnum::NumberMax(max) => Self::validate_number_max(value, max, &rule.desc)?,
            }
        }
        Ok(())
    }

    // =============== 具体验证方法 ===============
    fn validate_not_null(value: &str, desc: &str) -> Result<(), ValidationErrorEnum> {
        if value.trim().is_empty() || value.eq_ignore_ascii_case("null") || value.eq_ignore_ascii_case("undefined") {
            Err(ValidationErrorEnum::NotNull(desc.to_string()))
        } else {
            Ok(())
        }
    }

    fn validate_length(value: &str, expected_len: usize, desc: &str) -> Result<(), ValidationErrorEnum> {
        let value_len = value.len();
        if value_len != expected_len {
            return Err(ValidationErrorEnum::Length(desc.to_string(), format!("必须为 {} 个字符", expected_len)));
        }
        Ok(())
    }

    fn validate_length_range(value: &str, min: usize, max: usize, desc: &str) -> Result<(), ValidationErrorEnum> {
        let value_len = value.len();
        if value_len < min || value_len > max {
            return Err(ValidationErrorEnum::Length(desc.to_string(), format!("必须在 {}~{} 个字符之间", min, max)));
        }
        Ok(())
    }

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

    fn validate_number_min(value: &str, min: i64, desc: &str) -> Result<(), ValidationErrorEnum> {
        let num = i64::from_str(value).map_err(|_| ValidationErrorEnum::NumberFormatError)?;

        if num < min {
            return Err(ValidationErrorEnum::NumberMin(desc.to_string(), min));
        }
        Ok(())
    }

    fn validate_number_max(value: &str, max: i64, desc: &str) -> Result<(), ValidationErrorEnum> {
        let num = i64::from_str(value).map_err(|_| ValidationErrorEnum::NumberFormatError)?;

        if num > max {
            return Err(ValidationErrorEnum::NumberMax(desc.to_string(), max));
        }
        Ok(())
    }
}

// ====================== 结构体验证接口 ======================
pub trait Validatable {
    fn validate(&self) -> Result<(), ValidationErrorEnum>;
}

mod enums;
// ====================== 模块导出 ======================
pub use enums::*;
