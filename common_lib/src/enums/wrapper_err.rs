#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WrapperErrEnum {
    Success,
    Fail,
    UnknownError,
}

impl WrapperErr {
    // 获取固定错误码
    pub const fn code(&self) -> i32 {
        match self {
            Self::Success => 1,
            Self::Fail => -1,
            Self::UnknownError => -2,
        }
    }

    // 获取固定错误信息（直接返回静态字符串）
    pub const fn message(&self) -> &'static str {
        match self {
            Self::Success => "成功",
            Self::Fail => "失败",
            Self::UnknownError => "未知错误",
        }
    }

    // 按错误码查找枚举值
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            1 => Some(Self::Success),
            -1 => Some(Self::Fail),
            -2 => Some(Self::UnknownError),
            _ => None,
        }
    }
}