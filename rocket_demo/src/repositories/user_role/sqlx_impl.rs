use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::models::SysUserRole;
use crate::models::constants::USER_ROLE_FIELDS;
use crate::repositories::user_role::user_role_repository::UserRoleRepository;

/// 用户角色关联仓库SQLx实现
#[derive(Debug)]
pub struct UserRoleRepositorySqlxImpl {
    pool: Arc<MySqlPool>,
}

/// SQLx的用户角色实体映射
#[derive(Debug, FromRow)]
struct UserRoleRow {
    id: String,
    user_id: Option<String>,
    role_id: Option<String>,
}

impl From<UserRoleRow> for SysUserRole {
    fn from(row: UserRoleRow) -> Self {
        SysUserRole { id: row.id, user_id: row.user_id, role_id: row.role_id }
    }
}

impl UserRoleRepositorySqlxImpl {
    /// 创建用户角色关联仓库SQLx实现
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool: Arc::new(pool) }
    }
}

#[rocket::async_trait]
impl UserRoleRepository for UserRoleRepositorySqlxImpl {
    /// 根据主键删除用户角色关联
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_user_role WHERE id = ?";
        sqlx::query(sql)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    /// 插入用户角色关联记录
    async fn insert(&self, row: &SysUserRole) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "INSERT INTO sys_user_role (id, user_id, role_id) VALUES (?, ?, ?)";
        sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.user_id)
            .bind(&row.role_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    /// 选择性插入用户角色关联记录
    async fn insert_selective(&self, row: &SysUserRole) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

        fields.push("id");
        placeholders.push("?");
        params.push(&row.id);

        if row.user_id.is_some() {
            fields.push("user_id");
            placeholders.push("?");
            params.push(&row.user_id);
        }

        if row.role_id.is_some() {
            fields.push("role_id");
            placeholders.push("?");
            params.push(&row.role_id);
        }

        let sql = format!("INSERT INTO sys_user_role ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let query = sqlx::query(&sql).bind_all(params);
        query.execute(self.pool.as_ref()).await?;
        Ok(())
    }

    /// 根据主键查询用户角色关联
    async fn select_by_id(&self, id: &str) -> Result<Option<SysUserRole>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_user_role WHERE id = ?", USER_ROLE_FIELDS);
        let result: Option<UserRoleRow> = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(result.map(SysUserRole::from))
    }

    /// 查询用户角色关联列表
    async fn select_list(&self, user_role_param: crate::services::params::user_param::UserRoleParam) -> Result<Vec<SysUserRole>, Box<dyn StdError + Send + Sync>> {
        let mut sql = format!("SELECT {} FROM sys_user_role WHERE 1=1", USER_ROLE_FIELDS);
        let mut params: Vec<Box<(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)>> = vec![];

        if let Some(user_id) = &user_role_param.user_id {
            sql.push_str(" AND user_id = ?");
            params.push(Box::new(user_id.clone()));
        }

        if let Some(role_id) = &user_role_param.role_id {
            sql.push_str(" AND role_id = ?");
            params.push(Box::new(role_id.clone()));
        }

        sql.push_str(" ORDER BY id");

        let mut query = sqlx::query_as::<_, UserRoleRow>(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(self.pool.as_ref()).await?;
        Ok(result.into_iter().map(SysUserRole::from).collect())
    }

    /// 根据主键更新用户角色关联
    async fn update_by_id(&self, row: &SysUserRole) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_user_role SET user_id = ?, role_id = ? WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(&row.user_id)
            .bind(&row.role_id)
            .bind(&row.id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }

    /// 根据主键选择性更新用户角色关联
    async fn update_by_id_selective(&self, row: &SysUserRole) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let mut sets = vec![];
        let mut params: Vec<Box<(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)>> = vec![];

        if row.user_id.is_some() {
            sets.push("user_id = ?");
            params.push(Box::new(&row.user_id));
        }

        if row.role_id.is_some() {
            sets.push("role_id = ?");
            params.push(Box::new(&row.role_id));
        }

        if sets.is_empty() {
            return Ok(0);
        }

        let mut sql = format!("UPDATE sys_user_role SET {}", sets.join(", "));
        sql.push_str(" WHERE id = ?");
        params.push(Box::new(&row.id));

        let mut query = sqlx::query(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }
        query = query.bind(&row.id);

        let result = query.execute(self.pool.as_ref()).await?;
        Ok(result.rows_affected())
    }

    /// 根据主键删除用户角色关联
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_user_role WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }
}
