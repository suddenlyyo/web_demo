use crate::DateTimeFormatEnum;

// 验证规则枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationRulesEnum {
    /// 不允许为空
    NotNull,
    /// 固定长度，如 "5" 表示长度为5
    Length(usize),
    /// 长度区间，如 "5~20" 表示5到20之间
    LengthRange(usize, usize),
    /// 存在时验证固定长度
    ExistLength(usize),
    /// 存在时验证长度区间
    ExistLengthRange(usize, usize),
    /// 指定格式的日期时间
    DateFormat(DateTimeFormatEnum),
    /// 数字最小值
    NumberMin(i64),
    /// 数字最大值    
    NumberMax(i64),
}
