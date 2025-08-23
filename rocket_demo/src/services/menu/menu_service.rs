//! 菜单服务接口定义

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};

use crate::models::Menu;

/// 菜单服务trait
#[rocket::async_trait]
pub trait MenuService {
    /// 根据ID获取菜单信息
    async fn get_menu_by_id(&self, id: &str) -> SingleWrapper<Menu>;

    /// 获取菜单列表
    async fn list_menus(&self) -> ListWrapper<Menu>;

    /// 分页查询菜单列表
    async fn list_menus_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> PageWrapper<Menu>;

    /// 新增菜单
    async fn add_menu(&self, menu: Menu) -> SingleWrapper<Menu>;

    /// 修改菜单
    async fn update_menu(&self, menu: Menu) -> SingleWrapper<Menu>;

    /// 删除菜单
    async fn delete_menu(&self, id: &str) -> SingleWrapper<Menu>;

    /// 修改菜单状态
    async fn update_menu_status(&self, id: &str, status: i32) -> SingleWrapper<Menu>;
}
