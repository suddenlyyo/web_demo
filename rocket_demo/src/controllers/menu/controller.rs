//! 菜单控制器

use rocket::{State, delete, get, post, put, routes, serde::json::Json};

use common_wrapper::{ListWrapper, PageWrapper, ResponseWrapper, SingleWrapper};

use crate::params::menu_param::MenuParam;
use crate::services::menu::menu_service::{MenuService, RouterVO};
use crate::services::menu::menu_service_impl::MenuServiceImpl;

/// 菜单控制器

/// 获取路由信息
#[get("/menu/getRouters")]
pub async fn get_routers(menu_service: &State<MenuServiceImpl>) -> SingleWrapper<Vec<RouterVO>> {
    // 注意：这里应该从请求中获取实际的用户ID
    let user_id = "1"; // 示例用户ID
    let routers = menu_service.select_menu_tree_by_user_id(user_id).await;
    let mut wrapper = SingleWrapper::new();
    wrapper.set_success(routers);
    wrapper
}

/// 获取菜单树
#[get("/menu/getMenuTree")]
pub async fn get_menu_tree(menu_service: &State<MenuServiceImpl>) -> SingleWrapper<Vec<RouterVO>> {
    // 注意：这里应该从请求中获取实际的用户ID
    let user_id = "1"; // 示例用户ID
    let routers = menu_service.select_menu_tree_by_user_id(user_id).await;
    let mut wrapper = SingleWrapper::new();
    wrapper.set_success(routers);
    wrapper
}

/// 查询菜单列表
#[get("/menu/list")]
pub async fn list_menus(menu_param: MenuParam, menu_service: &State<MenuServiceImpl>) -> PageWrapper<Menu> {
    let result = menu_service.select_menu_list(menu_param).await;
    let mut wrapper = PageWrapper::new();
    if let Some(data) = result.data {
        wrapper.set_page(1, 10, data); // TODO: 从参数中获取分页信息
    }
    wrapper.set_code(result.get_code());
    wrapper.set_message(result.get_message());
    wrapper
}

/// 修改菜单状态
#[put("/menu/editMenuStatus", data = "<menu_param>")]
pub async fn edit_menu_status(menu_param: Json<MenuParam>, menu_service: &State<MenuServiceImpl>) -> ResponseWrapper {
    if let (Some(id), Some(status)) = (&menu_param.id, menu_param.status) {
        menu_service.edit_menu_status(id, status).await
    } else {
        ResponseWrapper::new(500, "参数不完整")
    }
}

/// 新增菜单
#[post("/menu/addMenu", data = "<menu_param>")]
pub async fn add_menu(menu_param: Json<MenuParam>, menu_service: &State<MenuServiceImpl>) -> ResponseWrapper {
    menu_service.add_menu(menu_param.into_inner()).await
}

/// 编辑菜单
#[put("/menu/editMenu", data = "<menu_param>")]
pub async fn edit_menu(menu_param: Json<MenuParam>, menu_service: &State<MenuServiceImpl>) -> ResponseWrapper {
    menu_service.edit_menu(menu_param.into_inner()).await
}

/// 删除菜单
#[delete("/menu/deleteMenu/<menu_id>")]
pub async fn delete_menu(menu_id: String, menu_service: &State<MenuServiceImpl>) -> ResponseWrapper {
    menu_service.delete_menu(&menu_id).await
}

/// 注册菜单相关路由
///
/// # 返回值
///
/// 返回路由列表，类型: [Vec<rocket::Route>]
pub fn routes() -> Vec<rocket::Route> {
    routes![get_routers, get_menu_tree, list_menus, edit_menu_status, add_menu, edit_menu, delete_menu]
}
