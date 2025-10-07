mod config;
mod controllers;
mod models;
mod params;
mod repositories;
mod services;
mod views;

// 只在使用SeaORM时才导入entities模块
#[cfg(feature = "seaorm_impl")]
mod entities;
// 只在使用Diesel时才导入schema模块
#[cfg(feature = "diesel_impl")]
mod schema;

use controllers::{dept::controller as dept_controller, index::controller as index_controller};
use rocket::figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use services::dept::{dept_service::DeptService, dept_service_impl::DeptServiceImpl};

// 为每种实现定义类型别名，简化条件编译代码
#[cfg(feature = "sqlx_impl")]
use repositories::dept::sqlx_impl::DeptRepositorySqlxImpl as DeptRepositoryImpl;

#[cfg(feature = "diesel_impl")]
use repositories::dept::diesel_impl::DeptRepositoryDieselImpl as DeptRepositoryImpl;

#[cfg(feature = "seaorm_impl")]
use repositories::dept::seaorm_impl::DeptRepositorySeaormImpl as DeptRepositoryImpl;

// 统一导入trait
use repositories::dept::dept_repository::DeptRepository;
use std::sync::Arc;

#[rocket::launch]
async fn rocket() -> _ {
    // 根据启用的特性初始化对应的数据访问层实现
    #[cfg(feature = "sqlx_impl")]
    let repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositoryImpl::new()
                .await
                .expect("无法创建SQLx数据库连接"),
        )
    };

    #[cfg(feature = "diesel_impl")]
    let repository: Arc<dyn DeptRepository> = { Arc::new(DeptRepositoryImpl::new()) };

    #[cfg(feature = "seaorm_impl")]
    let repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositoryImpl::new()
                .await
                .expect("无法创建SeaORM数据库连接"),
        )
    };

    // 初始化部门服务
    let dept_service = Box::new(DeptServiceImpl::new(repository)) as Box<dyn DeptService + Send + Sync>;

    // 创建自定义配置，配置优先级从低到高为：
    // 1. Rocket框架内置默认值（如address=127.0.0.1, port=8000）
    // 2. Rocket.toml配置文件中的值
    // 3. 环境变量ROCKET_*（优先级最高）
    let figment = Figment::from(rocket::Config::default())
        .merge(Toml::file("Rocket.toml").nested())
        .merge(Env::prefixed("ROCKET_").global());

    // 构建Rocket实例
    rocket::custom(figment)
        .manage(dept_service)
        .mount("/", index_controller::routes())
        .mount("/dept", dept_controller::routes())
}
