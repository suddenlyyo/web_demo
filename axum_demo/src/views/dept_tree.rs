//! 部门树节点视图定义

use serde::{Deserialize, Serialize};

/// 部门树节点
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptTree {
    /// 部门ID
    pub id: String,
    /// 上级部门ID
    pub parent_id: Option<String>,
    /// 部门名称
    pub name: Option<String>,
    /// 子部门列表
    pub children: Vec<DeptTree>,
}
