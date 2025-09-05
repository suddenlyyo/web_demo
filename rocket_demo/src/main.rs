//! Rocket Web框架演示程序入口
//!
//! 本程序演示了如何使用Rocket框架构建Web应用，包括：
//! - 用户管理（增删改查）
//! - 部门管理（增删改查）
//! - 角色管理（增删改查）
//! - 菜单管理（增删改查）

use rocket::Config;
use rocket::figment::Figment;

mod controllers;
mod models;
mod params;
mod repositories;
mod services;

/// 程序入口点
#[rocket::launch]
fn rocket() -> _ {
    // 创建默认配置
    let figment = Figment::from(Config::default());

    // 构建Rocket实例并挂载路由
    rocket::custom(figment)
        // 挂载控制器路由
        .mount("/", controllers::index::controller::routes())
        .mount("/user", controllers::user::controller::routes())
        .mount("/dept", controllers::dept::controller::routes())
        .mount("/role", controllers::role::controller::routes())
        .mount("/menu", controllers::menu::controller::routes())
}
