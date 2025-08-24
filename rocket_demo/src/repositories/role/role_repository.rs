//! 角色数据访问层接口定义

use crate::models::Role;
use rocket::async_trait;
use std::fmt::Debug;

/// 角色数据访问trait
#[async_trait]
pub trait RoleRepository: Debug + Send + Sync {
    /// 根据主键删除角色
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 插入角色记录
    async fn insert(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 选择性插入角色记录
    async fn insert_selective(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键查询角色
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Role>, Box<dyn std::error::Error + Send + Sync>>;

    /// 查询角色列表
    async fn select_role_list(&self, row: &Role) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID查询角色列表
    async fn select_role_by_user_id(&self, user_id: &str) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键更新角色
    async fn update_by_primary_key(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键选择性更新角色
    async fn update_by_primary_key_selective(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}