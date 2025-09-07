//! # 包装器错误枚举
//!
//! 定义统一的响应状态码和消息
//!
//! 该枚举定义了系统中常用的响应状态码及其对应的消息，
//! 用于统一API响应格式。

/// 错误枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WrapperErrEnum {
    /// 成功
    ///
    /// 状态码: 1
    /// 消息: "Success"
    Success = 1,
    /// 失败
    ///
    /// 状态码: -1
    /// 消息: "Fail"
    Fail = -1,
    /// 未知错误
    ///
    /// 状态码: -2
    /// 消息: "Unknown Error"
    UnknownError = -2,
}

impl WrapperErrEnum {
    // pub const  定义公共常量函数
    // 需要编译时计算值时
    // 提供常量上下文中使用的构造函数或工具函数
    // 提供不可变的返回值
    // 获取固定错误信息（直接返回静态字符串）
    /// 获取错误信息
    ///
    /// 获取与枚举值对应的默认错误消息
    ///
    /// # 返回值
    ///
    /// [&'static str] - 与枚举值对应的错误消息
    pub const fn message(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::Fail => "Fail",
            Self::UnknownError => "Unknown Error",
        }
    }
}

/// 实现从i32到WrapperErrEnum的转换
impl From<i32> for WrapperErrEnum {
    /// 从i32转换为WrapperErrEnum
    ///
    /// # 参数
    ///
    /// * `code` - 错误码，类型: [i32]
    ///
    /// # 返回值
    ///
    /// [WrapperErrEnum] - 对应的WrapperErrEnum枚举值，如果找不到则返回UnknownError
    fn from(code: i32) -> Self {
        match code {
            1 => Self::Success,
            -1 => Self::Fail,
            -2 => Self::UnknownError,
            // 处理非法值
            _ => Self::UnknownError,
        }
    }
}
