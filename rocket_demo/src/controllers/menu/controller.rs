/// 获取路由信息
///
/// # 参数
///
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的路由信息列表，类型: [SingleWrapper<Vec<serde_json::Value>>]
#[get("/menu/getRouters")]
pub async fn get_routers(menu_service: &rocket::State<MenuServiceImpl>) -> SingleWrapper<Vec<serde_json::Value>> {
    // TODO: 实现获取路由信息的逻辑
    SingleWrapper::new()
}

/// 获取菜单树
///
/// # 参数
///
/// * `menu_param` - 菜单查询参数，类型: [MenuParam]
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的菜单树列表，类型: [SingleWrapper<Vec<serde_json::Value>>]
#[get("/menu/getMenuTree")]
pub async fn get_menu_tree(menu_param: MenuParam, menu_service: &rocket::State<MenuServiceImpl>) -> SingleWrapper<Vec<serde_json::Value>> {
    // TODO: 实现获取菜单树的逻辑
    SingleWrapper::new()
}

/// 获取菜单列表
///
/// # 参数
///
/// * `menu_param` - 菜单查询参数，类型: [MenuParam]
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的菜单列表，类型: [serde_json::Value]
#[get("/menu/list")]
pub async fn list_menus(menu_param: MenuParam, menu_service: &rocket::State<MenuServiceImpl>) -> serde_json::Value {
    // TODO: 实现获取菜单列表的逻辑
    serde_json::Value::Null
}

/// 修改菜单状态
///
/// # 参数
///
/// * `menu_param` - 菜单信息，类型: [Json<MenuParam>]
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的菜单信息，类型: [ResponseWrapper]
#[put("/menu/editMenuStatus", data = "<menu_param>")]
pub async fn edit_menu_status(menu_param: Json<MenuParam>, menu_service: &rocket::State<MenuServiceImpl>) -> ResponseWrapper {
    if let (Some(id), Some(status)) = (menu_param.id.as_ref(), menu_param.status) {
        menu_service.edit_menu_status(id, status).await
    } else {
        ResponseWrapper::new(500, "参数不完整")
    }
}
//! 菜单控制器

use rocket::{delete, get, post, put, serde::json::Json};
use common_wrapper::{SingleWrapper, ResponseWrapper};
use crate::services::menu::menu_service::MenuServiceImpl;


/// 获取菜单列表
///
/// # 参数
///
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的菜单列表，类型: [ListWrapper<Menu>]
#[get("/menu/list")]
pub async fn list_menus(menu_service: &rocket::State<MenuServiceImpl>) -> ListWrapper<Menu> {
    menu_service.list_menus().await
}


/// 新增菜单
///
/// # 参数
///
/// * `menu_param` - 菜单信息，类型: [Json<MenuParam>]
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的菜单信息，类型: [ResponseWrapper]
#[post("/menu/addMenu", data = "<menu_param>")]
pub async fn add_menu(menu_param: Json<MenuParam>, menu_service: &rocket::State<MenuServiceImpl>) -> ResponseWrapper {
    menu_service.add_menu(menu_param.into_inner()).await
}

/// 编辑菜单
///
/// # 参数
///
/// * `menu_param` - 菜单信息，类型: [Json<MenuParam>]
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的菜单信息，类型: [ResponseWrapper]
#[put("/menu/editMenu", data = "<menu_param>")]
pub async fn edit_menu(menu_param: Json<MenuParam>, menu_service: &rocket::State<MenuServiceImpl>) -> ResponseWrapper {
    menu_service.edit_menu(menu_param.into_inner()).await
}

/// 删除菜单
///
/// # 参数
///
/// * `menu_id` - 菜单ID，类型: [String]
/// * `menu_service` - 菜单服务，类型: [&rocket::State<MenuServiceImpl>]
///
/// # 返回值
///
/// 返回包装后的菜单信息，类型: [ResponseWrapper]
#[delete("/menu/deleteMenu/<menu_id>")]
pub async fn delete_menu(menu_id: String, menu_service: &rocket::State<MenuServiceImpl>) -> ResponseWrapper {
    menu_service.delete_menu(&menu_id).await
}

/// 获取菜单控制器的所有路由
pub fn routes() -> Vec<rocket::Route> {
    routes![get_routers, get_menu_tree, list_menus, add_menu, edit_menu, delete_menu, edit_menu_status]
}
