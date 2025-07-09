use chrono::{NaiveDate, NaiveDateTime, NaiveTime};


mod enums;
// 重新导出
pub use enums::*;

// ====================== 验证规则配置 ======================

/// 参数验证规则配置
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// 参数描述
    pub desc: String,
    /// 验证规则列表
    pub rules: Vec<ValidateRulesEnum>,
    /// 长度规则 (例如 "5", "5~20")
    pub length: Option<String>,
    /// 日期格式
    pub date_format: DateTimeFormatEnum,
    /// 最小值
    pub number_min: Option<i64>,
    /// 最大值
    pub number_max: Option<i64>,
}

impl ValidationRule {
    /// 创建新的验证规则
    pub fn new(desc: &str) -> Self {
        ValidationRule {
            desc: desc.to_string(),
            rules: Vec::new(),
            length: None,
            date_format: DateTimeFormatEnum::None,
            number_min: None,
            number_max: None,
        }
    }

    /// 添加验证规则
    pub fn with_rule(mut self, rule: ValidateRulesEnum) -> Self {
        self.rules.push(rule);
        self
    }

    /// 设置长度规则
    pub fn with_length(mut self, length: &str) -> Self {
        self.length = Some(length.to_string());
        self
    }

    /// 设置日期格式
    pub fn with_date_format(mut self, format: DateTimeFormatEnum) -> Self {
        self.date_format = format;
        self
    }

    /// 设置数字范围
    pub fn with_number_range(mut self, min: Option<i64>, max: Option<i64>) -> Self {
        self.number_min = min;
        self.number_max = max;
        self
    }
}

// ====================== 验证器实现 ======================

/// 参数验证器
pub struct ParameterValidator;

