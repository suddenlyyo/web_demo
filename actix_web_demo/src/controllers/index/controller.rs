//! 首页控制器层

use actix_web::{HttpResponse, Responder, get, web};

/// 根路径处理函数
///
/// # 返回值
///
/// 返回"Hello, world!"字符串
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// 注册首页相关路由
///
/// 将首页相关路由注册到Actix Web应用中
///
/// # 参数
///
/// - `cfg`: 服务配置，类型: &mut [web::ServiceConfig]
///
/// # 返回值
///
/// 无返回值，直接修改服务配置
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/").service(hello));
}
