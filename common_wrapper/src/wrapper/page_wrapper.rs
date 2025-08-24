//! # 分页数据包装器
//!
//! 用于包装分页数据的响应结构

use crate::wrapper::{ResponseTrait, ResponseWrapper};

use serde::{Deserialize, Serialize};

/// # 带分页数据的响应包装结构体
/// 用于统一 API 响应格式，包含基础响应信息和可选的分页数据
///
/// # 示例
///
/// ```rust
/// use common_wrapper::{PageWrapper,ResponseTrait};
///
/// let mut wrapper = PageWrapper::new();
/// wrapper.set_success(vec!["Hello", "World"], 100, 1, 10);
/// assert!(wrapper.is_success());
/// assert_eq!(wrapper.get_total(), Some(&100));
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PageWrapper<T> {
    /// 响应基础信息，包含响应码和消息
    #[serde(flatten)] // 扁平化，去掉json中的base把内部结构解构出来
    pub base: ResponseWrapper,
    /// 可选的数据列表
    pub data: Option<Vec<T>>,
    /// 总记录数
    pub total: Option<u64>,
    /// 总页数
    pub total_page: Option<u64>,
    /// 当前页码
    pub current_page: Option<u64>,
    /// 每页条数
    pub page_size: Option<u64>,
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
            data: None,
            total: None,
            total_page: None,
            current_page: None,
            page_size: None,
        }
    }

    /// 设置分页成功的响应数据
    ///
    /// # 参数
    ///
    /// - `data`: 数据列表
    /// - `total`: 总记录数
    /// - `total_page`: 总页数
    /// - `current_page`: 当前页码
    /// - `page_size`: 每页条数
    pub fn set_success_pagination(&mut self, data: Vec<T>, total: u64, total_page: u64, current_page: u64, page_size: u64) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
        self.total = Some(total);
        self.total_page = Some(total_page);
        self.current_page = Some(current_page);
        self.page_size = Some(page_size);
    }

    /// 设置分页失败的响应数据
    ///
    /// # 参数
    ///
    /// - `msg`: 错误消息
    pub fn set_fail_pagination(&mut self, msg: &str) {
        self.base.set_fail(msg);
        self.data = None;
        self.total = None;
        self.total_page = None;
        self.current_page = None;
        self.page_size = None;
    }

    /// 获取总记录数
    ///
    /// # 返回值
    ///
    /// 返回总记录数的引用
    pub fn get_total(&self) -> Option<&u64> {
        self.total.as_ref()
    }

    /// 获取总页数
    ///
    /// # 返回值
    ///
    /// 返回总页数的引用
    pub fn get_total_page(&self) -> Option<&u64> {
        self.total_page.as_ref()
    }

    /// 获取当前页码
    ///
    /// # 返回值
    ///
    /// 返回当前页码的引用
    pub fn get_current_page(&self) -> Option<&u64> {
        self.current_page.as_ref()
    }

    /// 获取每页条数
    ///
    /// # 返回值
    ///
    /// 返回每页条数的引用
    pub fn get_page_size_ref(&self) -> Option<&u64> {
        self.page_size.as_ref()
    }

    /// 设置为成功响应，并附带数据和分页信息（自动计算总页数）
    ///
    /// # 参数
    ///
    /// * `data` - 要包装的数据列表
    /// * `total` - 总条数
    /// * `current_page` - 当前页码
    /// * `page_size` - 每页大小
    pub fn set_success(&mut self, data: Vec<T>, total: u64, current_page: u64, page_size: u64) {
        let total_page = Self::calculate_total_pages(total, page_size);
        self.set_success_pagination(data, total, total_page, current_page, page_size);
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

    /// 获取总条数
    pub fn get_total_count(&self) -> u64 {
        self.total.unwrap_or(0)
    }

    /// 获取总页数
    pub fn get_total_page_count(&self) -> u64 {
        self.total_page.unwrap_or(0)
    }

    /// 获取当前页码
    pub fn get_current_page_num(&self) -> u64 {
        self.current_page.unwrap_or(0)
    }

    /// 获取每页大小
    pub fn get_page_size(&self) -> u64 {
        self.page_size.unwrap_or(0)
    }

    /// 获取数据
    pub fn get_data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
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

    /// 设置为失败响应，并自定义消息，数据和分页信息重置
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的失败消息
    fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
        self.total = None;
        self.total_page = None;
        self.current_page = None;
        self.page_size = None;
    }

    /// 设置为未知错误响应，并自定义消息，数据和分页信息重置
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
        self.total = None;
        self.total_page = None;
        self.current_page = None;
        self.page_size = None;
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
        assert_eq!(page_wrapper.get_data(), Some(&vec!["item1", "item2"]));
    }
}
