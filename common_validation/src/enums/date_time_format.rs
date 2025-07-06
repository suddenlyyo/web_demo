// 日期格式枚举
#[derive(Debug, Clone, Copy)]
pub enum DateTimeFormat {
    Time,
    DateTime,
    DateTimeStr,
    Year,
    YearNoSplit,
    DateTimePattern,
}

impl DateTimeFormat {
    /// 获取日期时间格式的字符串模式
    pub const fn pattern(&self) -> &'static str {
        match self {
            DateTimeFormat::Time => "%H:%M",
            DateTimeFormat::DateTime => "%Y-%m-%d %H:%M:%S",
            DateTimeFormat::DateTimeStr => "%Y%m%d%H%M%S",
            DateTimeFormat::Year => "%Y-%m-%d",
            DateTimeFormat::YearNoSplit => "%Y%m%d",
            DateTimeFormat::DateTimePattern => "%H%M%S",
        }
    }
}
