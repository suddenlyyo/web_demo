use crate::wrapper::ResponseWrapper;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};
/// 分页包装
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PageWrapper<T> {
    /// 响应基础信息
    #[serde(flatten)]
    pub base: ResponseWrapper,
    /// 数据
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

    // 默认失败响应
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

    // 默认未知错误响应
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

    // 设置成功状态和数据
    pub fn set_success(
        &mut self,
        data: Vec<T>,
        total_count: u64,
        total_page: u64,
        current_page_num: u64,
        page_size: u64,
    ) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
        self.total_count = total_count;
        self.total_page = total_page;
        self.current_page_num = current_page_num;
        self.page_size = page_size;
    }

    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    pub fn get_data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }
    pub fn get_total_count(&self) -> u64 {
        self.total_count
    }
    pub fn get_total_page(&self) -> u64 {
        self.total_page
    }
    pub fn get_current_page_num(&self) -> u64 {
        self.current_page_num
    }
    pub fn get_page_size(&self) -> u64 {
        self.page_size
    }
}

impl<T> ResponseTrait for PageWrapper<T> {
    fn get_code(&self) -> i32 {
        self.base.get_code()
    }

    fn get_message(&self) -> &str {
        self.base.get_message()
    }

    fn is_success(&self) -> bool {
        self.base.is_success()
    }

    fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
        self.total_count = 0u64;
        self.total_page = 0u64;
        self.current_page_num = 1u64;
        self.page_size = 0u64;
    }

    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
        self.total_count = 0u64;
        self.total_page = 0u64;
        self.current_page_num = 1u64;
        self.page_size = 0u64;
    }
}

/// 分页信息结构体
#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub struct PageInfo {
    /// 当前页数
    current_page_num: Option<u64>,
    /// 页面大小
    page_size: Option<u64>,
}

impl PageInfo {
    /// 创建一个新的 PageInfo 实例
    pub fn new(current_page_num: Option<u64>, page_size: Option<u64>) -> Self {
        Self {
            current_page_num,
            page_size,
        }
    }

    /// 获取页面大小（带默认值逻辑）
    ///
    /// 如果页面大小为 None 或 0，则返回默认值 20
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
    pub fn get_page_offset(&self) -> u64 {
        let current_page_num = self.current_page_num.unwrap_or(1);
        let page_size = self.get_page_size();

        // 确保不会因减1导致下溢（当 current_page_num =0 时）
        if current_page_num == 0 {
            0
        } else {
            (current_page_num - 1) * page_size
        }
    }
}
