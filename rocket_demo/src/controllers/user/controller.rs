//! 用户控制器

use rocket::{State, routes, serde::json::Json};
use rocket::{delete, get, post, put};

use std::collections::{HashMap, HashSet};

use crate::models::user::User;
use crate::params::user_param::UserParam;
use crate::services::user::user_service::UserService;
use crate::services::user::user_service_impl::UserServiceImpl;
use common_wrapper::{ListWrapper, PageWrapper, ResponseWrapper, SingleWrapper};

/// 用户控制器

/// 获取用户信息
#[get("/user/getInfo")]
pub async fn get_info(user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<HashMap<String, serde_json::Value>> {
    let mut wrapper = SingleWrapper::new();
    // TODO: 实现获取当前用户信息的逻辑
    wrapper.set_success(HashMap::new());
    wrapper
}

/// 获取指定用户信息
///
/// # 参数
/// * `user_name` - 用户名，类型: [String]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回指定用户信息，类型: [SingleWrapper<serde_json::Value>]
#[get("/user/info/<user_name>")]
pub async fn get_user_info(user_name: String, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<serde_json::Value> {
    let mut wrapper = SingleWrapper::new();
    // TODO: 实现获取指定用户信息的逻辑
    wrapper.set_success(serde_json::Value::Null);
    wrapper
}

/// 获取用户列表
///
/// # 参数
/// * `user_param` - 用户参数，类型: [Json<UserParam>]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回分页用户列表，类型: [PageWrapper<serde_json::Value>]
#[post("/user/list", data = "<user_param>")]
pub async fn list_users(user_param: Json<UserParam>, user_service: &rocket::State<UserServiceImpl>) -> PageWrapper<serde_json::Value> {
    user_service
        .get_user_list_by_page(user_param.into_inner())
        .await
}

/// 新增用户
///
/// # 参数
/// * `user_param` - 用户参数，类型: [Json<UserParam>]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[post("/user/addUser", data = "<user_param>")]
pub async fn add_user(user_param: Json<UserParam>, user_service: &rocket::State<UserServiceImpl>) -> ResponseWrapper {
    user_service.add_user(user_param.into_inner()).await
}

/// 编辑用户
///
/// # 参数
/// * `user_param` - 用户参数，类型: [Json<UserParam>]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[put("/user/editUser", data = "<user_param>")]
pub async fn edit_user(user_param: Json<UserParam>, user_service: &rocket::State<UserServiceImpl>) -> ResponseWrapper {
    user_service.edit_user(user_param.into_inner()).await
}

/// 修改用户状态
///
/// # 参数
/// * `user_param` - 用户参数，类型: [Json<UserParam>]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[put("/user/editUserStatus", data = "<user_param>")]
pub async fn edit_user_status(user_param: Json<UserParam>, user_service: &rocket::State<UserServiceImpl>) -> ResponseWrapper {
    if let (Some(id), Some(status)) = (user_param.id.as_ref(), user_param.status) {
        user_service.edit_user_status(id, status).await
    } else {
        let mut response = ResponseWrapper::new(-1, "");
        response.set_fail("参数不完整");
        response
    }
}

/// 删除用户
///
/// # 参数
/// * `user_id` - 用户ID，类型: [String]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[delete("/user/deleteUser/<user_id>")]
pub async fn delete_user(user_id: String, user_service: &rocket::State<UserServiceImpl>) -> ResponseWrapper {
    user_service.delete_user(&user_id).await
}

/// 重置密码
///
/// # 参数
/// * `user_param` - 用户参数，类型: [Json<UserParam>]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[put("/user/resetUserPwd", data = "<user_param>")]
pub async fn reset_user_pwd(user_param: Json<UserParam>, user_service: &rocket::State<UserServiceImpl>) -> ResponseWrapper {
    user_service.reset_user_pwd(user_param.into_inner()).await
}

/// 获取用户的角色信息
///
/// # 参数
/// * `user_id` - 用户ID，类型: [String]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回用户角色ID集合，类型: [SingleWrapper<std::collections::HashSet<String>>]
#[get("/user/getUserRoleIdList/<user_id>")]
pub async fn get_user_role_id_list(user_id: String, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<std::collections::HashSet<String>> {
    user_service.select_role_ids_by_user_id(&user_id).await
}

/// 分配角色
///
/// # 参数
/// * `json_str` - JSON字符串，类型: [String]
/// * `user_service` - 用户服务实例，类型: [&rocket::State<UserServiceImpl>]
///
/// # 返回值
/// 返回响应结果，类型: [ResponseWrapper]
#[put("/user/setUserRole", data = "<json_str>")]
pub async fn set_user_role(json_str: String, user_service: &rocket::State<UserServiceImpl>) -> ResponseWrapper {
    // TODO: 解析JSON并调用服务方法
    user_service.set_user_role("", &[]).await
}

/// 更新用户状态
///
/// # 参数
///
/// * `id` - 用户ID
/// * `status` - 状态值
/// * `user_service` - 用户服务
///
/// # 返回值
///
/// 返回包装后的用户信息
#[put("/user/<id>/status/<status>")]
pub async fn update_user_status(id: String, status: i32, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<crate::models::User> {
    // TODO: 实现更新用户状态的逻辑
    SingleWrapper::new()
}

/// 注册用户相关路由
///
/// # 返回值
/// 返回路由列表，类型: [Vec<rocket::Route>]
pub fn routes() -> Vec<rocket::Route> {
    routes![get_info, get_user_info, list_users, add_user, edit_user, edit_user_status, delete_user, reset_user_pwd, get_user_role_id_list, set_user_role, update_user_status]
}
