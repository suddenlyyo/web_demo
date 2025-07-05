// 验证规则枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidateRules {
    NotNull,
    Length,
    NullLength,
    Date,
    Time,
    DateTime,
    NumberMin,
    NumberMax,
    Object,
}