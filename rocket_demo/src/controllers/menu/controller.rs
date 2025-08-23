//! 菜单控制器

use rocket::{delete, get, post, put, routes, serde::json::Json};

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};

use crate::models::menu::Menu;
use crate::services::menu::menu_service::MenuService;
use crate::services::menu::menu_service_impl::MenuServiceImpl;

/// 根据ID获取菜单信息
///
/// # 参数
///
/// * `id` - 菜单ID
/// * `menu_service` - 菜单服务
///
/// # 返回值
///
/// 返回包装后的菜单信息
#[get("/menu/<id>")]
pub async fn get_menu(id: String, menu_service: &rocket::State<MenuServiceImpl>) -> SingleWrapper<Menu> {
    menu_service.get_menu_by_id(&id).await
}

/// 获取菜单列表
///
/// # 参数
///
/// * `menu_service` - 菜单服务
///
/// # 返回值
///
/// 返回包装后的菜单列表
#[get("/menu/list")]
pub async fn list_menus(menu_service: &rocket::State<MenuServiceImpl>) -> ListWrapper<Menu> {
    menu_service.list_menus().await
}

/// 分页查询菜单列表
///
/// # 参数
///
/// * `page_num` - 页码（可选，默认为1）
/// * `page_size` - 每页大小（可选，默认为10）
/// * `menu_service` - 菜单服务
///
/// # 返回值
///
/// 返回包装后的分页菜单列表
#[get("/menu/page?<page_num>&<page_size>")]
pub async fn list_menus_by_page(page_num: Option<u64>, page_size: Option<u64>, menu_service: &rocket::State<MenuServiceImpl>) -> PageWrapper<Menu> {
    menu_service.list_menus_by_page(page_num, page_size).await
}

/// 新增菜单
///
/// # 参数
///
/// * `menu` - 菜单信息
/// * `menu_service` - 菜单服务
///
/// # 返回值
///
/// 返回包装后的菜单信息
#[post("/menu", data = "<menu>")]
pub async fn add_menu(menu: Json<Menu>, menu_service: &rocket::State<MenuServiceImpl>) -> SingleWrapper<Menu> {
    menu_service.add_menu(menu.into_inner()).await
}

/// 修改菜单
///
/// # 参数
///
/// * `id` - 菜单ID
/// * `menu` - 菜单信息
/// * `menu_service` - 菜单服务
///
/// # 返回值
///
/// 返回包装后的菜单信息
#[put("/menu/<_id>", data = "<menu>")]
pub async fn update_menu(_id: String, menu: Json<Menu>, menu_service: &rocket::State<MenuServiceImpl>) -> SingleWrapper<Menu> {
    menu_service.update_menu(menu.into_inner()).await
}

/// 删除菜单
///
/// # 参数
///
/// * `id` - 菜单ID
/// * `menu_service` - 菜单服务
///
/// # 返回值
///
/// 返回包装后的菜单信息
#[delete("/menu/<id>")]
pub async fn delete_menu(id: String, menu_service: &rocket::State<MenuServiceImpl>) -> SingleWrapper<Menu> {
    menu_service.delete_menu(&id).await
}

/// 修改菜单状态
///
/// # 参数
///
/// * `id` - 菜单ID
/// * `status` - 菜单状态
/// * `menu_service` - 菜单服务
///
/// # 返回值
///
/// 返回包装后的菜单信息
#[put("/menu/<id>/status/<status>")]
pub async fn update_menu_status(id: String, status: i32, menu_service: &rocket::State<MenuServiceImpl>) -> SingleWrapper<Menu> {
    menu_service.update_menu_status(&id, status).await
}

/// 注册菜单相关路由
pub fn routes() -> Vec<rocket::Route> {
    routes![get_menu, list_menus, list_menus_by_page, add_menu, update_menu, delete_menu, update_menu_status]
}
