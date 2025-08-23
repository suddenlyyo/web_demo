//! 部门控制器

use rocket::{delete, get, post, put, routes, serde::json::Json};

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};

use crate::models::dept::Dept;
use crate::services::dept::dept_service::DeptService;
use crate::services::dept::dept_service_impl::DeptServiceImpl;

/// 根据ID获取部门信息
///
/// # 参数
///
/// - `id`: 部门ID
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的部门信息
#[get("/dept/<id>")]
pub async fn get_dept(id: String, dept_service: &rocket::State<DeptServiceImpl>) -> SingleWrapper<Dept> {
    dept_service.get_dept_by_id(&id).await
}

/// 获取部门列表
///
/// # 参数
///
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的部门列表
#[get("/dept/list")]
pub async fn list_depts(dept_service: &rocket::State<DeptServiceImpl>) -> ListWrapper<Dept> {
    dept_service.list_depts().await
}

/// 分页查询部门列表
///
/// # 参数
///
/// - `page_num`: 页码
/// - `page_size`: 每页大小
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的分页部门列表
#[get("/dept/page?<page_num>&<page_size>")]
pub async fn list_depts_by_page(page_num: Option<u64>, page_size: Option<u64>, dept_service: &rocket::State<DeptServiceImpl>) -> PageWrapper<Dept> {
    dept_service.list_depts_by_page(page_num, page_size).await
}

/// 根据父部门ID获取子部门列表
///
/// # 参数
///
/// - `parent_id`: 父部门ID
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的子部门列表
#[get("/dept/children/<parent_id>")]
pub async fn list_children_by_parent_id(parent_id: String, dept_service: &rocket::State<DeptServiceImpl>) -> ListWrapper<Dept> {
    dept_service.list_children_by_parent_id(&parent_id).await
}

/// 获取部门树结构
///
/// # 参数
///
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的部门树结构
#[get("/dept/tree")]
pub async fn list_dept_tree(dept_service: &rocket::State<DeptServiceImpl>) -> ListWrapper<Dept> {
    dept_service.list_dept_tree().await
}

/// 新增部门
///
/// # 参数
///
/// - `dept`: 部门信息
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的新增结果
#[post("/dept", data = "<dept>")]
pub async fn add_dept(dept: Json<Dept>, dept_service: &rocket::State<DeptServiceImpl>) -> SingleWrapper<Dept> {
    dept_service.add_dept(dept.into_inner()).await
}

/// 修改部门
///
/// # 参数
///
/// - `id`: 部门ID
/// - `dept`: 部门信息
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的修改结果
#[put("/dept/<_id>", data = "<dept>")]
pub async fn update_dept(_id: String, dept: Json<Dept>, dept_service: &rocket::State<DeptServiceImpl>) -> SingleWrapper<Dept> {
    dept_service.update_dept(dept.into_inner()).await
}

/// 删除部门
///
/// # 参数
///
/// - `id`: 部门ID
///
/// # 返回值
///
/// 返回包装后的删除结果
#[delete("/dept/<id>")]
pub async fn delete_dept(id: String, dept_service: &rocket::State<DeptServiceImpl>) -> SingleWrapper<Dept> {
    dept_service.delete_dept(&id).await
}

/// 修改部门状态
///
/// # 参数
///
/// - `id`: 部门ID
/// - `status`: 部门状态
/// - `dept_service`: 部门服务
///
/// # 返回值
///
/// 返回包装后的修改结果
#[put("/dept/<id>/status/<status>")]
pub async fn update_dept_status(id: String, status: i32, dept_service: &rocket::State<DeptServiceImpl>) -> SingleWrapper<Dept> {
    dept_service.update_dept_status(&id, status).await
}

/// 注册部门相关路由
pub fn routes() -> Vec<rocket::Route> {
    routes![get_dept, list_depts, list_depts_by_page, list_children_by_parent_id, list_dept_tree, add_dept, update_dept, delete_dept, update_dept_status]
}
