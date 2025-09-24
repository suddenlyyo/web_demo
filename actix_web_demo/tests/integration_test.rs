//! 集成测试
//!
//! 使用 Actix Web 提供的测试工具进行集成测试

use actix_web::{
    App,
    http::StatusCode,
    test::{self, TestRequest},
};
use serde_json::json;

use actix_web_demo::{
    controllers::{dept::controller as dept_controller, index::controller as index_controller},
    repositories::dept::{dept_repository::DeptRepository, sqlx_impl::DeptRepositorySqlxImpl},
    services::dept::{dept_service::DeptService, dept_service_impl::DeptServiceImpl},
};
use std::sync::Arc;

/// 测试首页路由
#[actix_web::test]
async fn test_index() {
    // 初始化测试服务
    let app = test::init_service(App::new().configure(index_controller::config)).await;

    // 创建测试请求
    let req = TestRequest::get().uri("/").to_request();

    // 发送请求并获取响应
    let resp = test::call_service(&app, req).await;

    // 验证响应状态
    assert_eq!(resp.status(), StatusCode::OK);
}

/// 测试部门列表接口
#[actix_web::test]
async fn test_dept_list() {
    // 注意：由于数据库依赖，这个测试可能需要mock repository才能正常运行
    // 这里提供一个基本的测试框架示例

    /*
    // 创建mock repository（实际使用中可能需要mock框架）
    let repository = Arc::new(MockDeptRepository::new()) as Arc<dyn DeptRepository>;
    let dept_service = Box::new(DeptServiceImpl::new(repository)) as Box<dyn DeptService + Send + Sync>;
    let dept_service_data = actix_web::web::Data::new(dept_service);

    // 初始化测试服务
    let app = test::init_service(
        App::new()
            .app_data(dept_service_data)
            .configure(dept_controller::config)
    ).await;

    // 创建测试数据
    let dept_param = json!({
        "parentId": null,
        "name": null,
        "email": null,
        "telephone": null,
        "address": null,
        "logo": null,
        "deptLevel": null,
        "seqNo": null,
        "status": null,
        "createBy": null,
        "createTime": null,
        "updateBy": null,
        "updateTime": null,
        "remark": null,
        "pageNum": 1,
        "pageSize": 10
    });

    // 创建测试请求
    let req = TestRequest::post()
        .uri("/dept/list")
        .set_json(&dept_param)
        .to_request();

    // 发送请求并获取响应
    let resp = test::call_service(&app, req).await;

    // 验证响应状态
    assert_eq!(resp.status(), StatusCode::OK);
    */
}

/// 测试添加部门接口
#[actix_web::test]
async fn test_add_dept() {
    // 同样，这个测试需要mock repository

    /*
    // 创建mock repository
    let repository = Arc::new(MockDeptRepository::new()) as Arc<dyn DeptRepository>;
    let dept_service = Box::new(DeptServiceImpl::new(repository)) as Box<dyn DeptService + Send + Sync>;
    let dept_service_data = actix_web::web::Data::new(dept_service);

    // 初始化测试服务
    let app = test::init_service(
        App::new()
            .app_data(dept_service_data)
            .configure(dept_controller::config)
    ).await;

    // 创建测试数据
    let dept_data = json!({
        "parentId": "065a3eb180214ccfbb653f63287d285d",
        "name": "测试部门",
        "seqNo": 1,
        "telephone": "13800000000",
        "email": "test@example.com",
        "status": 1,
        "createBy": "test_user"
    });

    // 创建测试请求
    let req = TestRequest::post()
        .uri("/dept/add")
        .set_json(&dept_data)
        .to_request();

    // 发送请求并获取响应
    let resp = test::call_service(&app, req).await;

    // 验证响应状态
    assert_eq!(resp.status(), StatusCode::OK);
    */
}

/// 测试编辑部门接口
#[actix_web::test]
async fn test_edit_dept() {
    // 同样，这个测试需要mock repository

    /*
    // 创建mock repository
    let repository = Arc::new(MockDeptRepository::new()) as Arc<dyn DeptRepository>;
    let dept_service = Box::new(DeptServiceImpl::new(repository)) as Box<dyn DeptService + Send + Sync>;
    let dept_service_data = actix_web::web::Data::new(dept_service);

    // 初始化测试服务
    let app = test::init_service(
        App::new()
            .app_data(dept_service_data)
            .configure(dept_controller::config)
    ).await;

    // 创建测试数据
    let dept_data = json!({
        "id": "test_dept_id",
        "parentId": "065a3eb180214ccfbb653f63287d285d",
        "name": "更新测试部门",
        "seqNo": 2,
        "telephone": "13900000000",
        "email": "update@example.com",
        "status": 1,
        "updateBy": "test_user"
    });

    // 创建测试请求
    let req = TestRequest::put()
        .uri("/dept/edit")
        .set_json(&dept_data)
        .to_request();

    // 发送请求并获取响应
    let resp = test::call_service(&app, req).await;

    // 验证响应状态
    assert_eq!(resp.status(), StatusCode::OK);
    */
}
