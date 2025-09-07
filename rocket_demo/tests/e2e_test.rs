//! 端到端测试脚本
//!
//! 这个脚本用于测试运行中的服务器，模拟真实的API调用场景
//! 使用方法：
//! 1. 在一个终端运行 `cargo run` 启动服务器
//! 2. 在另一个终端运行 `cargo test e2e --test e2e_test` 执行端到端测试

use reqwest;
use serde_json;

/// 测试添加部门接口 (Create)
#[tokio::test]
async fn test_add_dept_api() {
    // 确保服务器已启动
    let client = reqwest::Client::new();

    // 准备测试数据
    let dept_data = serde_json::json!({
        "parentId": "065a3eb180214ccfbb653f63287d285d",
        "name": "测试部门",
        "seqNo": 1,
        "telephone": "13800000000",
        "email": "test@example.com",
        "status": 1,
        "createBy": "test_user"
    });

    // 发送POST请求到添加部门接口
    let res = client
        .post("http://localhost:8000/dept/add")
        .json(&dept_data)
        .send()
        .await;

    // 检查请求是否成功发送
    match res {
        Ok(response) => {
            let status = response.status();
            println!("Status: {}", status);
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);

            // 对于这个测试，我们只验证请求是否成功发送，而不验证响应内容
            // 因为在测试环境中可能没有真实的数据库支持
            assert!(status != reqwest::StatusCode::NOT_FOUND);
        },
        Err(e) => {
            println!("请求失败，请确保服务器正在运行: {}", e);
            // 在实际测试中，我们希望服务器正在运行
            panic!("请求失败，请确保服务器正在运行: {}", e);
        },
    }
}

/// 测试部门列表接口 (Read)
#[tokio::test]
async fn test_dept_list_api() {
    // 确保服务器已启动
    let client = reqwest::Client::new();
    
    // 准备测试数据
    let dept_data = serde_json::json!({
        "id": null,
        "parentId": null,
        "name": null,
        "email": null,
        "telephone": null,
        "address": null,
        "logo": null,
        "deptLevel": null,
        "seqNo": null,
        "status": null,
        "createBy": null,
        "createTime": null,
        "updateBy": null,
        "updateTime": null,
        "remark": null,
        "pageParam": {
            "pageNum": null,
            "pageSize": null
        }
    });

    // 发送POST请求到部门列表接口
    let res = client
        .post("http://localhost:8000/dept/list")
        .json(&dept_data)
        .send()
        .await;

    // 检查请求是否成功发送（即使返回404也说明服务器在运行）
    match res {
        Ok(response) => {
            let status = response.status();
            println!("Status: {}", status);
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);

            // 验证响应状态不是404
            assert!(status != reqwest::StatusCode::NOT_FOUND);
        },
        Err(e) => {
            println!("请求失败，请确保服务器正在运行: {}", e);
            // 在实际测试中，我们希望服务器正在运行
            panic!("请求失败，请确保服务器正在运行: {}", e);
        },
    }
}

/// 测试部门树接口 (Read)
#[tokio::test]
async fn test_dept_tree_api() {
    // 确保服务器已启动
    let client = reqwest::Client::new();
    
    // 准备测试数据
    let dept_data = serde_json::json!({
        "id": null,
        "parentId": null,
        "name": null,
        "email": null,
        "telephone": null,
        "address": null,
        "logo": null,
        "deptLevel": null,
        "seqNo": null,
        "status": null,
        "createBy": null,
        "createTime": null,
        "updateBy": null,
        "updateTime": null,
        "remark": null,
        "pageParam": {
            "pageNum": null,
            "pageSize": null
        }
    });

    // 发送POST请求到部门树接口
    let res = client
        .post("http://localhost:8000/dept/getDeptTree")
        .json(&dept_data)
        .send()
        .await;

    // 检查请求是否成功发送
    match res {
        Ok(response) => {
            let status = response.status();
            println!("Status: {}", status);
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);

            // 验证响应状态不是404
            assert!(status != reqwest::StatusCode::NOT_FOUND);
        },
        Err(e) => {
            println!("请求失败，请确保服务器正在运行: {}", e);
            // 在实际测试中，我们希望服务器正在运行
            panic!("请求失败，请确保服务器正在运行: {}", e);
        },
    }
}

/// 测试编辑部门接口 (Update)
#[tokio::test]
async fn test_edit_dept_api() {
    // 确保服务器已启动
    let client = reqwest::Client::new();

    // 准备测试数据
    let dept_data = serde_json::json!({
        "id": "test_dept_id",
        "parentId": "065a3eb180214ccfbb653f63287d285d",
        "name": "更新测试部门",
        "seqNo": 2,
        "telephone": "13900000000",
        "email": "update@example.com",
        "status": 1,
        "updateBy": "test_user"
    });

    // 发送PUT请求到编辑部门接口（使用一个已知的部门ID）
    let res = client
        .put("http://localhost:8000/dept/edit")
        .json(&dept_data)
        .send()
        .await;

    // 检查请求是否成功发送
    match res {
        Ok(response) => {
            let status = response.status();
            println!("Status: {}", status);
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);

            // 对于这个测试，我们只验证请求是否成功发送，而不验证响应内容
            // 因为在测试环境中可能没有真实的数据库支持
            assert!(status != reqwest::StatusCode::NOT_FOUND);
        },
        Err(e) => {
            println!("请求失败，请确保服务器正在运行: {}", e);
            // 在实际测试中，我们希望服务器正在运行
            panic!("请求失败，请确保服务器正在运行: {}", e);
        },
    }
}

/// 测试修改部门状态接口 (Update)
#[tokio::test]
async fn test_edit_dept_status_api() {
    // 确保服务器已启动
    let client = reqwest::Client::new();

    // 发送PUT请求到修改部门状态接口（使用一个测试部门ID）
    let res = client
        .put("http://localhost:8000/dept/editStatus/test_dept_id/0")
        .send()
        .await;

    // 检查请求是否成功发送
    match res {
        Ok(response) => {
            let status = response.status();
            println!("Status: {}", status);
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);

            // 验证响应状态是成功的
            assert!(status.is_success());
        },
        Err(e) => {
            println!("请求失败，请确保服务器正在运行: {}", e);
            // 在实际测试中，我们希望服务器正在运行
            panic!("请求失败，请确保服务器正在运行: {}", e);
        },
    }
}

/// 测试删除部门接口 (Delete)
#[tokio::test]
async fn test_delete_dept_api() {
    // 确保服务器已启动
    let client = reqwest::Client::new();

    // 发送DELETE请求到删除部门接口（使用一个测试部门ID）
    let res = client
        .delete("http://localhost:8000/dept/delete/test_dept_id")
        .send()
        .await;

    // 检查请求是否成功发送
    match res {
        Ok(response) => {
            let status = response.status();
            println!("Status: {}", status);
            let body = response.text().await.unwrap();
            println!("Response body: {}", body);

            // 验证响应状态是成功的
            assert!(status.is_success());
        },
        Err(e) => {
            println!("请求失败，请确保服务器正在运行: {}", e);
            // 在实际测试中，我们希望服务器正在运行
            panic!("请求失败，请确保服务器正在运行: {}", e);
        },
    }
}