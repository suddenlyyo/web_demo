//! # 响应包装器
//!
//! 基础响应包装结构体，包含响应码和响应消息
//!
//! 这是所有响应包装器的基础结构，包含了响应的基本信息。
//! 它实现了 [ResponseTrait] trait，可以直接用作简单的响应包装，
//! 也可以作为其他复杂响应包装器的基础。

use crate::enums::wrapper_err::WrapperErrEnum;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};

/// # 响应包装结构体
/// 用于统一 API 响应格式，包含响应码和响应消息
///
/// # 示例
///
/// ```rust
/// use common_wrapper::{ResponseWrapper,ResponseTrait};
/// use common_wrapper::enums::wrapper_err::WrapperErrEnum;
///
/// let success_response = ResponseWrapper::success_default();
/// assert_eq!(success_response.get_code(), WrapperErrEnum::Success as i32);
/// assert_eq!(success_response.get_message(), "Success");
///
/// let mut fail_response = ResponseWrapper::fail_default();
/// fail_response.set_fail("Something went wrong");
/// assert_eq!(fail_response.get_code(), WrapperErrEnum::Fail as i32);
/// assert_eq!(fail_response.get_message(), "Something went wrong");
/// ```
///
/// 参见: [ResponseTrait], [WrapperErrEnum]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWrapper {
    /// 响应码，通常用于标识请求结果（如成功、失败等）
    ///
    /// 类型: [i32]
    ///
    /// 响应码使用 [WrapperErrEnum] 枚举值进行定义，常见值有：
    /// - 1: 成功 (WrapperErrEnum::Success)
    /// - -1: 失败 (WrapperErrEnum::Fail)
    /// - -2: 未知错误 (WrapperErrEnum::UnknownError)
    code: i32,
    /// 响应消息，描述请求结果的详细信息
    ///
    /// 类型: [String]
    ///
    /// 响应消息通常与响应码对应，提供更详细的说明信息
    message: String,
}

impl ResponseWrapper {
    /// 通用构造函数，根据传入的 code 和 message 创建响应包装
    ///
    /// # 参数
    ///
    /// * `code` - 响应码，类型: [i32]
    /// * `message` - 响应消息，类型: [S] 其中 S 实现了 [Into]<[String]>
    ///
    /// # 返回值
    ///
    /// [ResponseWrapper] - 新的ResponseWrapper实例
    ///
    /// # 泛型参数
    ///
    /// * S - 消息类型，必须实现 [Into]<[String]> trait
    pub fn new<S: Into<String>>(code: i32, message: S) -> Self {
        Self { code, message: message.into() }
    }

    /// 默认成功响应，使用 WrapperErrEnum::Success
    ///
    /// 创建一个表示操作成功的默认响应包装器
    ///
    /// # 返回值
    ///
    /// [ResponseWrapper] - 表示成功的ResponseWrapper实例
    pub fn success_default() -> Self {
        ResponseWrapper::from(WrapperErrEnum::Success)
    }

    /// 默认失败响应，使用 WrapperErrEnum::Fail
    ///
    /// 创建一个表示操作失败的默认响应包装器
    ///
    /// # 返回值
    ///
    /// [ResponseWrapper] - 表示失败的ResponseWrapper实例
    pub fn fail_default() -> Self {
        ResponseWrapper::from(WrapperErrEnum::Fail)
    }

    /// 默认未知错误响应，使用 WrapperErrEnum::UnknownError
    ///
    /// 创建一个表示未知错误的默认响应包装器
    ///
    /// # 返回值
    ///
    /// [ResponseWrapper] - 表示未知错误的ResponseWrapper实例
    pub fn unknown_error_default() -> Self {
        ResponseWrapper::from(WrapperErrEnum::UnknownError)
    }
}

/// 实现 WrapperErrEnum 到 ResponseWrapper 的转换
impl From<WrapperErrEnum> for ResponseWrapper {
    /// 从WrapperErrEnum转换为ResponseWrapper
    ///
    /// # 参数
    ///
    /// * `item` - WrapperErrEnum枚举值
    ///
    /// # 返回值
    ///
    /// [ResponseWrapper] - 对应的ResponseWrapper实例
    fn from(item: WrapperErrEnum) -> Self {
        Self { code: item as i32, message: item.message().to_owned() }
    }
}

/// 实现 ResponseTrait，便于统一处理响应包装
impl ResponseTrait for ResponseWrapper {
    /// 获取响应码
    ///
    /// # 返回值
    ///
    /// [i32] - 响应码
    fn get_code(&self) -> i32 {
        self.code
    }

    /// 获取响应消息
    ///
    /// # 返回值
    ///
    /// &[str] - 响应消息的引用
    fn get_message(&self) -> &str {
        &self.message
    }

    /// 判断是否为成功响应
    ///
    /// # 返回值
    ///
    /// [bool] - 如果响应成功返回true，否则返回false
    fn is_success(&self) -> bool {
        WrapperErrEnum::from(self.code) == WrapperErrEnum::Success
    }

    /// 设置为失败响应，并自定义消息
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的失败消息，类型: impl [Into]<[String]>
    fn set_fail(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::Fail as i32;
        self.message = msg.into();
    }

    /// 设置为未知错误响应，并自定义消息
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息，类型: impl [Into]<[String]>
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.code = WrapperErrEnum::UnknownError as i32;
        self.message = msg.into();
    }
}
