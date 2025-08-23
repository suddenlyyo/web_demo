//! 角色数据访问层接口定义

use std::fmt::Debug;

use rocket::async_trait;

use crate::models::Role;

/// 角色数据访问trait
#[async_trait]
pub trait RoleRepository: Debug + Send + Sync {
    /// 根据ID获取角色信息
    async fn get_role_by_id(&self, id: &str) -> Result<Role, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取角色列表
    async fn list_roles(&self) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>>;

    /// 分页查询角色列表
    async fn list_roles_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Role>, u64, u64), Box<dyn std::error::Error + Send + Sync>>;

    /// 新增角色
    async fn add_role(&self, role: Role) -> Result<Role, Box<dyn std::error::Error + Send + Sync>>;

    /// 修改角色
    async fn update_role(&self, role: Role) -> Result<Role, Box<dyn std::error::Error + Send + Sync>>;

    /// 删除角色
    async fn delete_role(&self, id: &str) -> Result<Role, Box<dyn std::error::Error + Send + Sync>>;

    /// 修改角色状态
    async fn update_role_status(&self, id: &str, status: i32) -> Result<Role, Box<dyn std::error::Error + Send + Sync>>;
}
