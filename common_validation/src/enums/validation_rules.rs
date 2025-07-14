// 验证规则枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationRulesEnum {
    /// 验证不允许为空
    NotNone,
    /// 验证指定长度
    Length,
    /// 如果存在则验证长度，否则不验证
    ExistLength,
    /// 验证指定格式的Date
    Date,
    /// 验证指定格式的Time
    Time,
    /// 验证指定格式的DateTime
    DateTime,
    /// 验证数字的最小值
    NumberMin,
    /// 验证数字的最大值
    NumberMax,
    /// 验证整个结构体
    Structure,
}
