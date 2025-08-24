//! 角色菜单数据访问层接口定义

use crate::models::RoleMenu;
use rocket::async_trait;
use std::fmt::Debug;

/// 角色菜单数据访问trait
#[async_trait]
pub trait RoleMenuRepository: Debug + Send + Sync {
    /// 根据角色ID和菜单ID删除角色菜单
    ///
    /// # 参数
    /// * `role_id` - 角色ID，类型: [&str]
    /// * `menu_id` - 菜单ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn delete_by_primary_key(&self, role_id: &str, menu_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 插入角色菜单记录
    ///
    /// # 参数
    /// * `row` - 角色菜单信息，类型: [&RoleMenu]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn insert(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 选择性插入角色菜单记录
    ///
    /// # 参数
    /// * `row` - 角色菜单信息，类型: [&RoleMenu]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn insert_selective(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据角色ID查询角色菜单列表
    ///
    /// # 参数
    /// * `role_id` - 角色ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回角色菜单列表，类型: [Result<Vec<RoleMenu>, Box<dyn std::error::Error + Send + Sync>>]
    async fn select_role_menu_by_role_id(&self, role_id: &str) -> Result<Vec<RoleMenu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 批量插入角色菜单
    ///
    /// # 参数
    /// * `list` - 角色菜单列表，类型: [&[RoleMenu]]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn batch_insert(&self, list: &[RoleMenu]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据角色ID和菜单ID列表批量删除角色菜单
    ///
    /// # 参数
    /// * `role_id` - 角色ID，类型: [&str]
    /// * `list` - 菜单ID列表，类型: [&[String]]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn batch_delete_by_role_id_and_menu_ids(&self, role_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据角色ID删除角色菜单
    ///
    /// # 参数
    /// * `role_id` - 角色ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn delete_by_role_id(&self, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
