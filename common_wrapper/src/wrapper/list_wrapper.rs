//! # 列表包装器
//!
//! 用于包装列表查询结果的结构体
//!
//! 该结构体实现了 [ResponseTrait] trait，提供统一的响应处理接口。
//! 数据部分使用 [Option]<[Vec]<T>> 类型存储，当操作成功时包含数据，
//! 失败时为 [None]。

use serde::{Deserialize, Serialize};

use crate::wrapper::response_trait::ResponseTrait;
use crate::wrapper::response_wrapper::ResponseWrapper;

/// 列表包装结构体
///
/// 用于统一 API 列表响应格式，包含状态码、消息和数据列表
///
/// 参见: [ResponseTrait], [ResponseWrapper]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ListWrapper<T> {
    /// 基础响应包装器
    ///
    /// 类型: [ResponseWrapper]
    ///
    /// 包含响应的状态码和消息，用于表示操作结果
    #[serde(flatten)]
    base: ResponseWrapper,
    /// 数据列表
    ///
    /// 类型: [Option]<[Vec]<T>>
    ///
    /// 当操作成功时包含数据列表，失败时为 [None]
    data: Option<Vec<T>>,
}

impl<T> ListWrapper<T> {
    /// 创建一个新的 ListWrapper，数据为空
    ///
    /// 创建一个默认成功的 ListWrapper 实例，数据部分初始化为空的 [Vec]
    ///
    /// # 返回值
    ///
    /// [ListWrapper]<T> - 新的ListWrapper实例
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn new() -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: Some(Vec::new()),
        }
    }

    /// 创建一个默认失败的 ListWrapper，数据为空
    ///
    /// 创建一个默认失败状态的 ListWrapper 实例，数据部分为 [None]
    ///
    /// # 返回值
    ///
    /// [ListWrapper]<T> - 新的ListWrapper实例（失败状态）
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn fail_default() -> Self {
        Self { base: ResponseWrapper::fail_default(), data: None }
    }

    /// 创建一个默认未知错误的 ListWrapper，数据为空
    ///
    /// 创建一个默认未知错误状态的 ListWrapper 实例，数据部分为 [None]
    ///
    /// # 返回值
    ///
    /// [ListWrapper]<T> - 新的ListWrapper实例（未知错误状态）
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn unknown_error_default() -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: None,
        }
    }

    /// 设置为成功状态并附带数据
    ///
    /// 将当前实例设置为成功状态，并用指定的数据列表填充数据部分
    ///
    /// # 参数
    ///
    /// * `data` - 要包装的数据列表，类型: [Vec]<T>
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn set_success(&mut self, data: Vec<T>) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
    }

    /// 设置为失败状态并附带消息
    ///
    /// 将当前实例设置为失败状态，并用指定的消息更新响应消息
    ///
    /// # 参数
    ///
    /// * `msg` - 失败消息，类型: impl [Into]<[String]>，可以接受 &str 或 String
    pub fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
    }

    /// 设置为未知错误状态并附带消息
    ///
    /// 将当前实例设置为未知错误状态，并用指定的消息更新响应消息
    ///
    /// # 参数
    ///
    /// * `msg` - 未知错误消息，类型: impl [Into]<[String]>，可以接受 &str 或 String
    pub fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
    }

    /// 获取基础响应包装器的引用
    ///
    /// # 返回值
    ///
    /// &[ResponseWrapper] - 基础响应包装器的引用
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    /// 获取数据列表的引用
    ///
    /// # 返回值
    ///
    /// &[Option]<[Vec]<T>> - 数据列表的引用
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn get_data(&self) -> &Option<Vec<T>> {
        &self.data
    }

    /// 将ListWrapper<T>映射为ListWrapper<U>
    ///
    /// 如果原ListWrapper是成功的，则使用提供的函数将数据转换为新的类型，
    /// 如果原ListWrapper是失败的，则保持失败状态不变
    ///
    /// # 参数
    ///
    /// * `f` - 转换函数，将Vec<T>转换为Vec<U>
    ///
    /// # 返回值
    ///
    /// ListWrapper<U> - 转换后的新ListWrapper
    ///
    /// # 泛型参数
    ///
    /// * T - 原数据列表中元素的类型
    /// * U - 新数据列表中元素的类型
    ///
    /// # 示例
    ///
    /// ```rust
    /// use common_wrapper::ListWrapper;
    ///
    /// let mut wrapper: ListWrapper<i32> = ListWrapper::new();
    /// wrapper.set_success(vec![1, 2, 3]);
    ///
    /// let string_wrapper: ListWrapper<String> = wrapper.map(|data| {
    ///     data.into_iter().map(|x| x.to_string()).collect()
    /// });
    ///
    /// assert_eq!(string_wrapper.get_data().as_ref().unwrap(), &vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    /// ```
    pub fn map<U, F>(self, f: F) -> ListWrapper<U>
    where
        F: FnOnce(Vec<T>) -> Vec<U>,
    {
        let mut new_wrapper = ListWrapper::<U>::new();
        if self.is_success() {
            if let Some(data) = self.data {
                new_wrapper.set_success(f(data));
            } else {
                new_wrapper.data = Some(Vec::new());
            }
        } else {
            new_wrapper.base = self.base;
            new_wrapper.data = None;
        }
        new_wrapper
    }
}

impl<T> Default for ListWrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 实现 ResponseTrait 以便统一处理响应包装
impl<T> ResponseTrait for ListWrapper<T> {
    /// 获取响应码
    ///
    /// 从基础响应包装器中获取响应码
    ///
    /// # 返回值
    ///
    /// [i32] - 响应码
    fn get_code(&self) -> i32 {
        self.base.get_code()
    }

    /// 获取响应消息
    ///
    /// 从基础响应包装器中获取响应消息
    ///
    /// # 返回值
    ///
    /// &[str] - 响应消息的引用
    fn get_message(&self) -> &str {
        self.base.get_message()
    }

    /// 判断是否为成功响应
    ///
    /// 根据基础响应包装器判断是否为成功响应
    ///
    /// # 返回值
    ///
    /// [bool] - 如果响应成功返回true，否则返回false
    fn is_success(&self) -> bool {
        self.base.is_success()
    }

    /// 设置为失败响应，并自定义消息，数据清空
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的失败消息，类型: impl [Into]<[String]>
    fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
    }

    /// 设置为未知错误响应，并自定义消息，数据清空
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息，类型: impl [Into]<[String]>
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
    }
}
