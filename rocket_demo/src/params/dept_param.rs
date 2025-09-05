//! 部门参数

use crate::params::page_param::PageParam;

/// 部门参数
#[derive(Debug, Clone, Default)]
pub struct DeptParam {
    pub id: Option<String>,
    pub parent_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub address: Option<String>,
    pub logo: Option<String>,
    pub dept_level: Option<String>,
    pub seq_no: Option<i32>,
    pub status: Option<i32>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub remark: Option<String>,
    pub page_param: PageParam,
}
