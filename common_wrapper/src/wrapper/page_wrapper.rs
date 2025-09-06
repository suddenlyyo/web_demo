//! # 分页包装器
//!
//! 用于包装分页查询结果的结构体

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
    #[serde(flatten)]
    base: ResponseWrapper,
    /// 数据列表
    ///
    /// 类型: [Option]<[Vec]<T>>
    data: Option<Vec<T>>,
    /// 总记录数
    ///
    /// 类型: [u64]
    total: u64,
    /// 总页数
    ///
    /// 类型: [u64]
    total_page: u64,
    /// 当前页码
    ///
    /// 类型: [u64]
    current_page: u64,
    /// 每页大小
    ///
    /// 类型: [u64]
    page_size: u64,
}

impl<T> PageWrapper<T> {
    /// 创建一个默认成功的 PageWrapper，数据为空，分页信息为初始值
    ///
    /// # 返回值
    ///
    /// 新的PageWrapper实例
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
    /// # 返回值
    ///
    /// 新的PageWrapper实例（失败状态）
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
    /// # 返回值
    ///
    /// 新的PageWrapper实例（未知错误状态）
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
    /// # 参数
    ///
    /// * `data` - 要包装的数据列表
    /// * `total` - 总条数
    /// * `current_page` - 当前页码
    /// * `page_size` - 每页大小
    pub fn set_success(&mut self, data: Vec<T>, total: u64, current_page: u64, page_size: u64) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
        self.total = total;
        self.total_page = Self::calculate_total_pages(total, page_size);
        self.current_page = current_page;
        self.page_size = page_size;
    }

    /// 设置为失败状态并附带消息
    ///
    /// # 参数
    ///
    /// * `msg` - 失败消息
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
    /// # 参数
    ///
    /// * `msg` - 未知错误消息
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
    /// 基础响应包装器的引用
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    /// 获取数据列表的引用
    ///
    /// # 返回值
    ///
    /// 数据列表的引用（Option包装）
    pub fn get_data(&self) -> &Option<Vec<T>> {
        &self.data
    }

    /// 获取总记录数
    ///
    /// # 返回值
    ///
    /// 总记录数
    pub fn get_total(&self) -> u64 {
        self.total
    }

    /// 获取总页数
    ///
    /// # 返回值
    ///
    /// 总页数
    pub fn get_total_page(&self) -> u64 {
        self.total_page
    }

    /// 获取当前页码
    ///
    /// # 返回值
    ///
    /// 当前页码
    pub fn get_current_page(&self) -> u64 {
        self.current_page
    }

    /// 获取每页大小
    ///
    /// # 返回值
    ///
    /// 每页大小
    pub fn get_page_size(&self) -> u64 {
        self.page_size
    }

    /// 根据总记录数和每页大小计算总页数
    ///
    /// 使用公式: total_page = (total + page_size - 1) / page_size
    /// 这种计算方式避免了使用浮点运算或条件判断，更高效且准确
    ///
    /// # 参数
    ///
    /// * `total` - 总记录数
    /// * `page_size` - 每页大小
    ///
    /// # 返回值
    ///
    /// 总页数
    ///
    /// # 示例
    ///
    /// ```rust
    /// use common_wrapper::PageWrapper;
    ///
    /// assert_eq!(PageWrapper::<String>::calculate_total_pages(0, 10), 0);  // 0条记录 = 0页
    /// assert_eq!(PageWrapper::<String>::calculate_total_pages(1, 10), 1);  // 1条记录 = 1页
    /// assert_eq!(PageWrapper::<String>::calculate_total_pages(10, 10), 1); // 10条记录 = 1页
    /// assert_eq!(PageWrapper::<String>::calculate_total_pages(11, 10), 2); // 11条记录 = 2页
    /// ```
    pub fn calculate_total_pages(total: u64, page_size: u64) -> u64 {
        if page_size == 0 {
            return 0;
        }
        (total + page_size - 1) / page_size
    }
}

/// 实现 ResponseTrait 以便统一处理响应包装
impl<T> ResponseTrait for PageWrapper<T> {
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
        self.total = 0;
        self.total_page = 0;
        self.current_page = 1;
        self.page_size = 0;
    }

    /// 设置为未知错误响应，并自定义消息，数据清空
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息
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
