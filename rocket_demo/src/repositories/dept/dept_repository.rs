//! 部门数据访问层接口定义

use std::fmt::Debug;

use rocket::async_trait;

use crate::models::Dept;

/// 部门数据访问trait
#[async_trait]
pub trait DeptRepository: Debug + Send + Sync {
    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 插入部门记录
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据主键更新部门
    async fn update_by_primary_key(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据父部门ID查询子部门列表
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>>;
}
