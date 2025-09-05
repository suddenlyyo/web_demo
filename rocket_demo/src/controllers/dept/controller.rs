//! 部门控制器

use rocket::State;
use rocket::routes;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

use common_wrapper::{ResponseWrapper, SingleWrapper};

use crate::services::dept::dept_service::DeptService;
use crate::services::dept::dept_service_impl::DeptServiceImpl;

/// 部门控制器

/// 查询部门列表
#[get("/dept/list")]
pub async fn list_depts(dept_param: DeptParam, dept_service: &rocket::State<DeptServiceImpl>) -> serde_json::Value {
    // TODO: 实现查询部门列表的逻辑
    serde_json::Value::Null
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
