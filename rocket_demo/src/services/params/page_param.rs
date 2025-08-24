//! 分页参数定义

/// 公共分页参数
#[derive(Debug, Clone)]
pub struct PageParam {
    /// 页码
    pub page_num: Option<u64>,
    /// 每页条数
    pub page_size: Option<u64>,
}
