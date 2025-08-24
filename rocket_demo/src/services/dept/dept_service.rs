//! 部门服务接口定义

use super::PageParam;
use crate::models::Dept;
use common_wrapper::{ListWrapper, ResponseWrapper};
use std::collections::HashMap;

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

/// 部门参数
#[derive(Debug, Clone)]
pub struct DeptParam {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub telephone: Option<String>,
    pub address: Option<String>,
    pub logo: Option<String>,
    pub parent_id: Option<String>,
    pub dept_level: Option<String>,
    pub seq_no: Option<i32>,
    pub status: Option<i32>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    pub remark: Option<String>,
}

/// 树形结构VO
#[derive(Debug, Clone)]
pub struct TreeVO {
    // 根据实际需要定义字段
}
