//! # 分页信息结构体
//!
//! 用于描述分页请求参数

use serde::Serialize;

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
}
