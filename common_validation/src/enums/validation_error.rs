//! # 验证错误枚举
//!
//! 定义验证过程中可能发生的各种错误类型

use thiserror::Error;

// 错误类型定义：用于表单或数据校验时的各种错误情况
#[derive(Debug, PartialEq, Eq, Hash, Error)]
pub enum ValidationErrorEnum {
    /// 字段不能为空
    #[error("{0} 不能为空")]
    NotNull(String),
    /// 字段长度不符合要求，第二个参数为具体说明
    #[error("{0} 长度不符合要求: {1}")]
    Length(String, String),
    /// 字段格式不正确
    #[error("{0} 格式不正确")]
    Format(String),
    /// 数值不能小于指定最小值
    #[error("{0} 值不能小于 {1}")]
    NumberMin(String, i64),
    /// 数值不能大于指定最大值
    #[error("{0} 值不能大于 {1}")]
    NumberMax(String, i64),
    /// 长度区间设置本身有误
    #[error("长度区间设置错误: {0}")]
    LengthRangeError(String),
    /// 日期时间格式未设置
    #[error("日期时间格式未设置")]
    DateTimeFormatNotSet,
    /// 数字格式错误
    #[error("数字格式错误")]
    NumberFormatError,
}
