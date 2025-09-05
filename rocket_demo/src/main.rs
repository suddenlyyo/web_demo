#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod models;
mod params;
mod repositories;
mod services;

use rocket::fairing::AdHoc;
use services::role::role_service_impl::RoleServiceImpl;
use services::user::user_service_impl::UserServiceImpl;

#[launch]
async fn rocket() -> _ {
    // 构建Rocket实例
    rocket::build()
        // 注册控制器
        .mount(
            "/",
            routes![
                controllers::index::controller::index,
                controllers::user::controller::add_user,
                controllers::user::controller::edit_user,
                controllers::user::controller::edit_user_status,
                controllers::user::controller::delete_user,
                controllers::user::controller::get_user_list_by_page,
                controllers::user::controller::reset_user_pwd,
                controllers::user::controller::set_user_role,
                controllers::user::controller::select_role_ids_by_user_id,
                controllers::dept::controller::get_dept_list,
                controllers::menu::controller::get_menu_list,
                controllers::role::controller::get_role_list_by_page,
                controllers::role::controller::add_role,
                controllers::role::controller::edit_role,
                controllers::role::controller::delete_role,
            ],
        )
        // 添加用户服务作为状态管理
        .attach(AdHoc::on_ignite("Initialize user service", |rocket| async move {
            let user_service = UserServiceImpl::new().await;
            rocket.manage(user_service)
        }))
        // 添加角色服务作为状态管理
        .attach(AdHoc::on_ignite("Initialize role service", |rocket| async move {
            let role_service = RoleServiceImpl::new().await;
            rocket.manage(role_service)
        }))
}
