// 日期格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeFormatEnum {
    /// 仅时间格式: %H:%M (例如 "14:30")
    Time,
    /// 完整日期时间格式: %Y-%m-%d %H:%M:%S (例如 "2023-01-15 14:30:00")
    DateTime,
    /// 紧凑型日期时间格式: %Y%m%d%H%M%S (例如 "20230115143000")
    DateTimeStr,
    /// 年-月-日格式: %Y-%m-%d (例如 "2023-01-15")
    Year,
    /// 无分隔符日期格式: %Y%m%d (例如 "20230115")
    YearNoSplit,
    /// 紧凑型时间格式: %H%M%S (例如 "143000")
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
