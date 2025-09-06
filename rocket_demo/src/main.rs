#[macro_use]
extern crate rocket;

mod config;
mod controllers;
mod models;
mod params;
mod repositories;
mod services;
mod views;

#[launch]
async fn rocket() -> _ {
    // 构建Rocket实例
    rocket::build()
        // 注册控制器
        .mount(
            "/",
            routes![
                controllers::index::controller::index,
                controllers::dept::controller::list_depts,
                controllers::dept::controller::get_dept_tree,
                controllers::dept::controller::add_dept,
                controllers::dept::controller::edit_dept,
                controllers::dept::controller::delete_dept,
                controllers::dept::controller::edit_dept_status,
            ],
        )
}
