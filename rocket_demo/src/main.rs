mod config;
mod controllers;
mod models;
mod params;
mod repositories;
mod services;
mod views;

use controllers::{dept::controller as dept_controller, index::controller as index_controller};
use services::dept::{dept_service::DeptService, dept_service_impl::DeptServiceImpl};
use std::sync::Arc;

// 为每种实现定义类型别名，简化条件编译代码
#[cfg(feature = "sqlx_impl")]
use repositories::dept::sqlx_impl::DeptRepositorySqlxImpl as DeptRepositoryImpl;

#[cfg(feature = "diesel_impl")]
use repositories::dept::diesel_impl::DeptRepositoryDieselImpl as DeptRepositoryImpl;

#[cfg(feature = "seaorm_impl")]
use repositories::dept::seaorm_impl::DeptRepositorySeaormImpl as DeptRepositoryImpl;

// 统一导入trait
use repositories::dept::dept_repository::DeptRepository;

#[rocket::launch]
async fn rocket() -> _ {
    // 根据启用的特性初始化对应的数据访问层实现
    #[cfg(feature = "sqlx_impl")]
    let _repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositoryImpl::new()
                .await
                .expect("无法创建SQLx数据库连接"),
        )
    };

    #[cfg(feature = "diesel_impl")]
    let _repository: Arc<dyn DeptRepository> = { Arc::new(DeptRepositoryImpl::new()) };

    #[cfg(feature = "seaorm_impl")]
    let _repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositoryImpl::new()
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
