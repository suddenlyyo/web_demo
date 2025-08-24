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
pub struct PageWrapper<T> {
    /// 基础响应包装器
    ///
    /// 类型: [ResponseWrapper]
    #[serde(flatten)]
    base: ResponseWrapper,
    /// 数据列表
    ///
    /// 类型: [Vec]<T>
    data: Vec<T>,
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
            data: Vec::new(),
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
    pub fn fail_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::fail_default(),
            data: Vec::new(),
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
    pub fn unknown_error_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: Vec::new(),
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
    /// * `total_page` - 总页数
    /// * `current_page` - 当前页码
    /// * `page_size` - 每页大小
    pub fn set_success(&mut self, data: Vec<T>, total: u64, current_page: u64, page_size: u64) {
        let total_page = Self::calculate_total_pages(total, page_size);
        self.base = ResponseWrapper::success_default();
        self.data = data;
        self.total = total;
        self.total_page = total_page;
        self.current_page = current_page;
        self.page_size = page_size;
    }

    /// 获取数据列表的引用
    ///
    /// # 返回值
    ///
    /// 数据列表的引用
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_total_count(&self) -> u64 {
        self.total
    }

    pub fn get_total_page_count(&self) -> u64 {
        self.total_page
    }

    pub fn get_current_page_num(&self) -> u64 {
        self.current_page
    }

    pub fn get_page_size(&self) -> u64 {
        self.page_size
    }

    /// 计算总页数
    ///
    /// # 参数
    /// * `total` - 总记录数，类型: [u64]
    /// * `page_size` - 每页大小，类型: [u64]
    ///
    /// # 返回值
    /// 返回总页数，类型: [u64]
    ///
    /// # 示例
    /// ```
    /// let total_pages = PageWrapper::<()>::calculate_total_pages(100, 10);
    /// assert_eq!(total_pages, 10);
    /// ```
    pub fn calculate_total_pages(total: u64, page_size: u64) -> u64 {
        if page_size == 0 {
            return 0;
        }
        total.div_ceil(page_size)
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
        self.data = Vec::new();
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
        self.data = Vec::new();
        self.total = 0;
        self.total_page = 0;
        self.current_page = 1;
        self.page_size = 0;
    }
}

/// # 分页信息结构体
/// 用于描述分页请求参数
///
/// # 示例
///
/// ```rust
/// use common_wrapper::PageInfo;
///
/// let page_info = PageInfo::new(None, None);
/// assert_eq!(page_info.get_page_size(), PageInfo::DEFAULT_PAGE_SIZE);
/// ```
#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub struct PageInfo {
    /// 当前页数
    current_page_num: Option<u64>,
    /// 页面大小
    page_size: Option<u64>,
}

impl PageInfo {
    /// 默认页面大小,每页多少条数据
    pub const DEFAULT_PAGE_SIZE: u64 = 20;

    /// 默认当前页
    pub const DEFAULT_CURRENT_PAGE: u64 = 1;

    /// 最大页面大小限制，防止过大的分页请求
    pub const MAX_PAGE_SIZE: u64 = 1000;

    /// 创建一个新的 PageInfo 实例
    ///
    /// # 参数
    ///
    /// * `current_page_num` - 当前页数
    /// * `page_size` - 页面大小
    ///
    /// # 返回值
    ///
    /// 新的PageInfo实例
    pub fn new(current_page_num: Option<u64>, page_size: Option<u64>) -> Self {
        Self { current_page_num, page_size }
    }

    /// 使用默认值创建一个新的 PageInfo 实例
    ///
    /// 这是一个便捷方法，可以直接使用Option类型的页码和页面大小创建实例
    ///
    /// # 参数
    ///
    /// * `current_page_num` - 当前页数
    /// * `page_size` - 页面大小
    ///
    /// # 返回值
    ///
    /// 新的PageInfo实例
    pub fn new_with_defaults(page_num: Option<u64>, page_size: Option<u64>) -> Self {
        Self::new(page_num, page_size)
    }

    /// 获取页面大小（带默认值逻辑）
    ///
    /// 如果页面大小为 None 或 0，则返回默认值 20
    /// 最大页面大小限制为100
    ///
    /// # 返回值
    ///
    /// 页面大小（带默认值和最大值限制）
    pub fn get_page_size(&self) -> u64 {
        self.page_size
            .filter(|&size| size > 0)
            .map(|size| size.min(Self::MAX_PAGE_SIZE))
            .unwrap_or(Self::DEFAULT_PAGE_SIZE)
    }

    /// 获取当前页码（带默认值逻辑）
    ///
    /// 如果当前页码为 None 或 0，则返回默认值 1
    ///
    /// # 返回值
    ///
    /// 当前页码（带默认值）
    pub fn get_current_page_num(&self) -> u64 {
        self.current_page_num
            .filter(|&num| num > 0)
            .unwrap_or(Self::DEFAULT_CURRENT_PAGE)
    }

    /// 计算分页偏移量（起始索引）
    ///
    /// sql语句：select * from 表名 limit offset,pageSize;
    /// offset:就是当前页的起始索引（从 0 开始计数）,pageSize就是每页的条数.
    /// currentPage:就是当前页
    /// 计算公式:offset=(currentPage-1)*pageSize
    ///
    /// # 返回值
    ///
    /// 分页偏移量
    pub fn get_page_offset(&self) -> u64 {
        let current_page_num = self.get_current_page_num();
        let page_size = self.get_page_size();

        (current_page_num - 1) * page_size
    }

