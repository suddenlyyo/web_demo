//! # 验证规则枚举
//!
//! 定义支持的各种验证规则类型

use crate::DateTimeFormatEnum;

// 验证规则枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationRulesEnum {
    /// 不允许为空
    NotNull,
    /// 固定长度，len：指定长度，如 5 表示长度为5
    Length(usize),
    /// 长度区间，min/max：最小/最大长度，如 min=5, max=20 表示5~20之间
    LengthRange(usize, usize),
    /// 存在时验证固定长度，len：指定长度
    ExistLength(usize),
    /// 存在时验证长度区间，min/max：最小/最大长度，如 min=5, max=20 表示5~20之间
    ExistLengthRange(usize, usize),
    /// 指定格式的日期时间，format：日期格式枚举
    DateFormat(DateTimeFormatEnum),
    /// 数字最小值，min：最小值
    NumberMin(i64),
    /// 数字最大值，max：最大值
    NumberMax(i64),
    /// 用于判断是否递归校验自定义类型，使用此枚举值表示需要递归校验
    Structure,
}
