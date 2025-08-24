//! 部门数据访问层接口定义

use crate::models::Dept;
use crate::services::params::user_param::DeptParam;
use rocket::async_trait;
use std::error::Error as StdError;
use std::fmt::Debug;

/// 部门数据访问trait
#[async_trait]
pub trait DeptRepository: Debug + Send + Sync {
    /// 根据主键删除部门
    ///
    /// # 参数
    /// * `id` - 部门ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 插入部门记录
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 选择性插入部门记录
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 根据主键查询部门
    ///
    /// # 参数
    /// * `id` - 部门ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回部门信息，类型: [Result<Option<Dept>, Box<dyn StdError + Send + Sync>>]
    async fn select_dept_by_id(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>>;

    /// 查询部门列表
    ///
    /// # 参数
    /// * `dept_param` - 部门查询参数，类型: [DeptParam]
    ///
    /// # 返回值
    /// 返回部门列表，类型: [Result<Vec<Dept>, Box<dyn StdError + Send + Sync>>]
    async fn select_dept_list(&self, dept_param: DeptParam) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>>;

    /// 根据主键更新部门
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回更新影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn update_by_id(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 根据主键选择性更新部门
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回更新影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn update_by_id_selective(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 根据主键删除部门
    ///
    /// # 参数
    /// * `id` - 部门ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回删除影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn StdError + Send + Sync>>;
}
