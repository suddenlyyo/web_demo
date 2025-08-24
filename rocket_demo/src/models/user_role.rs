//! 用户角色关联实体模型

use rocket::serde::{Deserialize, Serialize};

/// 用户角色关联信息实体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserRole {
    /// 用户ID
    pub user_id: String,
    /// 角色ID
    pub role_id: String,
}
