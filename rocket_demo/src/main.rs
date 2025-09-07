#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod models;
mod params;
mod repositories;
mod services;
mod views;

use {
    controllers::{dept::controller as dept_controller, index::controller as index_controller},
    repositories::dept::{dept_repository::DeptRepository, diesel_impl::DeptRepositoryDieselImpl, seaorm_impl::DeptRepositorySeaormImpl, sqlx_impl::DeptRepositorySqlxImpl},
    services::dept::{dept_service::DeptService, dept_service_impl::DeptServiceImpl},
    std::sync::Arc,
};

#[rocket::launch]
async fn rocket() -> _ {
    // 根据启用的特性初始化对应的数据访问层实现
    #[cfg(feature = "sqlx_impl")]
    let _repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositorySqlxImpl::new()
                .await
                .expect("无法创建SQLx数据库连接"),
        )
    };

    #[cfg(feature = "diesel_impl")]
    let _repository: Arc<dyn DeptRepository> = { Arc::new(DeptRepositoryDieselImpl::new()) };

    #[cfg(feature = "seaorm_impl")]
    let _repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositorySeaormImpl::new()
                .await
                .expect("无法创建SeaORM数据库连接"),
        )
    };

    // 初始化部门服务
    let dept_service: Arc<dyn DeptService> = Arc::new(DeptServiceImpl::new(_repository));

    // 构建Rocket实例
    rocket::build()
        .manage(dept_service)
        .mount("/", index_controller::routes())
        .mount("/dept", dept_controller::routes())
}
