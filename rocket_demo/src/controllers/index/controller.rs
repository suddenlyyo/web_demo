//! 首页控制器层

use rocket::*;

/// 根路径处理函数
///
/// # 返回值
///
/// 返回"Hello, world!"字符串
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

/// 注册首页相关路由
pub fn routes() -> Vec<rocket::Route> {
    routes![index]
}