    /// 计算分页偏移量（起始索引）
    ///
    /// 这是一个便捷方法，可以直接计算偏移量
    ///
    /// # 返回值
    ///
    /// 分页偏移量
    pub fn calculate_offset(&self) -> u64 {
        self.get_page_offset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_defaults() {
        let page_info = PageInfo::new_with_defaults(Some(2), Some(10));
        assert_eq!(page_info.get_current_page_num(), 2);
        assert_eq!(page_info.get_page_size(), 10);
    }

    #[test]
    fn test_calculate_offset() {
        let page_info = PageInfo::new_with_defaults(Some(3), Some(20));
        assert_eq!(page_info.calculate_offset(), 40); // (3-1)*20 = 40
    }

    #[test]
    fn test_calculate_offset_with_defaults() {
        let page_info = PageInfo::new_with_defaults(None, None);
        assert_eq!(page_info.calculate_offset(), 0); // (1-1)*20 = 0 (默认页码为1，默认页面大小为20)
    }

    #[test]
    fn test_page_info_defaults() {
        // 测试默认值是否正确应用
        let page_info = PageInfo::new(None, None);
        assert_eq!(page_info.get_current_page_num(), PageInfo::DEFAULT_CURRENT_PAGE);
        assert_eq!(page_info.get_page_size(), PageInfo::DEFAULT_PAGE_SIZE);
    }

    #[test]
    fn test_page_info_edge_cases() {
        // 测试边界条件
        let page_info = PageInfo::new(Some(0), Some(0));
        assert_eq!(page_info.get_current_page_num(), PageInfo::DEFAULT_CURRENT_PAGE);
        assert_eq!(page_info.get_page_size(), PageInfo::DEFAULT_PAGE_SIZE);

        let page_info = PageInfo::new(Some(u64::MAX), Some(u64::MAX));
        assert_eq!(page_info.get_page_size(), PageInfo::MAX_PAGE_SIZE); // 应该被限制为最大值
    }

    #[test]
    fn test_page_info_new_with_defaults() {
        // 测试new_with_defaults方法的不同参数组合
        let page_info = PageInfo::new_with_defaults(None, None);
        assert_eq!(page_info.get_current_page_num(), PageInfo::DEFAULT_CURRENT_PAGE);
        assert_eq!(page_info.get_page_size(), PageInfo::DEFAULT_PAGE_SIZE);

        let page_info = PageInfo::new_with_defaults(Some(0), Some(0));
        assert_eq!(page_info.get_current_page_num(), PageInfo::DEFAULT_CURRENT_PAGE);
        assert_eq!(page_info.get_page_size(), PageInfo::DEFAULT_PAGE_SIZE);

        let page_info = PageInfo::new_with_defaults(Some(5), Some(100));
        assert_eq!(page_info.get_current_page_num(), 5);
        assert_eq!(page_info.get_page_size(), 100);
    }

    #[test]
    fn test_page_info_getters() {
        // 测试get_page_size的默认值和最大值限制
        let page_info = PageInfo::new(None, None);
        assert_eq!(page_info.get_page_size(), PageInfo::DEFAULT_PAGE_SIZE);

        let page_info = PageInfo::new(None, Some(0));
        assert_eq!(page_info.get_page_size(), PageInfo::DEFAULT_PAGE_SIZE);

        let page_info = PageInfo::new(None, Some(1500));
        assert_eq!(page_info.get_page_size(), PageInfo::MAX_PAGE_SIZE);

        // 测试get_current_page_num的默认值
        let page_info = PageInfo::new(None, None);
        assert_eq!(page_info.get_current_page_num(), PageInfo::DEFAULT_CURRENT_PAGE);

        let page_info = PageInfo::new(Some(0), None);
        assert_eq!(page_info.get_current_page_num(), PageInfo::DEFAULT_CURRENT_PAGE);
    }

    #[test]
    fn test_page_wrapper_calculate_total_pages() {
        // 测试 PageWrapper 中的计算总页数方法
        assert_eq!(PageWrapper::<String>::calculate_total_pages(0, 10), 0); // 0条记录 = 0页
        assert_eq!(PageWrapper::<String>::calculate_total_pages(1, 10), 1); // 1条记录 = 1页
        assert_eq!(PageWrapper::<String>::calculate_total_pages(10, 10), 1); // 10条记录 = 1页
        assert_eq!(PageWrapper::<String>::calculate_total_pages(11, 10), 2); // 11条记录 = 2页
        assert_eq!(PageWrapper::<String>::calculate_total_pages(20, 10), 2); // 20条记录 = 2页
        assert_eq!(PageWrapper::<String>::calculate_total_pages(21, 10), 3); // 21条记录 = 3页

        // 测试页面大小为0的情况
        assert_eq!(PageWrapper::<String>::calculate_total_pages(10, 0), 0); // 页面大小为0时，总页数为0

        // 测试大数值
        assert_eq!(PageWrapper::<String>::calculate_total_pages(1000, 10), 100); // 1000条记录，每页10条 = 100页
        assert_eq!(PageWrapper::<String>::calculate_total_pages(1001, 10), 101); // 1001条记录，每页10条 = 101页
    }

    #[test]
    fn test_page_wrapper_set_success() {
        let mut page_wrapper = PageWrapper::new();
        page_wrapper.set_success(vec!["item1", "item2"], 25, 1, 10);

        assert_eq!(page_wrapper.get_total_count(), 25);
        assert_eq!(page_wrapper.get_total_page_count(), 3); // 自动计算的总页数 (25 + 10 - 1) / 10 = 3
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 10);
        assert_eq!(page_wrapper.get_data(), &vec!["item1", "item2"]);
    }
}
