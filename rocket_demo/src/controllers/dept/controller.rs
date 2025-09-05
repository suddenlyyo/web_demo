//! 部门控制器

use rocket::State;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

use crate::models::dept::Dept;
use crate::params::dept_param::DeptParam;
use crate::services::dept::dept_service::DeptService;
use crate::services::dept::dept_service_impl::DeptServiceImpl;
use common_wrapper::ResponseTrait;
use common_wrapper::{ResponseWrapper, SingleWrapper};
use serde_json::Value;

/// 部门控制器

/// 查询部门列表
#[get("/dept/list")]
pub async fn list_depts(dept_param: DeptParam, dept_service: &rocket::State<DeptServiceImpl>) -> Json<Value> {
    let result = dept_service.select_dept_list(dept_param).await;
    
    // 构造返回的JSON数据
    let mut response_data = serde_json::Map::new();
    response_data.insert("code".to_string(), Value::Number(result.get_code().into()));
    response_data.insert("msg".to_string(), Value::String(result.get_message().to_string()));
    
    if let Some(depts) = result.data {
        let depts_json: Vec<Value> = depts.into_iter().map(|dept| {
            let mut dept_map = serde_json::Map::new();
            dept_map.insert("id".to_string(), Value::String(dept.id));
            dept_map.insert("name".to_string(), Value::String(dept.name));
            dept_map.insert("parent_id".to_string(), Value::String(dept.parent_id.unwrap_or_default()));
            dept_map.insert("status".to_string(), Value::Number(dept.status.into()));
            // TODO: 添加parentName、statusDesc等字段
            Value::Object(dept_map)
        }).collect();
        
        response_data.insert("data".to_string(), Value::Array(depts_json));
    } else {
        response_data.insert("data".to_string(), Value::Array(vec![]));
    }
    
    Json(Value::Object(response_data))
}

/// 获取部门树
#[get("/dept/getDeptTree")]
pub async fn get_dept_tree(dept_service: &State<DeptServiceImpl>) -> SingleWrapper<Vec<Dept>> {
    let depts = dept_service.select_dept_list(DeptParam::default()).await;
    let mut wrapper = SingleWrapper::new();
    wrapper.set_data(depts.data.unwrap_or_default());
    wrapper
}

/// 新增部门
#[post("/dept/addDept", data = "<dept_param>")]
pub async fn add_dept(dept_param: Json<DeptParam>, dept_service: &rocket::State<DeptServiceImpl>) -> ResponseWrapper {
    dept_service.add_dept(dept_param.into_inner()).await
}

/// 编辑部门
#[put("/dept/editDept", data = "<dept_param>")]
pub async fn edit_dept(dept_param: Json<DeptParam>, dept_service: &rocket::State<DeptServiceImpl>) -> ResponseWrapper {
    dept_service.edit_dept(dept_param.into_inner()).await
}

/// 删除部门
#[delete("/dept/deleteDept/<dept_id>")]
pub async fn delete_dept(dept_id: String, dept_service: &rocket::State<DeptServiceImpl>) -> ResponseWrapper {
    dept_service.delete_dept(&dept_id).await
}

/// 修改部门状态
#[put("/dept/editDeptStatus", data = "<dept_param>")]
pub async fn edit_dept_status(dept_param: Json<DeptParam>, dept_service: &rocket::State<DeptServiceImpl>) -> ResponseWrapper {
    if let (Some(id), Some(status)) = (dept_param.id.as_ref(), dept_param.status) {
        dept_service.edit_dept_status(id, status).await
    } else {
        ResponseWrapper::new(500, "参数不完整")
    }
}

/// 注册部门相关路由
pub fn routes() -> Vec<rocket::Route> {
    routes![list_depts, get_dept_tree, add_dept, edit_dept, delete_dept, edit_dept_status]
}
