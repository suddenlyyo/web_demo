// Rocket Demo 最简集成测试文件
//
// 本测试文件验证 Rocket 应用的基本功能，
// 仅包含最核心的测试用例。

use rocket::http::Status;
use rocket::local::blocking::Client;

/// 测试基本的Rocket功能
#[test]
fn test_rocket_functionality() {
    // 创建一个简单的Rocket应用
    let rocket = rocket::build().mount("/", rocket::routes![hello]);

    // 创建测试客户端
    let client = Client::tracked(rocket).expect("valid rocket instance");

    // 发送 GET 请求
    let response = client.get("/hello").dispatch();

    // 验证响应状态
    assert_eq!(response.status(), Status::Ok);

    // 验证响应内容
    let body = response.into_string().unwrap();
    assert_eq!(body, "Hello, world!");
}

// 简单的测试路由
#[rocket::get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}
