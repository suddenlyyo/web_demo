use crate::enums::WrapperErr;
///# 响应包装
#[derive(Debug, serde::Serialize, PartialEq, Eq, Hash)]
pub struct ResponseWrapper {
    code: i32,
    message: String,
}
impl ResponseWrapper {
    // 通用构造函数
    pub fn new<S: Into<String>>(code: i32, message: S) -> Self {
        ResponseWrapper {
            code,
            message: message.into(),
        }
    }

    pub fn get_code(&self) -> i32 {
        self.code
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
    // 默认成功响应
    pub fn success_default() -> Self {
        Self::from(WrapperErr::Success)
    }

    // 默认失败响应
    pub fn fail_default() -> Self {
        Self::from(WrapperErr::Fail)
    }
    // 默认未知错误响应
    pub fn unknown_error_default() -> Self {
        Self::from(WrapperErr::UnknownError)
    }
    // 设置错误状态
    pub fn fail(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::Fail.code();
        self.msg = msg.into();
    }

    // 设置未知错误状态
    pub fn unknown_error(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::UnknownError.code();
        self.msg = msg.into();
    }
    // 从 WrapperErr 创建响应包装
    pub fn from(err: WrapperErr) -> Self {
        ResponseWrapper {
            code: err.code(),
            message: err.message().to_string(),
        }
    }
}