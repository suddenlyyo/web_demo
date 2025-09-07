//! 部门控制器

use common_wrapper::enums::status_enum::StatusEnum;
use rocket::serde::json::Json;
use rocket::{State, delete, get, post, put};

use crate::models::dept::Dept;
use crate::params::dept_param::DeptParam;
use crate::services::dept::dept_service::DeptService;
use crate::views::dept_tree::DeptTree;
use common_wrapper::{ListWrapper, ResponseWrapper};
use serde_json::Value;

/// 部门控制器

/// 查询部门列表
#[get("/dept/list")]
pub async fn list_depts(dept_service: &State<Box<dyn DeptService + Send + Sync>>) -> Json<Value> {
    let result: ListWrapper<Dept> = dept_service.select_dept_list(DeptParam::default()).await;

    // 如果没有数据直接返回
    let depts = result.get_data();
    if depts.is_none() {
        return Json(serde_json::to_value(&result).unwrap_or_default());
    }

    // 将result转换成json
    let mut json_value = serde_json::to_value(&result).unwrap_or_else(|_| serde_json::json!({}));

    // 在json中添加额外信息
    if let Some(data_array) = json_value.get_mut("data").and_then(|d| d.as_array_mut()) {
        let all_depts = dept_service.get_dept().await;
        for dept_json in data_array.iter_mut() {
            let mut new_map = serde_json::Map::new();
            let dept_obj = dept_json.as_object_mut().unwrap_or_else(|| &mut new_map);

            // 添加状态描述
            if let Some(status_value) = dept_obj.get("status") {
                if let Some(status) = status_value.as_i64() {
                    if let Some(status_enum) = StatusEnum::from_code(status as i32) {
                        dept_obj.insert("statusDesc".into(), serde_json::Value::String(status_enum.desc().to_string()));
                    }
                }
            }

            // 添加父部门名称
            if let Some(parent_id_value) = dept_obj.get("parent_id") {
                if let Some(parent_id) = parent_id_value.as_str() {
                    if !parent_id.is_empty() {
                        if let Some(parent) = all_depts.get(parent_id) {
                            dept_obj.insert("parentName".into(), serde_json::Value::String(parent.name.clone().unwrap_or_default()));
                        }
                    }
                }
            }
        }
    }

    Json(json_value)
}

/// 获取部门树
#[get("/dept/getDeptTree")]
pub async fn get_dept_tree(dept_service: &State<Box<dyn DeptService + Send + Sync>>) -> Json<ListWrapper<DeptTree>> {
    let result = dept_service.get_dept_tree(DeptParam::default()).await;
    Json(result)
}

/// 添加部门
#[post("/dept/add", data = "<dept_param>")]
pub async fn add_dept(dept_param: Json<DeptParam>, dept_service: &State<Box<dyn DeptService + Send + Sync>>) -> Json<ResponseWrapper> {
    let result = dept_service.add_dept(dept_param.into_inner()).await;
    Json(result)
}

/// 编辑部门
#[put("/dept/edit", data = "<dept_param>")]
pub async fn edit_dept(dept_param: Json<DeptParam>, dept_service: &State<Box<dyn DeptService + Send + Sync>>) -> Json<ResponseWrapper> {
    let result = dept_service.edit_dept(dept_param.into_inner()).await;
    Json(result)
}

/// 删除部门
#[delete("/dept/delete/<dept_id>")]
pub async fn delete_dept(dept_id: String, dept_service: &State<Box<dyn DeptService + Send + Sync>>) -> Json<ResponseWrapper> {
    let result = dept_service.delete_dept(&dept_id).await;
    Json(result)
}

/// 修改部门状态
#[put("/dept/editStatus/<id>/<status>")]
pub async fn edit_dept_status(id: String, status: i32, dept_service: &State<Box<dyn DeptService + Send + Sync>>) -> Json<ResponseWrapper> {
    let result = dept_service.edit_dept_status(&id, status).await;
    Json(result)
}

/// 注册部门相关路由
pub fn routes() -> Vec<rocket::Route> {
    routes![list_depts, get_dept_tree, add_dept, edit_dept, delete_dept, edit_dept_status]
}
