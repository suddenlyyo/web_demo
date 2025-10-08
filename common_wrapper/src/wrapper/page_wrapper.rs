//! # 分页包装器
//!
//! 用于包装分页查询结果的结构体
//!
//! 该结构体实现了 [ResponseTrait] trait，提供统一的响应处理接口。
//! 除了包含基本的响应信息外，还包含分页相关的信息，如总记录数、
//! 总页数、当前页码和每页大小。

use serde::{Deserialize, Serialize};

use crate::wrapper::response_trait::ResponseTrait;
use crate::wrapper::response_wrapper::ResponseWrapper;

/// 分页包装结构体
///
/// 用于统一 API 分页响应格式，包含分页信息和数据列表
///
/// 参见: [ResponseTrait]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PageWrapper<T> {
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
    /// 总记录数
    ///
    /// 类型: [u64]
    ///
    /// 数据库中满足查询条件的总记录数
    total: u64,
    /// 总页数
    ///
    /// 类型: [u64]
    ///
    /// 根据总记录数和每页大小计算出的总页数
    total_page: u64,
    /// 当前页码
    ///
    /// 类型: [u64]
    ///
    /// 当前请求的页码，从1开始计数
    current_page: u64,
    /// 每页大小
    ///
    /// 类型: [u64]
    ///
    /// 每页显示的记录数
    page_size: u64,
}

impl<T> PageWrapper<T> {
    /// 创建一个默认成功的 PageWrapper，数据为空，分页信息为初始值
    ///
    /// 创建一个默认成功的 PageWrapper 实例，数据部分初始化为空的 [Vec]，
    /// 分页信息设置为默认值
    ///
    /// # 返回值
    ///
    /// [PageWrapper]<T> - 新的PageWrapper实例
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn new() -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: Some(Vec::new()),
            total: 0,
            total_page: 0,
            current_page: 1,
            page_size: 0,
        }
    }

    /// 创建一个默认失败的 PageWrapper，数据为空
    ///
    /// 创建一个默认失败状态的 PageWrapper 实例，数据部分为 [None]，
    /// 分页信息设置为默认值
    ///
    /// # 返回值
    ///
    /// [PageWrapper]<T> - 新的PageWrapper实例（失败状态）
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn fail_default() -> Self {
        Self {
            base: ResponseWrapper::fail_default(),
            data: None,
            total: 0,
            total_page: 0,
            current_page: 1,
            page_size: 0,
        }
    }

    /// 创建一个默认未知错误的 PageWrapper，数据为空
    ///
    /// 创建一个默认未知错误状态的 PageWrapper 实例，数据部分为 [None]，
    /// 分页信息设置为默认值
    ///
    /// # 返回值
    ///
    /// [PageWrapper]<T> - 新的PageWrapper实例（未知错误状态）
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn unknown_error_default() -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: None,
            total: 0,
            total_page: 0,
            current_page: 1,
            page_size: 0,
        }
    }

    /// 设置为成功状态并附带数据和分页信息
    ///
    /// 将当前实例设置为成功状态，并用指定的数据列表和分页信息填充相应字段
    ///
    /// # 参数
    ///
    /// * `data` - 要包装的数据列表，类型: [Vec]<T>
    /// * `total` - 总记录数，类型: [u64]
    /// * `current_page` - 当前页码，类型: [u64]
    /// * `page_size` - 每页大小，类型: [u64]
    ///
    /// # 泛型参数
    ///
    /// * T - 数据列表中元素的类型
    pub fn set_success(&mut self, data: Vec<T>, total: u64, current_page: u64, page_size: u64) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
        self.total = total;
        self.current_page = current_page;
        self.page_size = page_size;
        // 计算总页数，如果有余数则加1
        //self.total_page = if page_size > 0 { (total + page_size - 1) / page_size } else { 0 };
        self.total_page = if page_size > 0 { total.div_ceil(page_size) } else { 0 };
    }

    /// 设置为失败状态并附带消息
    ///
    /// 将当前实例设置为失败状态，并用指定的消息更新响应消息，
    /// 同时清空数据部分
    ///
    /// # 参数
    ///
    /// * `msg` - 失败消息，类型: impl [Into]<[String]>，可以接受 &str 或 String
    pub fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
        self.total = 0;
        self.total_page = 0;
        self.current_page = 1;
        self.page_size = 0;
    }

    /// 设置为未知错误状态并附带消息
    ///
    /// 将当前实例设置为未知错误状态，并用指定的消息更新响应消息，
    /// 同时清空数据部分
    ///
    /// # 参数
    ///
    /// * `msg` - 未知错误消息，类型: impl [Into]<[String]>，可以接受 &str 或 String
    pub fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
        self.total = 0;
        self.total_page = 0;
        self.current_page = 1;
        self.page_size = 0;
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

    /// 获取总记录数
    ///
    /// # 返回值
    ///
    /// [u64] - 总记录数
    pub fn get_total(&self) -> u64 {
        self.total
    }

    /// 获取总页数
    ///
    /// # 返回值
    ///
    /// [u64] - 总页数
    pub fn get_total_page(&self) -> u64 {
        self.total_page
    }

    /// 获取当前页码
    ///
    /// # 返回值
    ///
    /// [u64] - 当前页码
    pub fn get_current_page(&self) -> u64 {
        self.current_page
    }

    /// 获取每页大小
    ///
    /// # 返回值
    ///
    /// [u64] - 每页大小
    pub fn get_page_size(&self) -> u64 {
        self.page_size
    }

    /// 对包装的数据执行类型转换操作
    ///
    /// 如果当前包装器处于成功状态，则使用提供的函数 [f] 将内部数据从 Vec<T> 转换为 Vec<U>，
    /// 并返回一个新的成功状态的 [PageWrapper]<U> 实例。
    /// 如果当前包装器处于失败状态，则返回一个新的失败状态的 [PageWrapper]<U> 实例，
    /// 保留原始的错误代码和消息，以及分页信息。
    ///
    /// # 参数
    ///
    /// * `f` - 类型转换函数，接受 Vec<T> 类型参数并返回 Vec<U> 类型值
    ///
    /// # 泛型参数
    ///
    /// * T - 原始数据类型
    /// * U - 转换后的数据类型
    /// * F - 转换函数类型，必须实现 [FnOnce](Vec<T>) -> Vec<U> trait
    ///
    /// # 返回值
    ///
    /// [PageWrapper]<U> - 转换后的新包装器实例
    ///
    /// # 示例
    ///
    /// ```
    /// use common_wrapper::PageWrapper;
    /// use common_wrapper::ResponseTrait;
    ///
    /// let mut int_wrapper = PageWrapper::new();
    /// int_wrapper.set_success(vec![1, 2, 3], 100, 1, 10);
    ///
    /// let string_wrapper = int_wrapper.map(|v| v.into_iter().map(|x| x.to_string()).collect());
    ///
    /// assert!(string_wrapper.is_success());
    /// assert_eq!(string_wrapper.get_data().as_ref().unwrap(), &vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    /// assert_eq!(string_wrapper.get_total(), 100);
    /// assert_eq!(string_wrapper.get_current_page(), 1);
    /// assert_eq!(string_wrapper.get_page_size(), 10);
    /// ```
    pub fn map<U, F>(self, f: F) -> PageWrapper<U>
    where
        F: FnOnce(Vec<T>) -> Vec<U>,
    {
        let mut new_wrapper = PageWrapper::<U>::new();
        if self.is_success() {
            if let Some(data) = self.data {
                new_wrapper.set_success(f(data), self.total, self.current_page, self.page_size);
            } else {
                new_wrapper.data = Some(Vec::new());
                new_wrapper.total = self.total;
                new_wrapper.total_page = self.total_page;
                new_wrapper.current_page = self.current_page;
                new_wrapper.page_size = self.page_size;
            }
        } else {
            new_wrapper.base = self.base;
            new_wrapper.data = None;
            new_wrapper.total = self.total;
            new_wrapper.total_page = self.total_page;
            new_wrapper.current_page = self.current_page;
            new_wrapper.page_size = self.page_size;
        }
        new_wrapper
    }
}

