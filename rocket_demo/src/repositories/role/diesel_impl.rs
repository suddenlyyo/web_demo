//! 角色数据访问层 Diesel 实现

// ==================== 导入依赖 ====================
use diesel::prelude::*;

use crate::models::Role;
use crate::repositories::role::role_repository::RoleRepository;

// ==================== 表结构体映射 ====================
table! {
    sys_role (id) {
        id -> Text,
        role_name -> Nullable<Text>,
        role_key -> Nullable<Text>,
        status -> Nullable<Integer>,
        create_by -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_by -> Nullable<Text>,
        update_time -> Nullable<Timestamp>,
        remark -> Nullable<Text>,
    }
}

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_role)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct RoleRow {
    id: String,
    role_name: Option<String>,
    role_key: Option<String>,
    status: Option<i32>,
    create_by: Option<String>,
    create_time: Option<chrono::NaiveDateTime>,
    update_by: Option<String>,
    update_time: Option<chrono::NaiveDateTime>,
    remark: Option<String>,
}

impl From<RoleRow> for Role {
    fn from(row: RoleRow) -> Self {
        Role {
            id: row.id,
            role_name: row.role_name,
            role_key: row.role_key,
            status: row.status,
            create_by: row.create_by,
            create_time: row
                .create_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: row.remark,
        }
    }
}

impl From<&Role> for RoleRow {
    fn from(role: &Role) -> Self {
        RoleRow {
            id: role.id.clone(),
            role_name: role.role_name.clone(),
            role_key: role.role_key.clone(),
            status: role.status,
            create_by: role.create_by.clone(),
            create_time: role.create_time.map(|t| t.naive_utc()),
            update_by: role.update_by.clone(),
            update_time: role.update_time.map(|t| t.naive_utc()),
            remark: role.remark.clone(),
        }
    }
}

// ==================== SQL trait 实现 ====================
#[derive(Debug)]
pub struct RoleRepositoryDieselImpl {
    connection: diesel::mysql::MysqlConnection,
}

impl RoleRepositoryDieselImpl {
    pub fn new() -> Self {
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() {
            config.database.url
        } else {
            panic!("无法从配置文件获取数据库连接信息");
        };

        let connection = diesel::mysql::MysqlConnection::establish(&database_url).expect("连接MySQL数据库时出错");

        Self { connection }
    }
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositoryDieselImpl {
    async fn select_role_by_id(&self, role_id: &str) -> Result<Option<Role>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let result = sys_role
            .filter(id.eq(role_id))
            .first::<RoleRow>(&mut self.connection)
            .optional()?;

        Ok(result.map(Role::from))
    }

    async fn select_role_list(&self, role_param: crate::params::role_param::RoleParam) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let mut query = sys_role.into_boxed();

        if let Some(role_name_filter) = role_param.role_name {
            query = query.filter(role_name.like(format!("%{}%", role_name_filter)));
        }

        if let Some(role_key_filter) = role_param.role_key {
            query = query.filter(role_key.like(format!("%{}%", role_key_filter)));
        }

        if let Some(status_filter) = role_param.status {
            query = query.filter(status.eq(status_filter));
        }

        let result = query.load::<RoleRow>(&mut self.connection)?;
        Ok(result.into_iter().map(Role::from).collect())
    }

    async fn insert(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let role_row: RoleRow = row.into();
        diesel::insert_into(sys_role)
            .values(&role_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    async fn update_by_id(&self, row: &Role) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let role_row: RoleRow = row.into();
        let result = diesel::update(sys_role.filter(id.eq(&row.id)))
            .set(&role_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    async fn delete_by_id(&self, role_id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let result = diesel::delete(sys_role.filter(id.eq(role_id))).execute(&mut self.connection)?;
        Ok(result as u64)
    }
}
