//! 首页控制器层

use axum::{Router, routing::get};

/// 根路径处理函数
///
/// # 返回值
///
/// 返回"Hello, world!"字符串
pub async fn index() -> &'static str {
    "Hello, world!"
}

/// 注册首页相关路由
pub fn routes() -> Router {
    Router::new().route("/", get(index))
}
