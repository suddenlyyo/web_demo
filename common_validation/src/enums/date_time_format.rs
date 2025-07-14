// 日期格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeFormatEnum {
    Time,
    DateTime,
    DateTimeStr,
    Year,
    YearNoSplit,
    DateTimePattern,
}

impl DateTimeFormatEnum {
    /// 获取日期时间格式的字符串模式
    pub const fn pattern(&self) -> Option<&'static str> {
        match self {
            DateTimeFormatEnum::Time => Some("%H:%M"),
            DateTimeFormatEnum::DateTime => Some("%Y-%m-%d %H:%M:%S"),
            DateTimeFormatEnum::DateTimeStr => Some("%Y%m%d%H%M%S"),
            DateTimeFormatEnum::Year => Some("%Y-%m-%d"),
            DateTimeFormatEnum::YearNoSplit => Some("%Y%m%d"),
            DateTimeFormatEnum::DateTimePattern => Some("%H%M%S"),
        }
    }
}
