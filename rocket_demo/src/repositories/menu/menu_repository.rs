//! 菜单数据访问层接口定义

use std::fmt::Debug;

use rocket::async_trait;

use crate::models::Menu;

/// 菜单数据访问trait
#[async_trait]
pub trait MenuRepository: Debug + Send + Sync {
    /// 根据主键删除菜单
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 插入菜单记录
    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 选择性插入菜单记录
    async fn insert_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键查询菜单
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Menu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键选择性更新菜单
    async fn update_by_primary_key_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键更新菜单
    async fn update_by_primary_key(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID查询菜单列表
    async fn select_sys_menu_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 查询所有菜单树
    async fn select_menu_tree_all(&self) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID查询菜单树
    async fn select_menu_tree_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 查询菜单列表
    async fn select_sys_menu_list(&self, menu_param: &Menu) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据父菜单ID查询子菜单列表
    async fn select_sys_menu_by_parent_id(&self, parent_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据角色ID查询菜单ID列表
    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>>;
}