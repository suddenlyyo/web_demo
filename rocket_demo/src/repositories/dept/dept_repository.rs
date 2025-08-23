//! 部门数据访问层接口定义

use std::fmt::Debug;

use rocket::async_trait;

use crate::models::Dept;

/// 部门数据访问trait
#[async_trait]
pub trait DeptRepository: Debug + Send + Sync {
    /// 根据ID获取部门信息
    async fn get_dept_by_id(&self, id: &str) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取部门列表
    async fn list_depts(&self) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>>;

    /// 分页查询部门列表
    async fn list_depts_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Dept>, u64, u64), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据父部门ID获取子部门列表
    async fn list_children_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取部门树结构
    async fn list_dept_tree(&self) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>>;

    /// 新增部门
    async fn add_dept(&self, dept: Dept) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>>;

    /// 更新部门
    async fn update_dept(&self, dept: Dept) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>>;

    /// 删除部门
    async fn delete_dept(&self, id: &str) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>>;

    /// 更新部门状态
    async fn update_dept_status(&self, id: &str, status: i32) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>>;
}
