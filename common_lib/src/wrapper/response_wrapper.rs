use crate::enums::WrapperErrEnum;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};
///# 响应包装
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResponseWrapper {
    code: i32,
    message: String,
}

impl ResponseWrapper {
    // 通用构造函数
    pub fn new<S: Into<String>>(code: i32, message: S) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
    
    // 默认成功响应
    pub fn success_default() -> Self {
        Self::from(WrapperErrEnum::Success)
    }

    // 默认失败响应
    pub fn fail_default() -> Self {
        Self::from(WrapperErrEnum::Fail)
    }
    
    // 默认未知错误响应
    pub fn unknown_error_default() -> Self {
        Self::from(WrapperErrEnum::UnknownError)
    }
}

impl ResponseTrait for ResponseWrapper {
    fn get_code(&self) -> i32 {
        self.code
    }

    fn get_message(&self) -> &str {
        &self.message
    }

    fn is_success(&self) -> bool {
        self.code == WrapperErrEnum::Success.code()
    }

    fn set_fail(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::Fail.code();
        self.message = msg.into();
    }

    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::UnknownError.code();
        self.message = msg.into();
    }
}