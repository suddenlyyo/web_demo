//! 部门数据访问层接口定义
//!
//! 该模块定义了部门相关的数据访问接口，提供了对部门信息的增删改查操作。
//! 所有方法都是异步的，支持在多线程环境下安全使用。
//!
//! # 主要功能
//! - 部门的增删改查操作
//! - 部门列表查询
//! - 部门状态更新
//!
//! # 使用示例
//! ```rust
//! // 使用示例需要具体的实现类，这里是接口定义
//! // 通常会有一个实现了DeptRepository trait的具体结构体
//! ```

use crate::models::Dept;
use rocket::async_trait;
use std::error::Error as StdError;
use std::fmt::Debug;

/// 部门数据访问trait
///
/// 定义了部门相关的数据访问接口，所有实现该trait的结构体都必须提供这些方法的具体实现。
/// 该trait要求实现Debug、Send和Sync trait，以确保可以在多线程环境中安全使用。
#[async_trait]
pub trait DeptRepository: Debug + Send + Sync {
    /// 根据主键删除部门
    ///
    /// # 参数
    /// * `id` - 部门ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    ///
    /// # 示例
    /// ```rust
    /// # async fn example() {
    /// // repo 是实现了 DeptRepository trait 的具体实例
    /// let result = repo.delete_by_primary_key("1").await;
    /// match result {
    ///     Ok(()) => println!("删除成功"),
    ///     Err(e) => println!("删除失败: {}", e),
    /// }
    /// # }
    /// ```
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 插入部门记录
    ///
    /// 插入完整的部门记录，即使某些字段为None也会插入对应字段的NULL值。
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    ///
    /// # 示例
    /// ```rust
    /// # async fn example() {
    /// use crate::models::Dept;
    /// use chrono::Utc;
    ///
    /// let dept = Dept {
    ///     id: "1".to_string(),
    ///     name: Some("研发部".to_string()),
    ///     // 其他字段...
    ///     ..Default::default()
    /// };
    ///
    /// // repo 是实现了 DeptRepository trait 的具体实例
    /// let result = repo.insert(&dept).await;
    /// match result {
    ///     Ok(()) => println!("插入成功"),
    ///     Err(e) => println!("插入失败: {}", e),
    /// }
    /// # }
    /// ```
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 选择性插入部门记录
    ///
    /// 只插入部门记录中非None的字段，对于None字段则不进行插入操作。
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 根据主键查询部门
    ///
    /// 根据部门ID查询部门信息，如果部门存在则返回Some(Dept)，否则返回None。
    ///
    /// # 参数
    /// * `id` - 部门ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回部门信息，类型: [Result<Option<Dept>, Box<dyn StdError + Send + Sync>>]
    ///
    /// # 示例
    /// ```rust
    /// # async fn example() {
    /// // repo 是实现了 DeptRepository trait 的具体实例
    /// let result = repo.select_by_primary_key("1").await;
    /// match result {
    ///     Ok(Some(dept)) => println!("查询到部门: {:?}", dept),
    ///     Ok(None) => println!("未找到该部门"),
    ///     Err(e) => println!("查询失败: {}", e),
    /// }
    /// # }
    /// ```
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>>;

    /// 根据父部门ID查询部门
    ///
    /// 根据父部门ID查询子部门信息，如果存在则返回Some(Dept)，否则返回None。
    ///
    /// # 参数
    /// * `parent_id` - 父部门ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回部门信息，类型: [Result<Option<Dept>, Box<dyn StdError + Send + Sync>>]
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>>;

    /// 查询部门列表
    ///
    /// 根据条件查询部门列表，支持按部门名称模糊查询和部门状态筛选。
    ///
    /// # 参数
    /// * `row` - 查询条件，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回部门列表，类型: [Result<Vec<Dept>, Box<dyn StdError + Send + Sync>>]
    ///
    /// # 示例
    /// ```rust
    /// # async fn example() {
    /// use crate::models::Dept;
    ///
    /// let condition = Dept {
    ///     name: Some("研发".to_string()),  // 模糊查询包含"研发"的部门
    ///     status: Some(0),                 // 查询状态为正常的部门
    ///     ..Default::default()
    /// };
    ///
    /// // repo 是实现了 DeptRepository trait 的具体实例
    /// let result = repo.select_dept_list(&condition).await;
    /// match result {
    ///     Ok(depts) => println!("查询到{}个部门", depts.len()),
    ///     Err(e) => println!("查询失败: {}", e),
    /// }
    /// # }
    /// ```
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>>;

    /// 根据主键更新部门
    ///
    /// 更新指定ID的部门信息，会更新所有字段，即使某些字段为None也会更新为NULL。
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回更新影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn update_by_primary_key(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 根据主键选择性更新部门
    ///
    /// 更新指定ID的部门信息，只会更新非None字段，对于None字段保持原值不变。
    ///
    /// # 参数
    /// * `row` - 部门信息，类型: [&Dept]
    ///
    /// # 返回值
    /// 返回更新影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>>;
}
