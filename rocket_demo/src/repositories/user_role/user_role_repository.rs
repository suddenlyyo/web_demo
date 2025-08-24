//! 用户角色数据访问层接口定义

use crate::models::UserRole;
use rocket::async_trait;
use std::fmt::Debug;

/// 用户角色数据访问trait
#[async_trait]
pub trait UserRoleRepository: Debug + Send + Sync {
    /// 根据用户ID和角色ID删除用户角色
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    /// * `role_id` - 角色ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn delete_by_primary_key(&self, user_id: &str, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 插入用户角色记录
    ///
    /// # 参数
    /// * `row` - 用户角色信息，类型: [&UserRole]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn insert(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 选择性插入用户角色记录
    ///
    /// # 参数
    /// * `row` - 用户角色信息，类型: [&UserRole]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn insert_selective(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据角色ID查询用户角色列表
    ///
    /// # 参数
    /// * `role_id` - 角色ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回用户角色列表，类型: [Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>>]
    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID查询用户角色列表
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回用户角色列表，类型: [Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>>]
    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>>;

    /// 批量插入用户角色
    ///
    /// # 参数
    /// * `list` - 用户角色列表，类型: [&[UserRole]]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn batch_insert(&self, list: &[UserRole]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID和角色ID列表批量删除用户角色
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    /// * `list` - 角色ID列表，类型: [&[String]]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn batch_delete_by_user_and_role_ids(&self, user_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID删除用户角色
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn std::error::Error + Send + Sync>>]
    async fn delete_by_user_id(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}