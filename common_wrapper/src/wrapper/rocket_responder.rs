//! Rocket Responder 实现模块
//!
//! 为 common_wrapper 中的包装器类型提供 Rocket Web 框架的 Responder 实现
//! 这个模块是可选的，只有在启用 rocket_responder 特性时才会编译

#[cfg(feature = "rocket_responder")]
use rocket::response::Responder;
#[cfg(feature = "rocket_responder")]
use rocket::serde::json::Json;
#[cfg(feature = "rocket_responder")]
use serde::Serialize;

use crate::wrapper::{ListWrapper, PageWrapper, SingleWrapper};

/// 为 SingleWrapper 实现 Rocket Responder
#[cfg(feature = "rocket_responder")]
impl<'r, T: Serialize> Responder<'r, 'static> for SingleWrapper<T> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = Json(self);
        json.respond_to(request)
    }
}

/// 为 ListWrapper 实现 Rocket Responder
#[cfg(feature = "rocket_responder")]
impl<'r, T: Serialize> Responder<'r, 'static> for ListWrapper<T> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = Json(self);
        json.respond_to(request)
    }
}

/// 为 PageWrapper 实现 Rocket Responder
#[cfg(feature = "rocket_responder")]
impl<'r, T: Serialize> Responder<'r, 'static> for PageWrapper<T> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = Json(self);
        json.respond_to(request)
    }
}
