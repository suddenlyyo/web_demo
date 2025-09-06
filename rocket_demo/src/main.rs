#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod models;
mod params;
mod repositories;
mod services;
mod views;

use rocket::State;
use std::sync::Arc;

use repositories::dept::dept_repository::DeptRepository;
use services::dept::dept_service::DeptService;
use services::dept::dept_service_impl::DeptServiceImpl;

#[cfg(feature = "sqlx_impl")]
use repositories::dept::sqlx_impl::DeptRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use repositories::dept::diesel_impl::DeptRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use repositories::dept::seaorm_impl::DeptRepositorySeaormImpl;

#[launch]
async fn rocket() -> _ {
    // 初始化部门仓库
    #[cfg(feature = "sqlx_impl")]
    let repository: Arc<dyn DeptRepository> = Arc::new(DeptRepositorySqlxImpl::new());

    #[cfg(feature = "diesel_impl")]
    let repository: Arc<dyn DeptRepository> = Arc::new(DeptRepositoryDieselImpl::new());

    #[cfg(feature = "seaorm_impl")]
    let repository: Arc<dyn DeptRepository> = Arc::new(DeptRepositorySeaormImpl::new().await.unwrap());

    // 初始化部门服务
    let dept_service: Arc<dyn DeptService> = Arc::new(DeptServiceImpl::new(repository));

    // 构建Rocket实例
    rocket::build()
        // 注册服务
        .manage(dept_service)
        // 注册控制器
        .mount(
            "/",
            routes![
                controllers::index::controller::index,
                controllers::dept::controller::list_depts,
                controllers::dept::controller::get_dept_tree,
                controllers::dept::controller::add_dept,
                controllers::dept::controller::edit_dept,
                controllers::dept::controller::delete_dept,
                controllers::dept::controller::edit_dept_status,
            ],
        )
}
