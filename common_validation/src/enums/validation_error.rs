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
    /// 类型不支持错误，第一个参数为类型名称，第二个参数为规则名称
    #[error("类型 {0} 不支持验证规则 {1}")]
    UnsupportedType(String, String),
    /// 必须为正数
    #[error("{0} 必须为正数")]
    PositiveNumber(String),
    /// 必须为非负数
    #[error("{0} 必须为非负数")]
    NonNegativeNumber(String),
    /// 必须为整数
    #[error("{0} 必须为整数")]
    Integer(String),
    /// 小数位数超出限制
    #[error("{0} 小数位数不能超过 {1} 位")]
    DecimalScale(String, u32),
    /// 必须为奇数
    #[error("{0} 必须为奇数")]
    OddNumber(String),
    /// 必须为偶数
    #[error("{0} 必须为偶数")]
    EvenNumber(String),
    /// 必须为指定数字的倍数
    #[error("{0} 必须为 {1} 的倍数")]
    MultipleOf(String, i64),
}
