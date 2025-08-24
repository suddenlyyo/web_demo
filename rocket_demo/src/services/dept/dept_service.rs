//! 部门服务接口定义

use crate::models::Dept;
use crate::services::params::page_param::PageParam;
use common_wrapper::{ListWrapper, ResponseWrapper};
use std::collections::HashMap;

/// 树形结构VO
#[derive(Debug, Clone)]
pub struct TreeVO {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub email: String,
    pub telephone: String,
    pub address: String,
    pub logo: String,
    pub dept_level: String,
    pub seq_no: i32,
    pub status: i32,
    pub create_by: String,
    pub create_time: chrono::NaiveDateTime,
    pub update_by: String,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub remark: String,
    pub children: Vec<TreeVO>,
}

/// 部门参数
#[derive(Debug, Clone)]
pub struct DeptParam {
    pub dept_id: Option<String>,
    pub dept_name: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub address: Option<String>,
    pub logo: Option<String>,
    pub parent_id: Option<String>,
    pub dept_level: Option<String>,
    pub seq_no: Option<i32>,
    pub status: Option<i32>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::NaiveDateTime>,
    pub remark: Option<String>,
}

/// 部门服务trait
#[rocket::async_trait]
pub trait DeptService {
    /// 获取部门树
    async fn get_dept_tree(&self, dept_param: DeptParam) -> ListWrapper<TreeVO>;

    /// 获取部门信息Map 用于部门信息匹配
    async fn get_dept(&self) -> HashMap<String, Dept>;

    /// 查询部门列表
    async fn select_dept_list(&self, dept_param: DeptParam) -> ListWrapper<Dept>;

    /// 新增部门
    async fn add_dept(&self, dept_param: DeptParam) -> ResponseWrapper;

    /// 编辑部门
    async fn edit_dept(&self, dept_param: DeptParam) -> ResponseWrapper;

    /// 编辑部门状态
    async fn edit_dept_status(&self, id: &str, status: i32) -> ResponseWrapper;

    /// 删除部门
    async fn delete_dept(&self, dept_id: &str) -> ResponseWrapper;
}
