//! 角色数据访问层 Diesel 实现

use diesel::prelude::*;

use crate::models::Role;
use crate::models::constants::ROLE_FIELDS;
use crate::repositories::role::role_repository::RoleRepository;

table! {
    sys_role (id) {
        id -> Text,
        name -> Nullable<Text>,
        role_key -> Nullable<Text>,
        status -> Nullable<Integer>,
        seq_no -> Nullable<Integer>,
        create_by -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_by -> Nullable<Text>,
        update_time -> Nullable<Timestamp>,
        remark -> Nullable<Text>,
    }
}

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_role)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct RoleRow {
    id: String,
    name: Option<String>,
    role_key: Option<String>,
    status: Option<i32>,
    seq_no: Option<i32>,
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
            name: row.name,
            role_key: row.role_key,
            status: row.status,
            seq_no: row.seq_no,
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
            name: role.name.clone(),
            role_key: role.role_key.clone(),
            status: role.status,
            seq_no: role.seq_no,
            create_by: role.create_by.clone(),
            create_time: role.create_time.map(|t| t.naive_utc()),
            update_by: role.update_by.clone(),
            update_time: role.update_time.map(|t| t.naive_utc()),
            remark: role.remark.clone(),
        }
    }
}

/// 角色数据访问 Diesel 实现
#[derive(Debug)]
pub struct RoleRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl RoleRepositoryDieselImpl {
    /// 创建角色仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositoryDieselImpl {
    /// 根据主键删除角色
    async fn delete_by_primary_key(&self, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        diesel::delete(sys_role.filter(id.eq(role_id))).execute(&mut self.connection)?;
        Ok(())
    }

    /// 插入角色记录
    async fn insert(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let role_row: RoleRow = row.into();
        diesel::insert_into(sys_role)
            .values(&role_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 选择性插入角色记录
    async fn insert_selective(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let role_row: RoleRow = row.into();
        diesel::insert_into(sys_role)
            .values(&role_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据主键查询角色
    async fn select_role_by_id(&self, role_id: &str) -> Result<Option<Role>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let result = sys_role
            .filter(id.eq(role_id))
            .first::<RoleRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(Role::from))
    }

    /// 查询角色列表
    async fn select_role_list(&self, role_param: crate::services::params::user_param::RoleParam) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let mut query = sys_role.into_boxed();

        if let Some(name_filter) = &role_param.name {
            query = query.filter(name.like(format!("%{}%", name_filter)));
        }

        if let Some(role_key_filter) = &role_param.role_key {
            query = query.filter(role_key.like(format!("%{}%", role_key_filter)));
        }

        if let Some(status_filter) = role_param.status {
            query = query.filter(status.eq(status_filter));
        }

        let result = query
            .order(id.asc())
            .load::<RoleRow>(&mut self.connection)?;
        Ok(result.into_iter().map(Role::from).collect())
    }

    /// 根据主键更新角色
    async fn update_by_id(&self, row: &Role) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let role_row: RoleRow = row.into();
        let result = diesel::update(sys_role.filter(id.eq(&row.id)))
            .set(&role_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键选择性更新角色
    async fn update_by_id_selective(&self, row: &Role) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let role_row: RoleRow = row.into();
        let result = diesel::update(sys_role.filter(id.eq(&row.id)))
            .set(&role_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键删除角色
    async fn delete_by_id(&self, role_id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role::diesel_impl::sys_role::dsl::*;

        let result = diesel::delete(sys_role.filter(id.eq(role_id))).execute(&mut self.connection)?;
        Ok(result as u64)
    }
}
