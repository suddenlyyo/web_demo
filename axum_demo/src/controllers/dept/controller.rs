//! 部门控制器
//!
//! 该模块实现了部门相关的HTTP接口，包括部门列表查询、部门树获取、新增、编辑、删除等操作。
//!
//! # 主要功能
//!
//! - 查询部门列表: [list_depts]
//! - 获取部门树: [get_dept_tree]
//! - 添加部门: [add_dept]
//! - 编辑部门: [edit_dept]
//! - 删除部门: [delete_dept]
//! - 修改部门状态: [edit_dept_status]
//!

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, post, put},
};
use common_wrapper::{ListWrapper, ResponseWrapper};
use std::sync::Arc;

use crate::params::dept_param::DeptParam;
use crate::services::dept::dept_service::DeptService;
use crate::views::{dept_tree::DeptTree, dept_vo::DeptVO};

/// 查询部门列表
///
/// 根据参数查询部门列表信息，返回包含状态描述和父部门名称等额外信息的部门VO列表
///
/// # 参数
///
/// - `dept_param`: 部门查询参数，类型: [Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，通过Axum状态管理提供
///
/// # 返回值
///
/// 返回JSON格式的部门列表结果，类型: [Json]<[ListWrapper]<[DeptVO]>>，参见: [ListWrapper]<[DeptVO]>
pub async fn list_depts(State(dept_service): State<Arc<dyn DeptService + Send + Sync>>, Json(dept_param): Json<DeptParam>) -> Json<ListWrapper<DeptVO>> {
    let result: ListWrapper<DeptVO> = dept_service.select_dept_vo_list(dept_param).await;
    Json(result)
}

/// 获取部门树
///
/// 获取所有部门的树形结构数据
///
/// # 参数
///
/// - `dept_param`: 部门查询参数，类型: [Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，通过Axum状态管理提供
///
/// # 返回值
///
/// 返回JSON格式的部门树结果，类型: [Json]<[ListWrapper]<[DeptTree]>>
pub async fn get_dept_tree(State(dept_service): State<Arc<dyn DeptService + Send + Sync>>, Json(dept_param): Json<DeptParam>) -> Json<ListWrapper<DeptTree>> {
    let result = dept_service.get_dept_tree(dept_param).await;
    Json(result)
}

/// 添加部门
///
/// 新增一个部门信息
///
/// # 参数
///
/// - `dept_param`: 部门参数，类型: [Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，通过Axum状态管理提供
///
/// # 返回值
///
/// 返回操作结果，类型: [Json]<[ResponseWrapper]>
pub async fn add_dept(State(dept_service): State<Arc<dyn DeptService + Send + Sync>>, Json(dept_param): Json<DeptParam>) -> Json<ResponseWrapper> {
    let result = dept_service.add_dept(dept_param).await;
    Json(result)
}

/// 编辑部门
///
/// 修改部门信息
///
/// # 参数
///
/// - `dept_param`: 部门参数，类型: [Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，通过Axum状态管理提供
///
/// # 返回值
///
/// 返回操作结果，类型: [Json]<[ResponseWrapper]>
pub async fn edit_dept(State(dept_service): State<Arc<dyn DeptService + Send + Sync>>, Json(dept_param): Json<DeptParam>) -> Json<ResponseWrapper> {
    let result = dept_service.edit_dept(dept_param).await;
    Json(result)
}

/// 删除部门
///
/// 根据部门ID删除指定部门
///
/// # 参数
///
/// - `dept_id`: 部门ID，通过URL路径传入
/// - `dept_service`: 部门服务实例，通过Axum状态管理提供
///
/// # 返回值
///
/// 返回操作结果，类型: [Json]<[ResponseWrapper]>
pub async fn delete_dept(State(dept_service): State<Arc<dyn DeptService + Send + Sync>>, Path(dept_id): Path<String>) -> Json<ResponseWrapper> {
    let result = dept_service.delete_dept(&dept_id).await;
    Json(result)
}

/// 修改部门状态
///
/// 根据部门ID修改部门状态
///
/// # 参数
///
/// - `id`: 部门ID，通过URL路径传入
/// - `status`: 部门状态，通过URL路径传入
/// - `dept_service`: 部门服务实例，通过Axum状态管理提供
///
/// # 返回值
///
/// 返回操作结果，类型: [Json]<[ResponseWrapper]>
pub async fn edit_dept_status(State(dept_service): State<Arc<dyn DeptService + Send + Sync>>, Path((id, status)): Path<(String, i32)>) -> Json<ResponseWrapper> {
    let result = dept_service.edit_dept_status(&id, status).await;
    Json(result)
}

/// 注册部门相关路由
///
/// 将部门相关路由注册到Axum应用中
///
/// # 返回值
///
/// 返回部门相关路由，类型: [Router]
pub fn routes(dept_service: Arc<dyn DeptService + Send + Sync>) -> Router {
    Router::new()
        .route("/list", post(list_depts))
        .route("/getDeptTree", post(get_dept_tree))
        .route("/add", post(add_dept))
        .route("/edit", put(edit_dept))
        .route("/delete/:dept_id", delete(delete_dept))
        .route("/editStatus/:id/:status", put(edit_dept_status))
        .with_state(dept_service)
}
