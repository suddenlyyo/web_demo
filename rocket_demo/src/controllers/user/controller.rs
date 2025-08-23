//! 用户控制器

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};
use rocket::{delete, get, post, put, routes, serde::json::Json};

use crate::models::user::{User, UserQuery};
use crate::services::user::user_service::UserService;
use crate::services::user::user_service_impl::UserServiceImpl;

/// 根据ID获取用户信息
///
/// # 参数
///
/// * `id` - 用户ID
/// * `user_service` - 用户服务
///
/// # 返回值
///
/// 返回包装后的用户信息
#[get("/user/<id>")]
pub async fn get_user(id: String, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<User> {
    user_service.get_user_by_id(&id).await
}

/// 获取用户列表
///
/// # 参数
///
/// * `user_service` - 用户服务
///
/// # 返回值
///
/// 返回包装后的用户列表
#[get("/user/list")]
pub async fn list_users(user_service: &rocket::State<UserServiceImpl>) -> ListWrapper<User> {
    user_service.list_users().await
}

/// 分页查询用户列表
///
/// # 参数
///
/// * `query` - 查询条件
/// * `user_service` - 用户服务
///
/// # 返回值
///
/// 返回包装后的分页用户列表
#[post("/user/query", data = "<query>")]
pub async fn list_users_by_query(query: Json<UserQuery>, user_service: &rocket::State<UserServiceImpl>) -> PageWrapper<User> {
    let page_num = query.current_page_num;
    let page_size = query.page_size;
    user_service.list_users_by_page(page_num, page_size).await
}

/// 新增用户
///
/// # 参数
///
/// * `user` - 用户信息
/// * `user_service` - 用户服务
///
/// # 返回值
///
/// 返回包装后的用户信息
#[post("/user", data = "<user>")]
pub async fn add_user(user: Json<User>, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<User> {
    user_service.add_user(user.into_inner()).await
}

/// 修改用户
///
/// # 参数
///
/// * `id` - 用户ID
/// * `user` - 用户信息
/// * `user_service` - 用户服务
///
/// # 返回值
///
/// 返回包装后的用户信息
#[put("/user/<_id>", data = "<user>")]
pub async fn update_user(_id: String, user: Json<User>, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<User> {
    user_service.update_user(user.into_inner()).await
}

/// 删除用户
///
/// # 参数
///
/// * `id` - 用户ID
/// * `user_service` - 用户服务
///
/// # 返回值
///
/// 返回包装后的用户信息
#[delete("/user/<id>")]
pub async fn delete_user(id: String, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<User> {
    user_service.delete_user(&id).await
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
pub async fn update_user_status(id: String, status: i32, user_service: &rocket::State<UserServiceImpl>) -> SingleWrapper<User> {
    user_service.update_user_status(&id, status).await
}

/// 注册用户相关路由
pub fn routes() -> Vec<rocket::Route> {
    routes![get_user, list_users, list_users_by_query, add_user, update_user, delete_user, update_user_status]
}
