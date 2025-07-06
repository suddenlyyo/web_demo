/// 错误枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperErrEnum {
    /// 成功
    Success = 1,
    /// 失败
    Fail = -1,
    /// 未知错误
    UnknownError = -2,
}

impl WrapperErrEnum {
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
}

impl From<i32> for WrapperErrEnum {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::Success,
            1 => Self::Fail,
            2 => Self::UnknownError,
            // 处理非法值
            _ => Self::UnknownError,
        }
    }
}
