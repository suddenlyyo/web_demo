// 日期格式枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeFormatEnum {
    Time,
    DateTime,
    DateTimeStr,
    Year,
    YearNoSplit,
    DateTimePattern,
    None,
}

impl DateTimeFormatEnum {
    /// 获取日期时间格式的字符串模式
    pub const fn pattern(&self) -> &'static str {
        match self {
            DateTimeFormatEnum::Time => "%H:%M",
            DateTimeFormatEnum::DateTime => "%Y-%m-%d %H:%M:%S",
            DateTimeFormatEnum::DateTimeStr => "%Y%m%d%H%M%S",
            DateTimeFormatEnum::Year => "%Y-%m-%d",
            DateTimeFormatEnum::YearNoSplit => "%Y%m%d",
            DateTimeFormatEnum::DateTimePattern => "%H%M%S",
            DateTimeFormatEnum::None => "",
        }
    }
}