impl ParameterValidator {
    /// 验证单个值
    pub fn validate_value(value: &str, rule: &ValidationRule) -> Result<(), ValidationErrorEnum> {
        for &rule_type in &rule.rules {
            match rule_type {
                ValidateRulesEnum::NotNone => Self::validate_not_none(value, &rule.desc)?,
                ValidateRulesEnum::Length => {
                    if let Some(length) = &rule.length {
                        Self::validate_length(value, length, &rule.desc)?
                    }
                }
                ValidateRulesEnum::ExistLength => {
                    if !value.is_empty() {
                        if let Some(length) = &rule.length {
                            Self::validate_length(value, length, &rule.desc)?
                        }
                    }
                }
                ValidateRulesEnum::Date => {
                    Self::validate_date(value, rule.date_format, &rule.desc)?
                }
                ValidateRulesEnum::Time => {
                    Self::validate_time(value, rule.date_format, &rule.desc)?
                }
                ValidateRulesEnum::DateTime => {
                    Self::validate_datetime(value, rule.date_format, &rule.desc)?
                }
                ValidateRulesEnum::NumberMin => {
                    if let Some(min) = rule.number_min {
                        Self::validate_number_min(value, min, &rule.desc)?
                    }
                }
                ValidateRulesEnum::NumberMax => {
                    if let Some(max) = rule.number_max {
                        Self::validate_number_max(value, max, &rule.desc)?
                    }
                }
                ValidateRulesEnum::Structure => {
                    // 结构体验证需要特殊处理
                    return Err(ValidationErrorEnum::Format(
                        "结构体验证需使用validate_structure方法".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// 验证结构体
    pub fn validate_structure<T: Validatable>(
        value: &T,
        rule: &ValidationRule,
    ) -> Result<(), ValidationErrorEnum> {
        if rule.rules.contains(&ValidateRulesEnum::Structure) {
            value.validate()
        } else {
            Err(ValidationErrorEnum::Format(
                "未启用结构体验证规则".to_string(),
            ))
        }
    }

    // =============== 具体验证方法 ===============

    /// 验证非空
    fn validate_not_none(value: &str, desc: &str) -> Result<(), ValidationErrorEnum> {
        if value.trim().is_empty()
            || value.eq_ignore_ascii_case("null")
            || value.eq_ignore_ascii_case("undefined")
        {
            Err(ValidationErrorEnum::NotNone(desc.to_string()))
        } else {
            Ok(())
        }
    }

    /// 验证长度
    fn validate_length(value: &str, length: &str, desc: &str) -> Result<(), ValidationErrorEnum> {
        let value_len = value.len();

        // 检查是否为单个数字
        if let Ok(expected_len) = length.parse::<usize>() {
            if value_len != expected_len {
                return Err(ValidationErrorEnum::Length(
                    desc.to_string(),
                    format!("长度必须为 {}", expected_len),
                ));
            }
            return Ok(());
        }

        // 检查范围格式 (如 "5~20")
        let parts: Vec<&str> = length.split('~').collect();
        if parts.len() != 2 {
            return Err(ValidationErrorEnum::LengthRangeError(
                "长度格式应为 '长度' 或 '最小~最大'".to_string(),
            ));
        }

        let min = parts[0]
            .parse::<usize>()
            .map_err(|_| ValidationErrorEnum::LengthRangeError("最小长度无效".to_string()))?;

        let max = parts[1]
            .parse::<usize>()
            .map_err(|_| ValidationErrorEnum::LengthRangeError("最大长度无效".to_string()))?;

        if min > max {
            return Err(ValidationErrorEnum::LengthRangeError(
                "最小长度不能大于最大长度".to_string(),
            ));
        }

        if value_len < min || value_len > max {
            return Err(ValidationErrorEnum::Length(
                desc.to_string(),
                format!("长度必须在 {}~{} 之间", min, max),
            ));
        }

        Ok(())
    }

    /// 验证日期
    fn validate_date(
        value: &str,
        format: DateTimeFormatEnum,
        desc: &str,
    ) -> Result<(), ValidationErrorEnum> {
        if format == DateTimeFormatEnum::None {
            return Err(ValidationErrorEnum::DateTimeFormatNotSet);
        }

        NaiveDate::parse_from_str(value, format.pattern())
            .map_err(|_| ValidationErrorEnum::Format(desc.to_string()))?;
        Ok(())
    }

    /// 验证时间
    fn validate_time(
        value: &str,
        format: DateTimeFormatEnum,
        desc: &str,
    ) -> Result<(), ValidationErrorEnum> {
        if format == DateTimeFormatEnum::None {
            return Err(ValidationErrorEnum::DateTimeFormatNotSet);
        }

        NaiveTime::parse_from_str(value, format.pattern())
            .map_err(|_| ValidationErrorEnum::Format(desc.to_string()))?;
        Ok(())
    }

    /// 验证日期时间
    fn validate_datetime(
        value: &str,
        format: DateTimeFormatEnum,
        desc: &str,
    ) -> Result<(), ValidationErrorEnum> {
        if format == DateTimeFormatEnum::None {
            return Err(ValidationErrorEnum::DateTimeFormatNotSet);
        }

        NaiveDateTime::parse_from_str(value, format.pattern())
            .map_err(|_| ValidationErrorEnum::Format(desc.to_string()))?;
        Ok(())
    }

    /// 验证最小值
    fn validate_number_min(value: &str, min: i64, desc: &str) -> Result<(), ValidationErrorEnum> {
        let num = i64::from_str(value).map_err(|_| ValidationErrorEnum::NumberFormatError)?;

        if num < min {
            return Err(ValidationErrorEnum::NumberMin(desc.to_string(), min));
        }
        Ok(())
    }

    /// 验证最大值
    fn validate_number_max(value: &str, max: i64, desc: &str) -> Result<(), ValidationErrorEnum> {
        let num = i64::from_str(value).map_err(|_| ValidationErrorEnum::NumberFormatError)?;

        if num > max {
            return Err(ValidationErrorEnum::NumberMax(desc.to_string(), max));
        }
        Ok(())
    }
}

// ====================== 结构体验证接口 ======================

/// 可验证结构体接口
pub trait Validatable {
    /// 验证结构体
    fn validate(&self) -> Result<(), ValidationErrorEnum>;
}




// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::enums::{DateTimeFormatEnum, ValidateRulesEnum, ValidationErrorEnum};
//     use chrono::NaiveDate;

   
// }
