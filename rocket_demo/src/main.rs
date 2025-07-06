//#[macro_use] extern crate rocket; //Rust 2015 宏导入语法
use rocket::*; //Rust 2018+

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests {

    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn index_test() {
        // 创建测试客户端
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // 发送 GET 请求到根路径
        let response = client.get("/").dispatch();

        // 验证响应
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello, world!");
    }
}
