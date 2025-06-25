use crate::enums::WrapperErr;
///# 响应包装
#[derive(Debug, serde::Serialize)]
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
        Self::from(WrapperErr::Success())
    }

    // 自定义成功响应
    pub fn success<S: Into<String>>(code: i32, message: S) -> Self {
        ResponseWrapper::new(code, message)
    }

    // 默认失败响应
    pub fn fail_default() -> Self {
        Self::from(WrapperErr::Fail())
    }

    // 自定义失败响应
    pub fn fail<S: Into<String>>(code: i32, message: S) -> Self {
        ResponseWrapper::new(code, message)
    }
    // 默认未知错误响应
    pub fn unknown_error_default() -> Self {
        Self::from(WrapperErr::UnknownError())
    }

    // 自定义未知错误响应
    pub fn unknown_error<S: Into<String>>(code: i32, message: S) -> Self {
        ResponseWrapper::new(code, message)
    }

    // 从 WrapperErr 创建响应包装
    pub fn from(err: WrapperErr) -> Self {
        ResponseWrapper {
            code: err.get_code(),
            message: err.message().to_string(),
        }
    }
}