use crate::enums::WrapperErrEnum;
use crate::wrapper::ResponseWrapper;

use serde::Serialize;
/// 分页包装
#[derive(Debug, Serialize, PartialEq, Eq, Hash)]
pub struct PageWrapper<T> {
    #[serde(flatten)]
    pub base: ResponseWrapper,
    pub data: Option<Vec<T>>,
    pub total: u64,
    pub total_page: u64,
    pub current_page_num: u64,
    pub page_size: u64,
}

impl<T> PageWrapper<T> {
    pub fn new() -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: None,
            total: 0u64,
            total_page: 0u64,
            current_page_num: 1u64,
            page_size: 0u64,
        }
    }

    pub fn success(
        data: Vec<T>,
        total: u64,
        total_page: u64,
        current_page_num: u64,
        page_size: u64,
    ) -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: Some(data),
            total,
            total_page,
            current_page_num,
            page_size,
        }
    }

    pub fn set_fail(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErrEnum::Fail.code(), msg.into());
        self.data = None;
        self.total = 0u64;
        self.total_page = 0u64;
        self.current_page_num = 1u64;
        self.page_size = 0u64;
    }

    pub fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErrEnum::UnknownError.code(), msg.into());
        self.data = None;
        self.total = 0u64;
        self.total_page = 0u64;
        self.current_page_num = 1u64;
        self.page_size = 0u64;
    }
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    pub fn get_data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }
    pub fn get_total(&self) -> u64 {
        self.total
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

/// 分页信息结构体
#[derive(Debug,Serialize, PartialEq, Eq, Hash)]
pub struct PageInfo {
    /// 当前页数
    page_num: Option<u64>,
    /// 页面大小
    page_size: Option<u64>,
}

impl PageInfo {
    /// 创建一个新的 PageInfo 实例
    pub fn new(page_num: Option<u64>, page_size: Option<u64>) -> Self {
        Self {
            page_num,
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
        let page_num = self.page_num.unwrap_or(1);
        let page_size = self.get_page_size();

        // 确保不会因减1导致下溢（当 page_num=0 时）
        if page_num == 0 {
            0
        } else {
            (page_num - 1) * page_size
        }
    }
}
