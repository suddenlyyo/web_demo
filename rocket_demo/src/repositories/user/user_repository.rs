//! 用户数据访问层接口定义

use crate::models::User;
use rocket::async_trait;
use std::fmt::Debug;

/// 用户数据访问trait
#[async_trait]
pub trait UserRepository: Debug + Send + Sync {
    /// 根据ID获取用户信息
    async fn get_user_by_id(&self, id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

    /// 获取用户列表
    async fn list_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>>;

    /// 分页查询用户列表
    async fn list_users_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<User>, u64), Box<dyn std::error::Error + Send + Sync>>;

    /// 带条件的分页查询用户列表
    async fn list_users_by_page_with_conditions(&self, page_num: Option<u64>, page_size: Option<u64>, where_clause: String) -> Result<(Vec<User>, u64, u64), Box<dyn std::error::Error + Send + Sync>>;

    /// 新增用户
    async fn add_user(&self, user: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

    /// 修改用户
    async fn update_user(&self, user: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

    /// 删除用户
    async fn delete_user(&self, id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;

    /// 修改用户状态
    async fn update_user_status(&self, id: &str, status: i32) -> Result<User, Box<dyn std::error::Error + Send + Sync>>;
}
