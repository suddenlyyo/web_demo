use actix_web::{App, HttpServer};
use controller::*;
use repository::UserRepository;
use service::UserService;
use std::sync::Arc;

mod controller;
mod model;
mod repository;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化仓储层
    let repository = UserRepository::new();
    
    // 初始化服务层
    let service = UserService::new(repository);
    
    // 将服务包装为线程安全的共享状态
    let service_data = Arc::new(service);
    
    println!("Server running at http://localhost:8080");
    
    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(service_data.clone()))
            .service(get_all_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}