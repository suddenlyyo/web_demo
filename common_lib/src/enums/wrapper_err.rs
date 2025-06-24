use std::borrow::Cow;
/// # 错误枚举
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WrapperErr {
    Success(ErrorInfo),
    Fail(ErrorInfo),
    UnknownError(ErrorInfo),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ErrorInfo {
    code: Option<i32>,
    message: Option<Cow<'static, str>>, // 使用Cow智能指针
}

impl ErrorInfo {
    pub fn new<S: Into<Cow<'static, str>>>(code: Option<i32>, message: Option<S>) -> Self {
        ErrorInfo {
            code,
            message: message.map(Into::into),
        }
    }
}

impl WrapperErr {
    pub fn success<S: Into<Cow<'static, str>>>(code: Option<i32>, msg: Option<S>) -> Self {
        WrapperErr::Success(ErrorInfo::new(code, msg))
    }

    // 默认构造函数
    pub fn success_default() -> Self {
        WrapperErr::Success(ErrorInfo::new(Some(1), Some("success")))
    }

    pub fn fail<S: Into<Cow<'static, str>>>(code: Option<i32>, msg: Option<S>) -> Self {
        WrapperErr::Fail(ErrorInfo::new(code, msg))
    }

    pub fn fail_default() -> Self {
        WrapperErr::Fail(ErrorInfo::new(Some(-1), Some("fail")))
    }

    pub fn unknown_error<S: Into<Cow<'static, str>>>(code: Option<i32>, msg: Option<S>) -> Self {
        WrapperErr::UnknownError(ErrorInfo::new(code, msg))
    }

    pub fn unknown_error_default() -> Self {
        WrapperErr::UnknownError(ErrorInfo::new(Some(-2), Some("unknown error")))
    }
    pub fn get_code(&self) -> i32 {
        match self {
            Self::Success(info) => info.code.unwrap_or(1),
            Self::Fail(info) => info.code.unwrap_or(-1),
            Self::UnknownError(info) => info.code.unwrap_or(-2),
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            Self::Success(info) => info.message.as_deref().unwrap_or("success"),
            Self::Fail(info) => info.message.as_deref().unwrap_or("fail"),
            Self::UnknownError(info) => info.message.as_deref().unwrap_or("unknown error"),
        }
    }
}
