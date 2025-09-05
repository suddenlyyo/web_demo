use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::query_dsl::methods::{FilterDsl, LoadQuery};

// ==================== 数据库连接 ====================
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::query_dsl::methods::{FilterDsl, LoadQuery};

use crate::models::UserRole;
use crate::repositories::user_role::user_role_repository::UserRoleRepository;

// ==================== 表结构体映射 ====================
table! {
    sys_user_role (id) {
        id -> Text,
        user_id -> Nullable<Text>,
        role_id -> Nullable<Text>,
    }
}

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_user_role)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
struct UserRoleRow {
    id: String,
    user_id: Option<String>,
    role_id: Option<String>,
}

impl From<UserRoleRow> for UserRole {
    fn from(row: UserRoleRow) -> Self {
        UserRole { id: row.id, user_id: row.user_id, role_id: row.role_id }
    }
}

impl From<&UserRole> for UserRoleRow {
    fn from(user_role: &UserRole) -> Self {
        UserRoleRow {
            id: user_role.id.clone(),
            user_id: user_role.user_id.clone(),
            role_id: user_role.role_id.clone(),
        }
    }
}

// ==================== SQL trait 实现 ====================
/// 用户角色数据访问 Diesel 实现
#[derive(Debug)]
pub struct UserRoleRepositoryDieselImpl {
    connection: diesel::mysql::MysqlConnection,
}

impl UserRoleRepositoryDieselImpl {
    /// 创建用户角色仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() { config.database.url } else { "mysql://root:123456@localhost:3306/demo".to_string() };

        let connection = diesel::mysql::MysqlConnection::establish(&database_url).expect("无法连接到数据库");

        Self { connection }
    }
}

#[async_trait::async_trait]
impl UserRoleRepository for UserRoleRepositoryDieselImpl {
    /// 根据主键删除用户角色关联
    async fn delete_by_primary_key(&self, user_role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        diesel::delete(sys_user_role.filter(id.eq(user_role_id))).execute(&mut self.connection)?;
        Ok(())
    }

    /// 插入用户角色关联记录
    async fn insert(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        let user_role_row: UserRoleRow = row.into();
        diesel::insert_into(sys_user_role)
            .values(&user_role_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 选择性插入用户角色关联记录
    async fn insert_selective(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        let user_role_row: UserRoleRow = row.into();
        diesel::insert_into(sys_user_role)
            .values(&user_role_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据主键查询用户角色关联
    async fn select_by_id(&self, user_role_id: &str) -> Result<Option<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        let result = sys_user_role
            .filter(id.eq(user_role_id))
            .first::<UserRoleRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(UserRole::from))
    }

    /// 查询用户角色关联列表
    async fn select_list(&self, user_role_param: crate::services::params::user_param::UserRoleParam) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        let mut query = sys_user_role.into_boxed();

        if let Some(user_id_filter) = &user_role_param.user_id {
            query = query.filter(user_id.eq(user_id_filter));
        }

        if let Some(role_id_filter) = &user_role_param.role_id {
            query = query.filter(role_id.eq(role_id_filter));
        }

        let result = query
            .order(id.asc())
            .load::<UserRoleRow>(&mut self.connection)?;
        Ok(result.into_iter().map(UserRole::from).collect())
    }

    /// 根据主键更新用户角色关联
    async fn update_by_id(&self, row: &UserRole) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        let user_role_row: UserRoleRow = row.into();
        let result = diesel::update(sys_user_role.filter(id.eq(&row.id)))
            .set(&user_role_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键选择性更新用户角色关联
    async fn update_by_id_selective(&self, row: &UserRole) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        let user_role_row: UserRoleRow = row.into();
        let result = diesel::update(sys_user_role.filter(id.eq(&row.id)))
            .set(&user_role_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键删除用户角色关联
    async fn delete_by_id(&self, user_role_id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::models::constants::USER_ROLE_FIELDS;
        use crate::repositories::user_role::diesel_impl::sys_user_role::dsl::*;

        let result = diesel::delete(sys_user_role.filter(id.eq(user_role_id))).execute(&mut self.connection)?;
        Ok(result as u64)
    }
}
