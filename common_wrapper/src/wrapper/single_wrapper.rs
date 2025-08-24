//! # 单个对象包装器
//!
//! 用于包装单个对象查询结果的结构体

use serde::{Deserialize, Serialize};

use crate::wrapper::response_trait::ResponseTrait;
use crate::wrapper::response_wrapper::ResponseWrapper;

/// 单个对象包装结构体
///
/// 用于统一 API 单个对象响应格式，包含状态码、消息和单个数据对象
///
/// 参见: [ResponseTrait], [ResponseWrapper]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SingleWrapper<T> {
    /// 基础响应包装器
    ///
    /// 类型: [ResponseWrapper]
    base: ResponseWrapper,
    /// 数据对象
    ///
    /// 类型: [Option]<T> (泛型)
    data: Option<T>,
}

impl<T> SingleWrapper<T> {
    /// 创建一个新的 SingleWrapper，数据为空
    ///
    /// # 返回值
    ///
    /// 新的SingleWrapper实例
    pub fn new() -> Self {
        Self { base: ResponseWrapper::success_default(), data: None }
    }

    /// 创建一个默认失败的 SingleWrapper，数据为空
    ///
    /// # 返回值
    ///
    /// 新的SingleWrapper实例（失败状态）
    pub fn fail_default(&mut self) -> Self {
        Self { base: ResponseWrapper::fail_default(), data: None }
    }

    /// 创建一个默认未知错误的 SingleWrapper，数据为空
    ///
    /// # 返回值
    ///
    /// 新的SingleWrapper实例（未知错误状态）
    pub fn unknown_error_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: None,
        }
    }

    /// 设置为成功状态并附带数据
    ///
    /// # 参数
    ///
    /// * `data` - 要包装的数据
    pub fn set_success(&mut self, data: T) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
    }

    /// 获取基础响应包装的引用
    ///
    /// # 返回值
    ///
    /// 基础响应包装的引用
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    /// 获取数据的引用
    ///
    /// # 返回值
    ///
    /// 如果存在数据则返回Some(&T)，否则返回None
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

impl<T> Default for SingleWrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 实现 ResponseTrait 以便统一处理响应包装
impl<T> ResponseTrait for SingleWrapper<T> {
    /// 获取响应码
    ///
    /// # 返回值
    ///
    /// 响应码
    fn get_code(&self) -> i32 {
        self.base.get_code()
    }

    /// 获取响应消息
    ///
    /// # 返回值
    ///
    /// 响应消息的引用
    fn get_message(&self) -> &str {
        self.base.get_message()
    }

    /// 判断是否为成功响应
    ///
    /// # 返回值
    ///
    /// 如果响应成功返回true，否则返回false
    fn is_success(&self) -> bool {
        self.base.is_success()
    }

    /// 设置为失败响应，并自定义消息，数据清空
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的失败消息
    fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
    }

    /// 设置为未知错误响应，并自定义消息，数据清空
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
    }
}
