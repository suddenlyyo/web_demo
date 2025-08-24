//! # 列表包装器
//!
//! 用于包装列表查询结果的结构体

use serde::{Deserialize, Serialize};

use crate::wrapper::response_trait::ResponseTrait;
use crate::wrapper::response_wrapper::ResponseWrapper;

/// 列表包装结构体
///
/// 用于统一 API 列表响应格式，包含状态码、消息和数据列表
///
/// 参见: [ResponseTrait], [ResponseWrapper]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListWrapper<T> {
    /// 基础响应包装器
    ///
    /// 类型: [ResponseWrapper]
    #[serde(flatten)]
    base: ResponseWrapper,
    /// 数据列表
    ///
    /// 类型: [Vec]<T>
    data: Vec<T>,
}

impl<T> ListWrapper<T> {
    /// 创建一个默认成功的 ListWrapper，数据为空
    ///
    /// # 返回值
    ///
    /// 新的ListWrapper实例
    pub fn new() -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: Vec::new(),
        }
    }

    /// 创建一个默认失败的 ListWrapper，数据为空
    ///
    /// # 返回值
    ///
    /// 新的ListWrapper实例（失败状态）
    pub fn fail_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::fail_default(),
            data: Vec::new(),
        }
    }

    /// 创建一个默认未知错误的 ListWrapper，数据为空
    ///
    /// # 返回值
    ///
    /// 新的ListWrapper实例（未知错误状态）
    pub fn unknown_error_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: Vec::new(),
        }
    }

    /// 设置为成功状态并附带数据
    ///
    /// # 参数
    ///
    /// * `data` - 要包装的数据列表
    pub fn set_success(&mut self, data: Vec<T>) {
        self.base = ResponseWrapper::success_default();
        self.data = data;
    }

    /// 获取基础响应包装的引用
    ///
    /// # 返回值
    ///
    /// 基础响应包装的引用
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    /// 获取数据列表的引用
    ///
    /// # 返回值
    ///
    /// 数据列表的引用
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
}

/// 实现 ResponseTrait 以便统一处理响应包装
impl<T> ResponseTrait for ListWrapper<T> {
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
    /// # 返回값
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
        self.data = Vec::new();
    }

    /// 设置为未知错误响应，并自定义消息，数据清空
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = Vec::new();
    }
}
