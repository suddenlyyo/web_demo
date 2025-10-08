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
use services::dept::{dept_service::DeptService, dept_service_impl::DeptServiceImpl};

// 为每种实现定义类型别名，简化条件编译代码
#[cfg(feature = "sqlx_impl")]
use repositories::dept::sqlx_impl::DeptRepositorySqlxImpl as DeptRepositoryImpl;

#[cfg(feature = "diesel_impl")]
use repositories::dept::diesel_impl::DeptRepositoryDieselImpl as DeptRepositoryImpl;

#[cfg(feature = "seaorm_impl")]
use repositories::dept::seaorm_impl::DeptRepositorySeaormImpl as DeptRepositoryImpl;

// 统一导入trait
use actix_web::{App, HttpServer, web};
use repositories::dept::dept_repository::DeptRepository;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 根据启用的特性初始化对应的数据访问层实现
    #[cfg(feature = "diesel_impl")]
    let repository: Arc<dyn DeptRepository> = { Arc::new(DeptRepositoryImpl::new().expect("无法创建Diesel数据库连接")) };

    #[cfg(feature = "seaorm_impl")]
    let repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositoryImpl::new()
                .await
                .expect("无法创建SeaORM数据库连接"),
        )
    };

    #[cfg(feature = "sqlx_impl")]
    let repository: Arc<dyn DeptRepository> = {
        Arc::new(
            DeptRepositoryImpl::new()
                .await
                .expect("无法创建SQLx数据库连接"),
        )
    };

    // 从环境变量中读取主机和端口配置，默认为127.0.0.1:8080
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let bind_address = format!("{}:{}", host, port);

    // 检查是否使用了默认值
    let host_source = if std::env::var("HOST").is_ok() { "环境变量" } else { "默认值" };
    let port_source = if std::env::var("PORT").is_ok() { "环境变量" } else { "默认值" };

    println!("Starting server at {bind_address} (host: {host} from {host_source}, port: {port} from {port_source})");

    // 初始化部门服务
    let dept_service = Box::new(DeptServiceImpl::new(repository)) as Box<dyn DeptService + Send + Sync>;
    let dept_service_data = web::Data::new(dept_service);

    HttpServer::new(move || {
        App::new()
            .app_data(dept_service_data.clone())
            .configure(dept_controller::config)
            .configure(index_controller::config)
    })
    .bind(bind_address)?
    .run()
    .await
}
