//! 角色控制器层

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};
use rocket::{delete, get, post, put, routes, serde::json::Json};

use crate::models::role::Role;
use crate::services::role::role_service::RoleService;
use crate::services::role::role_service_impl::RoleServiceImpl;

/// 根据ID获取角色信息
///
/// # 参数
///
/// - `id`: 角色ID
/// - `role_service`: 角色服务
///
/// # 返回值
///
/// 返回包装后的角色信息
#[get("/role/<id>")]
pub async fn get_role(id: String, role_service: &rocket::State<RoleServiceImpl>) -> SingleWrapper<Role> {
    role_service.get_role_by_id(&id).await
}

/// 获取角色列表
///
/// # 参数
///
/// - `role_service`: 角色服务
///
/// # 返回值
///
/// 返回包装后的角色列表
#[get("/role/list")]
pub async fn list_roles(role_service: &rocket::State<RoleServiceImpl>) -> ListWrapper<Role> {
    role_service.list_roles().await
}

/// 分页查询角色列表
///
/// # 参数
///
/// - `page_num`: 页码
/// - `page_size`: 每页大小
/// - `role_service`: 角色服务
///
/// # 返回值
///
/// 返回包装后的分页角色列表
#[get("/role/page?<page_num>&<page_size>")]
pub async fn list_roles_by_page(page_num: Option<u64>, page_size: Option<u64>, role_service: &rocket::State<RoleServiceImpl>) -> PageWrapper<Role> {
    role_service.list_roles_by_page(page_num, page_size).await
}

/// 新增角色
///
/// # 参数
///
/// - `role`: 角色信息
/// - `role_service`: 角色服务
///
/// # 返回值
///
/// 返回包装后的新增结果
#[post("/role", data = "<role>")]
pub async fn add_role(role: Json<Role>, role_service: &rocket::State<RoleServiceImpl>) -> SingleWrapper<Role> {
    role_service.add_role(role.into_inner()).await
}

/// 修改角色
///
/// # 参数
///
/// - `id`: 角色ID
/// - `role`: 角色信息
/// - `role_service`: 角色服务
///
/// # 返回值
///
/// 返回包装后的修改结果
#[put("/role/<_id>", data = "<role>")]
pub async fn update_role(_id: String, role: Json<Role>, role_service: &rocket::State<RoleServiceImpl>) -> SingleWrapper<Role> {
    role_service.update_role(role.into_inner()).await
}

/// 删除角色
///
/// # 参数
///
/// - `id`: 角色ID
///
/// # 返回值
///
/// 返回包装后的删除结果
#[delete("/role/<id>")]
pub async fn delete_role(id: String, role_service: &rocket::State<RoleServiceImpl>) -> SingleWrapper<Role> {
    role_service.delete_role(&id).await
}

/// 修改角色状态
///
/// # 参数
///
/// - `id`: 角色ID
/// - `status`: 角色状态
/// - `role_service`: 角色服务
///
/// # 返回值
///
/// 返回包装后的修改结果
#[put("/role/<id>/status/<status>")]
pub async fn update_role_status(id: String, status: i32, role_service: &rocket::State<RoleServiceImpl>) -> SingleWrapper<Role> {
    role_service.update_role_status(&id, status).await
}

/// 注册角色相关路由
pub fn routes() -> Vec<rocket::Route> {
    routes![get_role, list_roles, list_roles_by_page, add_role, update_role, delete_role, update_role_status]
}
