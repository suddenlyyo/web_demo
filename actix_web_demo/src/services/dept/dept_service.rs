//! 部门服务接口定义

use crate::models::Dept;
use crate::params::dept_param::DeptParam;
use crate::views::dept_tree::DeptTree;
use async_trait::async_trait;
use common_wrapper::{ListWrapper, ResponseWrapper};
use std::collections::HashMap;
/// 部门服务trait
#[async_trait]
pub trait DeptService: Send + Sync {
    /// 获取部门树
    async fn get_dept_tree(&self, dept_param: DeptParam) -> ListWrapper<DeptTree>;

    /// 获取部门信息Map 用于部门信息匹配
    async fn get_dept(&self, dept_param: DeptParam) -> HashMap<String, Dept>;

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
