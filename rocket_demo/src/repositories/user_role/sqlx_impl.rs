// ==================== 数据库连接 ====================
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::collections::HashSet;
use std::error::Error as StdError;
use std::sync::OnceLock;

use crate::models::UserRole;
use crate::repositories::user_role::user_role_repository::UserRoleRepository;

// 数据库连接池
static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

#[derive(Debug)]
pub struct UserRoleRepositorySqlxImpl {
    pool: MySqlPool,
}

impl UserRoleRepositorySqlxImpl {
    pub fn new() -> Self {
        let pool = DB_POOL.get().expect("数据库连接池未初始化").clone();
        Self { pool }
    }

    pub fn init_pool(pool: MySqlPool) {
        DB_POOL.set(pool).ok(); // 如果已经设置过，则忽略
    }
}

/// SQLx的用户角色实体映射
#[derive(Debug, FromRow)]
struct UserRoleRow {
    id: String,
    user_id: Option<String>,
    role_id: Option<String>,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<NaiveDateTime>,
}

impl From<UserRoleRow> for UserRole {
    fn from(row: UserRoleRow) -> Self {
        UserRole {
            id: row.id,
            user_id: row.user_id,
            role_id: row.role_id,
            create_by: row.create_by,
            create_time: row
                .create_time_raw
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
        }
    }
}

// ==================== SQL trait 实现 ====================
#[rocket::async_trait]
impl UserRoleRepository for UserRoleRepositorySqlxImpl {
    async fn delete_by_primary_key(&self, user_id: &str, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "DELETE FROM sys_user_role WHERE user_id = ? AND role_id = ?";
        let result = sqlx::query(sql)
            .bind(user_id)
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("用户角色删除失败"));
        }

        Ok(())
    }

    async fn insert(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "INSERT INTO sys_user_role (id, user_id, role_id, create_by, create_time) VALUES (?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.user_id)
            .bind(&row.role_id)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("用户角色插入失败"));
        }

        Ok(())
    }

    async fn insert_selective(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

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

        if row.create_by.is_some() {
            fields.push("create_by");
            placeholders.push("?");
            params.push(&row.create_by);
        }

        if row.create_time.is_some() {
            fields.push("create_time");
            placeholders.push("?");
            params.push(&row.create_time.map(|t| t.naive_utc()));
        }

        let sql = format!("INSERT INTO sys_user_role ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("用户角色插入失败"));
        }

        Ok(())
    }

    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, user_id, role_id, create_by, create_time FROM sys_user_role WHERE role_id = ?";
        let result: Vec<UserRoleRow> = sqlx::query_as(sql)
            .bind(role_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(result.into_iter().map(UserRole::from).collect())
    }

    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, user_id, role_id, create_by, create_time FROM sys_user_role WHERE user_id = ?";
        let result: Vec<UserRoleRow> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(result.into_iter().map(UserRole::from).collect())
    }

    async fn batch_insert(&self, list: &[UserRole]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let mut fields = vec![];
        let mut placeholders = vec![];

        fields.push("id");
        fields.push("user_id");
        fields.push("role_id");
        fields.push("create_by");
        fields.push("create_time");

        let mut query_params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        for user_role in list {
            placeholders.push("(?, ?, ?, ?, ?)".to_string());
            query_params.push(Box::new(&user_role.id));
            query_params.push(Box::new(&user_role.user_id));
            query_params.push(Box::new(&user_role.role_id));
            query_params.push(Box::new(&user_role.create_by));
            query_params.push(Box::new(user_role.create_time.map(|t| t.naive_utc())));
        }

        let sql = format!("INSERT INTO sys_user_role ({}) VALUES {}", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in &query_params {
            query = query.bind(param.as_ref());
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("用户角色批量插入失败"));
        }

        Ok(())
    }

    async fn batch_delete_by_user_and_role_ids(&self, user_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = list.iter().map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM sys_user_role WHERE user_id = ? AND role_id IN ({})", placeholders.join(","));

        let mut query = sqlx::query(&sql);
        query = query.bind(user_id);
        for role_id in list {
            query = query.bind(role_id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "DELETE FROM sys_user_role WHERE user_id = ?";
        sqlx::query(sql).bind(user_id).execute(&self.pool).await?;
        Ok(())
    }

    async fn select_role_ids_by_user_id(&self, user_id: &str) -> Result<HashSet<String>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT role_id FROM sys_user_role WHERE user_id = ?";
        let result: Vec<(String,)> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let role_ids: HashSet<String> = result.into_iter().map(|r| r.0).collect();
        Ok(role_ids)
    }
}
