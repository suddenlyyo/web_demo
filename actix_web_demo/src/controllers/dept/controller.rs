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

use actix_web::{HttpResponse, Responder, delete, post, put, web};
use common_wrapper::ListWrapper;

use crate::params::dept_param::DeptParam;
use crate::services::dept::dept_service::DeptService;
use crate::views::dept_vo::DeptVO;

/// 查询部门列表
///
/// 根据参数查询部门列表信息，返回包含状态描述和父部门名称等额外信息的部门VO列表
///
/// # 参数
///
/// - `dept_param`: 部门查询参数，类型: [web::Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，类型: [web::Data]<[Box]<dyn [DeptService] + Send + Sync>>，通过Actix Web依赖注入提供
///
/// # 返回值
///
/// 返回JSON格式的部门列表结果，类型: [HttpResponse]，包含: [ListWrapper]<[DeptVO]>
#[post("/dept/list")]
pub async fn list_depts(dept_param: web::Json<DeptParam>, dept_service: web::Data<Box<dyn DeptService + Send + Sync>>) -> impl Responder {
    let result: ListWrapper<DeptVO> = dept_service
        .select_dept_vo_list(dept_param.into_inner())
        .await;
    HttpResponse::Ok().json(result)
}

/// 获取部门树
///
/// 获取所有部门的树形结构数据
///
/// # 参数
///
/// - `dept_param`: 部门查询参数，类型: [web::Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，类型: [web::Data]<[Box]<dyn [DeptService] + Send + Sync>>，通过Actix Web依赖注入提供
///
/// # 返回值
///
/// 返回JSON格式的部门树结果，类型: [HttpResponse]，包含: [ListWrapper]<[DeptTree]>
#[post("/dept/getDeptTree")]
pub async fn get_dept_tree(dept_param: web::Json<DeptParam>, dept_service: web::Data<Box<dyn DeptService + Send + Sync>>) -> impl Responder {
    let result = dept_service.get_dept_tree(dept_param.into_inner()).await;
    HttpResponse::Ok().json(result)
}

/// 添加部门
///
/// 新增一个部门信息
///
/// # 参数
///
/// - `dept_param`: 部门参数，类型: [web::Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，类型: [web::Data]<[Box]<dyn [DeptService] + Send + Sync>>，通过Actix Web依赖注入提供
///
/// # 返回值
///
/// 返回操作结果，类型: [HttpResponse]，包含: [ResponseWrapper]
#[post("/dept/add")]
pub async fn add_dept(dept_param: web::Json<DeptParam>, dept_service: web::Data<Box<dyn DeptService + Send + Sync>>) -> impl Responder {
    let result = dept_service.add_dept(dept_param.into_inner()).await;
    HttpResponse::Ok().json(result)
}

/// 编辑部门
///
/// 修改部门信息
///
/// # 参数
///
/// - `dept_param`: 部门参数，类型: [web::Json]<[DeptParam]>，通过请求体传入
/// - `dept_service`: 部门服务实例，类型: [web::Data]<[Box]<dyn [DeptService] + Send + Sync>>，通过Actix Web依赖注入提供
///
/// # 返回值
///
/// 返回操作结果，类型: [HttpResponse]，包含: [ResponseWrapper]
#[put("/dept/edit")]
pub async fn edit_dept(dept_param: web::Json<DeptParam>, dept_service: web::Data<Box<dyn DeptService + Send + Sync>>) -> impl Responder {
    let result = dept_service.edit_dept(dept_param.into_inner()).await;
    HttpResponse::Ok().json(result)
}

/// 删除部门
///
/// 根据部门ID删除指定部门
///
/// # 参数
///
/// - `path`: 路径参数，包含部门ID，类型: [web::Path]<(String,)>，通过URL路径传入
/// - `dept_service`: 部门服务实例，类型: [web::Data]<[Box]<dyn [DeptService] + Send + Sync>>，通过Actix Web依赖注入提供
///
/// # 返回值
///
/// 返回操作结果，类型: [HttpResponse]，包含: [ResponseWrapper]
#[delete("/dept/delete/{dept_id}")]
pub async fn delete_dept(path: web::Path<(String,)>, dept_service: web::Data<Box<dyn DeptService + Send + Sync>>) -> impl Responder {
    let dept_id = path.into_inner().0;
    let result = dept_service.delete_dept(&dept_id).await;
    HttpResponse::Ok().json(result)
}

/// 修改部门状态
///
/// 根据部门ID修改部门状态
///
/// # 参数
///
/// - `path`: 路径参数，包含部门ID和状态，类型: [web::Path]<(String, i32)>，通过URL路径传入
/// - `dept_service`: 部门服务实例，类型: [web::Data]<[Box]<dyn [DeptService] + Send + Sync>>，通过Actix Web依赖注入提供
///
/// # 返回值
///
/// 返回操作结果，类型: [HttpResponse]，包含: [ResponseWrapper]
#[put("/dept/editStatus/{id}/{status}")]
pub async fn edit_dept_status(path: web::Path<(String, i32)>, dept_service: web::Data<Box<dyn DeptService + Send + Sync>>) -> impl Responder {
    let (id, status) = path.into_inner();
    let result = dept_service.edit_dept_status(&id, status).await;
    HttpResponse::Ok().json(result)
}

/// 注册部门相关路由
///
/// 将部门相关路由注册到Actix Web应用中
///
/// # 参数
///
/// - `cfg`: 服务配置，类型: &mut [web::ServiceConfig]
///
/// # 返回值
///
/// 无返回值，直接修改服务配置
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/dept")
            .service(list_depts)
            .service(get_dept_tree)
            .service(add_dept)
            .service(edit_dept)
            .service(delete_dept)
            .service(edit_dept_status),
    );
}
