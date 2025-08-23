//! 菜单数据访问层接口定义

use std::fmt::Debug;

use rocket::async_trait;

use crate::models::Menu;

/// 菜单数据访问trait
#[async_trait]
pub trait MenuRepository: Debug + Send + Sync {
    /// 根据ID获取菜单信息
    async fn get_menu_by_id(&self, id: &str) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取菜单列表
    async fn list_menus(&self) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>>;

    /// 分页查询菜单列表
    async fn list_menus_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Menu>, u64, u64), Box<dyn std::error::Error + Send + Sync>>;

    /// 新增菜单
    async fn add_menu(&self, menu: Menu) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>>;

    /// 修改菜单
    async fn update_menu(&self, menu: Menu) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>>;

    /// 删除菜单
    async fn delete_menu(&self, id: &str) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>>;

    /// 修改菜单状态
    async fn update_menu_status(&self, id: &str, status: i32) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>>;
}
