//! 角色控制器层

use rocket::routes;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use serde_json::Value;

use common_wrapper::{ResponseWrapper, SingleWrapper};
use std::collections::HashSet;

use crate::models::role::Role;
use crate::params::role_param::RoleParam;
use crate::services::role::role_service::RoleService;
use crate::services::role::role_service_impl::RoleServiceImpl;

/// 角色控制器

/// 获取角色列表
///
/// # 参数
/// * `role_param` - 角色参数，类型: [RoleParam]
/// * `role_service` - 角色服务实例，类型: [&rocket::State<RoleServiceImpl>]
///
/// # 返回值
/// 返回角色列表，类型: [serde_json::Value]
#[get("/role/list")]
pub async fn list_roles(role_param: RoleParam, role_service: &rocket::State<RoleServiceImpl>) -> Json<Value> {
    let result = role_service.select_role_list(role_param).await;
    // 构造返回的JSON数据
    let mut response_data = serde_json::Map::new();
    response_data.insert("code".to_string(), Value::Number(result.get_code().into()));
    response_data.insert("msg".to_string(), Value::String(result.get_message().to_string()));
    
    if let Some(roles) = result.data {
        let roles_json: Vec<Value> = roles.into_iter().map(|role| {
            let mut role_map = serde_json::Map::new();
            role_map.insert("id".to_string(), Value::String(role.id));
            role_map.insert("name".to_string(), Value::String(role.name));
            role_map.insert("role_key".to_string(), Value::String(role.role_key));
            role_map.insert("status".to_string(), Value::Number(role.status.into()));
            // TODO: 添加statusDesc字段
            Value::Object(role_map)
        }).collect();
        
        response_data.insert("data".to_string(), Value::Array(roles_json));
    } else {
        response_data.insert("data".to_string(), Value::Array(vec![]));
    }
    
    Json(Value::Object(response_data))
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
    // 解析JSON字符串
    match serde_json::from_str::<serde_json::Value>(&json_str) {
        Ok(json_value) => {
            if let Some(role_id) = json_value.get("roleId").and_then(|v| v.as_str()) {
                if let Some(menu_ids_array) = json_value.get("menuIds").and_then(|v| v.as_array()) {
                    let menu_ids: Vec<&str> = menu_ids_array.iter()
                        .filter_map(|v| v.as_str())
                        .collect();
                    return role_service.role_set_menu(role_id, &menu_ids).await;
                }
            }
            ResponseWrapper::new(500, "参数格式错误")
        },
        Err(_) => ResponseWrapper::new(500, "JSON解析失败")
    }
}

/// 注册角色相关路由
///
/// # 返回值
/// 返回路由列表，类型: [Vec<rocket::Route>]
pub fn routes() -> Vec<rocket::Route> {
    routes![list_roles, add_role, edit_role, delete_role, edit_role_status, get_role_menu_id_list, role_set_menu]
}
