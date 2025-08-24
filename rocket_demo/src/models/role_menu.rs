//! 角色菜单关联实体模型

use rocket::serde::{Deserialize, Serialize};

/// 角色菜单关联信息实体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RoleMenu {
    /// 角色ID
    pub role_id: String,
    /// 菜单ID
    pub menu_id: String,
}
