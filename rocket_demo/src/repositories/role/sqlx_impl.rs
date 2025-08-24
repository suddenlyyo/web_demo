//! 角色数据访问层SQLx实现

use sqlx::Row;
use sqlx::mysql::MySqlPool;

use crate::models::Role;
use crate::repositories::role::role_repository::RoleRepository;

/// 角色表的所有字段，用于SQL查询
const ROLE_FIELDS: &str = "id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark";

/// 数据库映射器
struct DbMapper;

impl DbMapper {
    /// 将数据库行映射为角色对象
    fn map_to_role(row: &MySqlRow) -> Result<Role, sqlx::Error> {
        Ok(Role {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            role_key: row.try_get("role_key")?,
            seq_no: row.try_get("seq_no")?,
            status: row.try_get("status")?,
            create_by: row.try_get("create_by")?,
            create_time: row
                .try_get::<Option<NaiveDateTime>, _>("create_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            update_by: row.try_get("update_by")?,
            update_time: row
                .try_get::<Option<NaiveDateTime>, _>("update_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            remark: row.try_get("remark")?,
        })
    }
}

/// SQLx实现的角色数据访问
#[derive(Debug)]
pub struct RoleRepositorySqlxImpl {
    pool: MySqlPool,
}

impl RoleRepositorySqlxImpl {
    /// 创建新的角色数据访问实例
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 从数据库URL创建连接池并初始化Repository
    pub async fn from_database_url(database_url: &str) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        let pool = MySqlPool::connect(database_url).await?;
        Ok(Self::new(pool))
    }
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositorySqlxImpl {
    /// 根据主键删除角色
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_role WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("角色删除失败"));
        }

        Ok(())
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
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

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

        let sql = format!(
            "INSERT INTO sys_role ({}) VALUES ({})",
            fields.join(", "),
            placeholders.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("角色插入失败"));
        }

        Ok(())
    }

    /// 根据主键查询角色
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Role>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_role WHERE id = ?", ROLE_FIELDS);
        let result = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => {
                let role = DbMapper::map_to_role(&row)?;
                Ok(Some(role))
            }
            None => Ok(None),
        }
    }

    /// 根据主键选择性更新角色
    async fn update_by_primary_key_selective(&self, row: &Role) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

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
            return Ok(());
        }

        let sql = format!(
            "UPDATE sys_role SET {} WHERE id = ?",
            updates.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }
        query = query.bind(&row.id);

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("角色更新失败"));
        }

        Ok(())
    }

    /// 根据主键更新角色
    async fn update_by_primary_key(&self, row: &Role) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_role SET name = ?, role_key = ?, seq_no = ?, status = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";

        let result = sqlx::query(sql)
            .bind(&row.name)
            .bind(&row.role_key)
            .bind(row.seq_no)
            .bind(row.status)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .bind(&row.id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("角色更新失败"));
        }

        Ok(())
    }

    /// 根据用户ID查询角色列表
    async fn select_role_by_user_id(&self, user_id: &str) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_role r LEFT JOIN sys_user_role ur ON r.id = ur.role_id WHERE ur.user_id = ? ORDER BY r.seq_no", ROLE_FIELDS);
        
        let rows = sqlx::query(&sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let roles: Result<Vec<Role>, _> = rows
            .iter()
            .map(|row| DbMapper::map_to_role(row))
            .collect();

        Ok(roles?)
    }

    /// 查询角色列表
    async fn select_role_list(&self, row: &Role) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut conditions = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

        if let Some(name) = &row.name {
            conditions.push("name LIKE ?");
            params.push(&format!("%{}%", name));
        }

        if let Some(status) = row.status {
            conditions.push("status = ?");
            params.push(&status);
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!("SELECT {} FROM sys_role {} ORDER BY seq_no", ROLE_FIELDS, where_clause);

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let rows = query.fetch_all(&self.pool).await?;
        let roles: Result<Vec<Role>, _> = rows
            .iter()
            .map(|row| DbMapper::map_to_role(row))
            .collect();

        Ok(roles?)
    }
}