//! 角色参数定义

use serde::{Deserialize, Serialize};

use crate::params::page_param::PageParam;

/// 角色参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleParam {
    /// 角色ID
    pub id: Option<String>,
    /// 角色名称
    pub name: Option<String>,
    /// 角色权限字符串
    pub role_key: Option<String>,
    /// 显示顺序
    pub seq_no: Option<i32>,
    /// 角色状态（0正常 1停用）
    pub status: Option<i32>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 备注
    pub remark: Option<String>,
    /// 分页参数
    #[serde(flatten)]
    pub page_param: PageParam,
}
