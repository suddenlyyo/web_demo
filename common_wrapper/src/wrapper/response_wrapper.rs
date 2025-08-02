use crate::enums::WrapperErrEnum;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};

/// # 响应包装结构体
/// 用于统一 API 响应格式，包含响应码和响应消息
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResponseWrapper {
    /// 响应码，通常用于标识请求结果（如成功、失败等）
    code: i32,
    /// 响应消息，描述请求结果的详细信息
    message: String,
}

impl ResponseWrapper {
    /// 通用构造函数，根据传入的 code 和 message 创建响应包装
    pub fn new<S: Into<String>>(code: i32, message: S) -> Self {
        Self { code, message: message.into() }
    }

    /// 默认成功响应，使用 WrapperErrEnum::Success
    pub fn success_default() -> Self {
        ResponseWrapper::from(WrapperErrEnum::Success)
    }

    /// 默认失败响应，使用 WrapperErrEnum::Fail
    pub fn fail_default() -> Self {
        ResponseWrapper::from(WrapperErrEnum::Fail)
    }

    /// 默认未知错误响应，使用 WrapperErrEnum::UnknownError
    pub fn unknown_error_default() -> Self {
        ResponseWrapper::from(WrapperErrEnum::UnknownError)
    }
}

/// 实现 WrapperErrEnum 到 ResponseWrapper 的转换
impl From<WrapperErrEnum> for ResponseWrapper {
    fn from(item: WrapperErrEnum) -> Self {
        Self { code: item as i32, message: item.message().to_owned() }
    }
}

/// 实现 ResponseTrait，便于统一处理响应包装
impl ResponseTrait for ResponseWrapper {
    /// 获取响应码
    fn get_code(&self) -> i32 {
        self.code
    }

    /// 获取响应消息
    fn get_message(&self) -> &str {
        &self.message
    }

    /// 判断是否为成功响应
    fn is_success(&self) -> bool {
        WrapperErrEnum::from(self.code) == WrapperErrEnum::Success
    }

    /// 设置为失败响应，并自定义消息
    fn set_fail(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::Fail as i32;
        self.message = msg.into();
    }

    /// 设置为未知错误响应，并自定义消息
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::UnknownError as i32;
        self.message = msg.into();
    }
}
