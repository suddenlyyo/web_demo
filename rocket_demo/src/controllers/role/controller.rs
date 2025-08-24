//! 角色控制器层

use rocket::{delete, get, post, put, routes, serde::json::Json};
use common_wrapper::{SingleWrapper, ResponseWrapper};

use crate::services::role::role_service::{RoleService, RoleServiceImpl};

/// 获取角色列表
///
/// # 参数
/// * `role_param` - 角色参数，类型: [RoleParam]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回角色列表，类型: [serde_json::Value]
#[get("/role/list")]
pub async fn list_roles(role_param: RoleParam, role_service: &rocket::State<RoleServiceImpl>) -> serde_json::Value {
    // TODO: 实现获取角色列表的逻辑
    serde_json::Value::Null
}

/// 新增角色
///
/// # 参数
/// * `role_param` - 角色参数，类型: [Json<RoleParam>]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[post("/role/addRole", data = "<role_param>")]
pub async fn add_role(role_param: Json<RoleParam>, role_service: &rocket::State<RoleServiceImpl>) -> ResponseWrapper {
    let result = role_service.add_role(role_param.into_inner()).await;
    ResponseWrapper::new(200, "角色新增成功")
}

/// 编辑角色
///
/// # 参数
/// * `role_param` - 角色参数，类型: [Json<RoleParam>]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[put("/role/editRole", data = "<role_param>")]
pub async fn edit_role(role_param: Json<RoleParam>, role_service: &rocket::State<RoleServiceImpl>) -> ResponseWrapper {
    let result = role_service.edit_role(role_param.into_inner()).await;
    ResponseWrapper::new(200, "角色编辑成功")
}

/// 删除角色
///
/// # 参数
/// * `role_id` - 角色ID，类型: [String]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[delete("/role/deleteRole/<role_id>")]
pub async fn delete_role(role_id: String, role_service: &rocket::State<RoleServiceImpl>) -> ResponseWrapper {
    role_service.delete_role(&role_id).await;
    ResponseWrapper::new(200, "角色删除成功")
}

/// 修改角色状态
///
/// # 参数
/// * `role_param` - 角色参数，类型: [Json<RoleParam>]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[put("/role/editRoleStatus", data = "<role_param>")]
pub async fn edit_role_status(role_param: Json<RoleParam>, role_service: &rocket::State<RoleServiceImpl>) -> ResponseWrapper {
    if let (Some(id), Some(status)) = (role_param.id.as_ref(), role_param.status) {
        role_service.edit_role_status(id, status).await;
        ResponseWrapper::new(200, "角色状态修改成功")
    } else {
        ResponseWrapper::new(500, "参数不完整")
    }
}

/// 获取角色菜单列表
///
/// # 参数
/// * `role_id` - 角色ID，类型: [String]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回角色菜单ID集合，类型: [SingleWrapper<HashSet<String>>]
#[get("/role/getRoleMenuIdList/<role_id>")]
pub async fn get_role_menu_id_list(role_id: String, role_service: &rocket::State<RoleServiceImpl>) -> SingleWrapper<HashSet<String>> {
    role_service.select_menu_ids_by_role_id(&role_id).await
}

/// 角色设置权限
///
/// # 参数
/// * `json_str` - JSON字符串，类型: [String]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[put("/role/roleSetMenu", data = "<json_str>")]
pub async fn role_set_menu(json_str: String, role_service: &rocket::State<RoleServiceImpl>) -> ResponseWrapper {
    // TODO: 解析JSON并调用服务方法
    role_service.role_set_menu("", &[]).await
}

/// 注册角色相关路由
///
/// # 返回值
/// 返回路由列表，类型: [Vec<rocket::Route>]
pub fn routes() -> Vec<rocket::Route> {
    routes![list_roles, add_role, edit_role, delete_role, edit_role_status, get_role_menu_id_list, role_set_menu]
}