impl<T> Default for PageWrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 实现 ResponseTrait 以便统一处理响应包装
impl<T> ResponseTrait for PageWrapper<T> {
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
        self.total = 0;
        self.total_page = 0;
        self.current_page = 1;
        self.page_size = 0;
    }

    /// 设置为未知错误响应，并自定义消息，数据清空
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息，类型: impl [Into]<[String]>
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
        self.total = 0;
        self.total_page = 0;
        self.current_page = 1;
        self.page_size = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_defaults() {
        let page_wrapper = PageWrapper::<String>::new();
        assert!(page_wrapper.is_success());
        assert_eq!(page_wrapper.get_code(), 1);
        assert_eq!(page_wrapper.get_message(), "Success");
        assert_eq!(page_wrapper.get_data(), &Some(Vec::new()));
        assert_eq!(page_wrapper.get_total(), 0);
        assert_eq!(page_wrapper.get_total_page(), 0);
        assert_eq!(page_wrapper.get_current_page(), 1);
        assert_eq!(page_wrapper.get_page_size(), 0);
    }

    #[test]
    fn test_page_wrapper_set_success() {
        let mut page_wrapper = PageWrapper::new();
        page_wrapper.set_success(vec!["item1", "item2"], 25, 1, 10);

        assert_eq!(page_wrapper.get_total(), 25);
        assert_eq!(page_wrapper.get_total_page(), 3); // 自动计算的总页数 (25 + 10 - 1) / 10 = 3
        assert_eq!(page_wrapper.get_current_page(), 1);
        assert_eq!(page_wrapper.get_page_size(), 10);
        assert_eq!(page_wrapper.get_data(), &Some(vec!["item1", "item2"]));
    }
}
