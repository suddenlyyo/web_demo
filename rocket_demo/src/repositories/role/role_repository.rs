/// 角色数据访问层接口定义
use crate::models::Role;
use crate::services::params::user_param::RoleParam;
use rocket::async_trait;
use std::error::Error as StdError;
use std::fmt::Debug;

/// 角色数据访问trait
#[async_trait]
pub trait RoleRepository: Debug + Send + Sync {
    /// 根据用户ID查询角色列表
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回角色列表，类型: [Result<Vec<Role>, Box<dyn StdError + Send + Sync>>]
    async fn select_roles_by_user_id(&self, user_id: &str) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>>;

    /// 查询角色列表
    ///
    /// # 参数
    /// * `role` - 角色信息，类型: [&Role]
    ///
    /// # 返回值
    /// 返回角色列表，类型: [Result<Vec<Role>, Box<dyn StdError + Send + Sync>>]
    async fn select_role_list(&self, role: &Role) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>>;

    /// 根据主键查询角色
    ///
    /// # 参数
    /// * `id` - 角色ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回角色信息，类型: [Result<Option<Role>, Box<dyn StdError + Send + Sync>>]
    async fn select_role_by_id(&self, id: &str) -> Result<Option<Role>, Box<dyn StdError + Send + Sync>>;

    /// 插入角色记录
    ///
    /// # 参数
    /// * `row` - 角色信息，类型: [&Role]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    async fn insert(&self, row: &Role) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 选择性插入角色记录
    ///
    /// # 参数
    /// * `row` - 角色信息，类型: [&Role]
    ///
    /// # 返回值
    /// 返回操作结果，类型: [Result<(), Box<dyn StdError + Send + Sync>>]
    async fn insert_selective(&self, row: &Role) -> Result<(), Box<dyn StdError + Send + Sync>>;

    /// 根据主键更新角色
    ///
    /// # 参数
    /// * `row` - 角色信息，类型: [&Role]
    ///
    /// # 返回值
    /// 返回更新影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn update_by_id(&self, row: &Role) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 根据主键选择性更新角色
    ///
    /// # 参数
    /// * `row` - 角色信息，类型: [&Role]
    ///
    /// # 返回值
    /// 返回更新影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn update_by_id_selective(&self, row: &Role) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 根据主键删除角色
    ///
    /// # 参数
    /// * `id` - 角色ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回删除影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 根据角色ID列表批量删除角色
    ///
    /// # 参数
    /// * `ids` - 角色ID列表，类型: &[&str]
    ///
    /// # 返回值
    /// 返回删除影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn batch_delete_by_ids(&self, ids: &[&str]) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 根据角色ID更新角色状态
    ///
    /// # 参数
    /// * `id` - 角色ID，类型: [&str]
    /// * `status` - 状态值，类型: [i32]
    ///
    /// # 返回值
    /// 返回更新影响的行数，类型: [Result<u64, Box<dyn StdError + Send + Sync>>]
    async fn update_role_status(&self, id: &str, status: i32) -> Result<u64, Box<dyn StdError + Send + Sync>>;

    /// 查询角色列表
    ///
    /// # 参数
    /// * `role_param` - 角色查询参数，类型: [RoleParam]
    ///
    /// # 返回值
    /// 返回角色列表，类型: [Result<Vec<Role>, Box<dyn StdError + Send + Sync>>]
    async fn select_roles(&self, role_param: RoleParam) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>>;
}
