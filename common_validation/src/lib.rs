mod enums;
// 重新导出
pub use enums::*;

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



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::enums::{DateTimeFormatEnum, ValidateRulesEnum, ValidationErrorEnum};
//     use chrono::NaiveDate;

   
// }
