//#[macro_use] extern crate rocket; //Rust 2015 宏导入语法
use rocket::{get, routes};//Rust 2018及以上

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
