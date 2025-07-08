use thiserror::Error;
// 错误类型定义
#[derive(Debug, PartialEq, Eq, Hash ,Error)]
pub enum ValidationErrorEnum {
    #[error("{0} 不能为空")]
    NotNone(String),
    #[error("{0} 长度不符合要求: {1}")]
    Length(String, String),
    #[error("{0} 格式不正确")]
    Format(String),
    #[error("{0} 值不能小于 {1}")]
    NumberMin(String, i64),
    #[error("{0} 值不能大于 {1}")]
    NumberMax(String, i64),
}
