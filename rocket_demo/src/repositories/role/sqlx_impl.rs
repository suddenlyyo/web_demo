use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::models::Role;
use crate::models::constants::ROLE_FIELDS;
use crate::repositories::role::role_repository::RoleRepository;
use crate::services::role::role_service::RoleParam;

// ==================== 数据库连接 ====================

/// SQLx实现的角色数据访问
#[derive(Debug)]
pub struct RoleRepositorySqlxImpl {
    pool: Arc<MySqlPool>,
}

/// SQLx的角色实体映射
#[derive(Debug, FromRow)]
struct RoleRow {
    id: String,
    name: Option<String>,
    role_key: Option<String>,
    status: Option<i32>,
    seq_no: Option<i32>,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<NaiveDateTime>,
    update_by: Option<String>,
    #[sqlx(rename = "update_time")]
    update_time_raw: Option<NaiveDateTime>,
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
                .create_time_raw
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time_raw
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            remark: row.remark,
        }
    }
}

// ==================== 表结构体映射 ====================

impl RoleRepositorySqlxImpl {
    /// 创建角色仓库 SQLx 实例
    ///
    /// # 返回值
    ///
    /// 返回新的角色仓库实例
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // 从配置文件读取数据库URL
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() {
            config.database.url
        } else {
            panic!("无法从配置文件获取数据库连接信息");
        };

        let pool = MySqlPool::connect(&database_url).await?;
        Ok(Self { pool: Arc::new(pool) })
    }
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositorySqlxImpl {
    // ==================== SQL trait 实现 ====================
    /// 根据主键查询角色
    async fn select_role_by_id(&self, id: &str) -> Result<Option<Role>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_role WHERE id = ?", ROLE_FIELDS);
        let result: Option<RoleRow> = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await?;

        Ok(result.map(Role::from))
    }

    /// 根据主键删除角色
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_role WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;

        Ok(result.rows_affected())
    }

    /// 根据主键更新角色
    async fn update_by_id(&self, row: &Role) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_role SET name = ?, role_key = ?, status = ?, seq_no = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(&row.name)
            .bind(&row.role_key)
            .bind(row.status)
            .bind(row.seq_no)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .bind(&row.id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }

    /// 插入角色记录
    async fn insert(&self, row: &Role) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "INSERT INTO sys_role (id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.name)
            .bind(&row.role_key)
            .bind(row.seq_no)
            .bind(row.status)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("角色插入失败"));
        }

        Ok(())
    }

    /// 选择性插入角色记录
    async fn insert_selective(&self, row: &Role) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

        fields.push("id");
        placeholders.push("?");
        params.push(&row.id);

        if row.name.is_some() {
            fields.push("name");
            placeholders.push("?");
            params.push(&row.name);
        }

        if row.role_key.is_some() {
            fields.push("role_key");
            placeholders.push("?");
            params.push(&row.role_key);
        }

        if row.seq_no.is_some() {
            fields.push("seq_no");
            placeholders.push("?");
            params.push(&row.seq_no);
        }

        if row.status.is_some() {
            fields.push("status");
            placeholders.push("?");
            params.push(&row.status);
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

        if row.update_by.is_some() {
            fields.push("update_by");
            placeholders.push("?");
            params.push(&row.update_by);
        }

        if row.update_time.is_some() {
            fields.push("update_time");
            placeholders.push("?");
            params.push(&row.update_time.map(|t| t.naive_utc()));
        }

        if row.remark.is_some() {
            fields.push("remark");
            placeholders.push("?");
            params.push(&row.remark);
        }

        let sql = format!("INSERT INTO sys_role ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(self.pool.as_ref()).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("角色插入失败"));
        }

        Ok(())
    }

    /// 根据主键选择性更新角色
    async fn update_by_id_selective(&self, row: &Role) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

        if row.name.is_some() {
            updates.push("name = ?");
            params.push(&row.name);
        }

        if row.role_key.is_some() {
            updates.push("role_key = ?");
            params.push(&row.role_key);
        }

        if row.seq_no.is_some() {
            updates.push("seq_no = ?");
            params.push(&row.seq_no);
        }

        if row.status.is_some() {
            updates.push("status = ?");
            params.push(&row.status);
        }

        if row.create_by.is_some() {
            updates.push("create_by = ?");
            params.push(&row.create_by);
        }

        if row.create_time.is_some() {
            updates.push("create_time = ?");
            params.push(&row.create_time.map(|t| t.naive_utc()));
        }

        if row.update_by.is_some() {
            updates.push("update_by = ?");
            params.push(&row.update_by);
        }

        if row.update_time.is_some() {
            updates.push("update_time = ?");
            params.push(&row.update_time.map(|t| t.naive_utc()));
        }

        if row.remark.is_some() {
            updates.push("remark = ?");
            params.push(&row.remark);
        }

        if updates.is_empty() {
            return Ok(0);
        }

        let sql = format!("UPDATE sys_role SET {} WHERE id = ?", updates.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }
        query = query.bind(&row.id);

        let result = query.execute(self.pool.as_ref()).await?;
        Ok(result.rows_affected())
    }

    /// 查询角色列表
    async fn select_role_list(&self, role: &Role) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>> {
        let mut sql = format!("SELECT {} FROM sys_role WHERE 1=1", ROLE_FIELDS);
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if let Some(name) = &role.name {
            sql.push_str(" AND name LIKE ?");
            params.push(Box::new(format!("%{}%", name)));
        }

        if let Some(status) = role.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        sql.push_str(" ORDER BY seq_no");

        let mut query = sqlx::query_as::<_, RoleRow>(&sql);
        for param in params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(self.pool.as_ref()).await?;
        Ok(result.into_iter().map(Role::from).collect())
    }

    /// 查询所有角色列表
    /*async fn select_role_list_all(&self) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_role ORDER BY seq_no", ROLE_FIELDS);
        let result: Vec<RoleRow> = sqlx::query_as(&sql).fetch_all(self.pool.as_ref()).await?;
        Ok(result.into_iter().map(Role::from).collect())
    }*/

    /// 根据用户ID查询角色列表
    async fn select_roles_by_user_id(&self, user_id: &str) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_role r LEFT JOIN sys_user_role ur ON r.id = ur.role_id WHERE ur.user_id = ?", ROLE_FIELDS);
        let result: Vec<RoleRow> = sqlx::query_as(&sql)
            .bind(user_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(result.into_iter().map(Role::from).collect())
    }

    /// 根据角色ID列表批量删除角色
    async fn batch_delete_by_ids(&self, ids: &[&str]) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        if ids.is_empty() {
            return Ok(0);
        }

        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM sys_role WHERE id IN ({})", placeholders.join(","));

        let mut query = sqlx::query(&sql);
        for id in ids {
            query = query.bind(id);
        }

        let result = query.execute(self.pool.as_ref()).await?;
        Ok(result.rows_affected())
    }

    /// 根据角色ID更新角色状态
    async fn update_role_status(&self, id: &str, status: i32) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_role SET status = ? WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(status)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }

    /// 查询角色列表
    async fn select_roles(&self, role_param: RoleParam) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>> {
        let mut sql = format!("SELECT {} FROM sys_role WHERE 1=1", ROLE_FIELDS);
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if let Some(name) = role_param.name {
            sql.push_str(" AND name LIKE ?");
            params.push(Box::new(format!("%{}%", name)));
        }

        if let Some(status) = role_param.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        sql.push_str(" ORDER BY seq_no");

        let mut query = sqlx::query_as::<_, RoleRow>(&sql);
        for param in params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(self.pool.as_ref()).await?;
        Ok(result.into_iter().map(Role::from).collect())
    }
}
