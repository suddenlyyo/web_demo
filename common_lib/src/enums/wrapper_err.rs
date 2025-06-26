/// 错误枚举
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WrapperErrEnum {
    Success = 1,
    Fail = -1,
    UnknownError = -2,
}

impl WrapperErrEnum {
    // 获取固定错误码
    pub const fn code(&self) -> i32 {
        match self {
            Self::Success => Self::Success as i32,
            Self::Fail => Self::Fail as i32,
            Self::UnknownError => Self::UnknownError as i32,
        }
    }

    // pub const  定义公共常量函数
    // 需要编译时计算值时
    // 提供常量上下文中使用的构造函数或工具函数
    // 提供不可变的返回值
    // 获取固定错误信息（直接返回静态字符串）
    pub const fn message(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::Fail => "Fail",
            Self::UnknownError => "Unknown Error",
        }
    }

    // 按错误码查找枚举值
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            _ if code == Self::Success as i32 => Some(Self::Success),
            _ if code == Self::Fail as i32 => Some(Self::Fail),
            _ if code == Self::UnknownError as i32 => Some(Self::UnknownError),
            _ => None,
        }
    }
}
