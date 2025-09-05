//! 用户数据访问层接口定义

use crate::models::{User, UserRole};
use crate::params::user_param::UserParam;
use rocket::async_trait;
use std::fmt::Debug;

/// 用户数据访问trait
#[async_trait]
pub trait UserRepository: Debug + Send + Sync {
    /// 插入用户记录
    async fn insert(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 选择性插入用户记录
    async fn insert_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户名查找用户
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据ID获取用户信息
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;

    /// 查询用户列表
    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据角色ID查询用户角色列表
    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID查询用户角色列表
    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>>;

    /// 分页获取用户列表
    async fn get_user_list_by_page(
        &self, name: Option<String>, dept_id: Option<String>, email: Option<String>, phone_number: Option<String>, status: Option<i32>, start_date: Option<chrono::DateTime<chrono::Utc>>, end_date: Option<chrono::DateTime<chrono::Utc>>, page_num: u64, page_size: u64,
    ) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取用户列表数量
    async fn get_user_list_count(
        &self, name: Option<String>, dept_id: Option<String>, email: Option<String>, phone_number: Option<String>, status: Option<i32>, start_date: Option<chrono::DateTime<chrono::Utc>>, end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>>;

    /// 根据ID更新用户信息
    async fn update_by_primary_key(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据ID选择性更新用户信息
    async fn update_by_primary_key_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据ID删除用户
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 批量插入用户角色
    async fn batch_insert_user_role(&self, list: &[UserRole]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID和角色ID列表批量删除用户角色
    async fn batch_delete_user_role_by_user_and_role_ids(&self, user_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// 根据用户ID删除用户角色
    async fn delete_user_role_by_user_id(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
