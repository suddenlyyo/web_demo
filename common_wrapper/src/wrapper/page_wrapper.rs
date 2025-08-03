//! # 分页包装器
//!
//! 用于包装分页数据的响应结构，包含分页相关信息

use crate::wrapper::ResponseWrapper;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};

/// # 分页响应包装结构体
/// 用于统一 API 响应格式，包含分页相关信息和数据列表
///
/// # 示例
///
/// ```rust
/// use common_wrapper::PageWrapper;
///
/// let mut wrapper = PageWrapper::new();
/// wrapper.set_success(vec!["item1", "item2"], 100, 10, 1, 10);
/// assert!(wrapper.is_success());
/// assert_eq!(wrapper.get_total_count(), 100);
/// assert_eq!(wrapper.get_current_page_num(), 1);
/// assert_eq!(wrapper.get_page_size(), 10);
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PageWrapper<T> {
    /// 响应基础信息
    #[serde(flatten)]
    pub base: ResponseWrapper,
    /// 可选的数据列表
    pub data: Option<Vec<T>>,
    /// 总条数
    pub total_count: u64,
    /// 总页数
    pub total_page: u64,
    /// 当前页码
    pub current_page_num: u64,
    /// 每页大小
    pub page_size: u64,
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
            total_count: 0u64,
            total_page: 0u64,
            current_page_num: 1u64,
            page_size: 0u64,
        }
    }

    /// 创建一个默认失败的 PageWrapper，数据为空，分页信息为初始值
    ///
    /// # 返回值
    ///
    /// 新的PageWrapper实例（失败状态）
    pub fn fail_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::fail_default(),
            data: None,
            total_count: 0u64,
            total_page: 0u64,
            current_page_num: 1u64,
            page_size: 0u64,
        }
    }

    /// 创建一个默认未知错误的 PageWrapper，数据为空，分页信息为初始值
    ///
    /// # 返回值
    ///
    /// 新的PageWrapper实例（未知错误状态）
    pub fn unknown_error_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: None,
            total_count: 0u64,
            total_page: 0u64,
            current_page_num: 1u64,
            page_size: 0u64,
        }
    }

    /// 设置为成功状态并附带数据和分页信息
    ///
    /// # 参数
    ///
    /// * `data` - 要包装的数据列表
    /// * `total_count` - 总条数
    /// * `total_page` - 总页数
    /// * `current_page_num` - 当前页码
    /// * `page_size` - 每页大小
    pub fn set_success(&mut self, data: Vec<T>, total_count: u64, total_page: u64, current_page_num: u64, page_size: u64) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
        self.total_count = total_count;
        self.total_page = total_page;
        self.current_page_num = current_page_num;
        self.page_size = page_size;
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
    /// 如果存在数据列表则返回Some(&Vec<T>)，否则返回None
    pub fn get_data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }

    /// 获取总条数
    ///
    /// # 返回值
    ///
    /// 总条数
    pub fn get_total_count(&self) -> u64 {
        self.total_count
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
    pub fn get_current_page_num(&self) -> u64 {
        self.current_page_num
    }

    /// 获取每页大小
    ///
    /// # 返回值
    ///
    /// 每页大小
    pub fn get_page_size(&self) -> u64 {
        self.page_size
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
        self.total_count = 0u64;
        self.total_page = 0u64;
        self.current_page_num = 1u64;
        self.page_size = 0u64;
    }

    /// 设置为未知错误响应，并自定义消息，数据和分页信息重置
    ///
    /// # 参数
    ///
    /// * `msg` - 自定义的未知错误消息
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
        self.total_count = 0u64;
        self.total_page = 0u64;
        self.current_page_num = 1u64;
        self.page_size = 0u64;
    }
}

/// # 分页信息结构体
/// 用于描述分页请求参数
///
/// # 示例
///
/// ```rust
/// use common_wrapper::PageWrapper;
///
/// let page_info = PageWrapper::<String>::new();
/// assert_eq!(page_info.get_page_offset(), 0);
/// ```
#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub struct PageInfo {
    /// 当前页数
    current_page_num: Option<u64>,
    /// 页面大小
    page_size: Option<u64>,
}

impl PageInfo {
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

    /// 获取页面大小（带默认值逻辑）
    ///
    /// 如果页面大小为 None 或 0，则返回默认值 20
    ///
    /// # 返回值
    ///
    /// 页面大小（带默认值）
    pub fn get_page_size(&self) -> u64 {
        self.page_size.filter(|&size| size > 0).unwrap_or(20)
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
        let current_page_num = self.current_page_num.unwrap_or(1);
        let page_size = self.get_page_size();

        // 确保不会因减1导致下溢（当 current_page_num =0 时）
        if current_page_num == 0 { 0 } else { (current_page_num - 1) * page_size }
    }
}
