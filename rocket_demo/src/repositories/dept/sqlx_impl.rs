use chrono::{NaiveDateTime, Utc};
use sqlx::Row;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;

use crate::models::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;

mod constants {
    use super::*;
    
    /// 部门表的所有字段，用于SQL查询
    pub const DEPT_FIELDS: &str = "id, parent_id, name, email, telephone, address, logo, dept_level, seq_no, status, create_by, create_time, update_by, update_time, remark";
}

mod mappers {
    use super::*;
    
    /// 数据库映射器
    struct DbMapper;

    impl DbMapper {
        /// 将数据库行映射为部门对象
        fn map_to_dept(row: &sqlx::mysql::MySqlRow) -> Result<Dept, sqlx::Error> {
            Ok(Dept {
                id: row.try_get("id")?,
                parent_id: row.try_get("parent_id")?,
                name: row.try_get("name")?,
                email: row.try_get("email")?,
                telephone: row.try_get("telephone")?,
                address: row.try_get("address")?,
                logo: row.try_get("logo")?,
                dept_level: row.try_get("dept_level")?,
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
}

/// SQLx实现的部门数据访问
#[derive(Debug)]
pub struct DeptRepositorySqlxImpl {
    pool: MySqlPool,
}

impl DeptRepositorySqlxImpl {
    /// 创建新的部门数据访问实例
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
impl DeptRepository for DeptRepositorySqlxImpl {
    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_dept WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("部门删除失败"));
        }

        Ok(())
    }

    /// 插入部门记录
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "INSERT INTO sys_dept (id, parent_id, name, email, telephone, address, logo, dept_level, seq_no, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.parent_id)
            .bind(&row.name)
            .bind(&row.email)
            .bind(&row.telephone)
            .bind(&row.address)
            .bind(&row.logo)
            .bind(&row.dept_level)
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
            return Err(Box::from("部门插入失败"));
        }

        Ok(())
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

        fields.push("id");
        placeholders.push("?");
        params.push(&row.id);

        if row.parent_id.is_some() {
            fields.push("parent_id");
            placeholders.push("?");
            params.push(&row.parent_id);
        }

        if row.name.is_some() {
            fields.push("name");
            placeholders.push("?");
            params.push(&row.name);
        }

        if row.email.is_some() {
            fields.push("email");
            placeholders.push("?");
            params.push(&row.email);
        }

        if row.telephone.is_some() {
            fields.push("telephone");
            placeholders.push("?");
            params.push(&row.telephone);
        }

        if row.address.is_some() {
            fields.push("address");
            placeholders.push("?");
            params.push(&row.address);
        }

        if row.logo.is_some() {
            fields.push("logo");
            placeholders.push("?");
            params.push(&row.logo);
        }

        if row.dept_level.is_some() {
            fields.push("dept_level");
            placeholders.push("?");
            params.push(&row.dept_level);
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
            "INSERT INTO sys_dept ({}) VALUES ({})",
            fields.join(", "),
            placeholders.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("部门插入失败"));
        }

        Ok(())
    }

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_dept WHERE id = ?", DEPT_FIELDS);
        let result = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => {
                let dept = DbMapper::map_to_dept(&row)?;
                Ok(Some(dept))
            }
            None => Ok(None),
        }
    }

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

        if row.parent_id.is_some() {
            updates.push("parent_id = ?");
            params.push(&row.parent_id);
        }

        if row.name.is_some() {
            updates.push("name = ?");
            params.push(&row.name);
        }

        if row.email.is_some() {
            updates.push("email = ?");
            params.push(&row.email);
        }

        if row.telephone.is_some() {
            updates.push("telephone = ?");
            params.push(&row.telephone);
        }

        if row.address.is_some() {
            updates.push("address = ?");
            params.push(&row.address);
        }

        if row.logo.is_some() {
            updates.push("logo = ?");
            params.push(&row.logo);
        }

        if row.dept_level.is_some() {
            updates.push("dept_level = ?");
            params.push(&row.dept_level);
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
            "UPDATE sys_dept SET {} WHERE id = ?",
            updates.join(", ")
        );

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }
        query = query.bind(&row.id);

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("部门更新失败"));
        }

        Ok(())
    }

    /// 根据主键更新部门
    async fn update_by_primary_key(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_dept SET parent_id = ?, name = ?, email = ?, telephone = ?, address = ?, logo = ?, dept_level = ?, seq_no = ?, status = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";

        let result = sqlx::query(sql)
            .bind(&row.parent_id)
            .bind(&row.name)
            .bind(&row.email)
            .bind(&row.telephone)
            .bind(&row.address)
            .bind(&row.logo)
            .bind(&row.dept_level)
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
            return Err(Box::from("部门更新失败"));
        }

        Ok(())
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut conditions = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

        if let Some(parent_id) = &row.parent_id {
            conditions.push("parent_id = ?");
            params.push(parent_id);
        }

        if let Some(name) = &row.name {
            conditions.push("name = ?");
            params.push(name);
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

        let sql = format!("SELECT {} FROM sys_dept {} ORDER BY seq_no", DEPT_FIELDS, where_clause);

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let rows = query.fetch_all(&self.pool).await?;
        let depts: Result<Vec<Dept>, _> = rows
            .iter()
            .map(|row| DbMapper::map_to_dept(row))
            .collect();

        Ok(depts?)
    }

    /// 根据父部门ID查询子部门列表
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_dept WHERE parent_id = ? ORDER BY seq_no", DEPT_FIELDS);
        
        let rows = sqlx::query(&sql)
            .bind(parent_id)
            .fetch_all(&self.pool)
            .await?;

        let depts: Result<Vec<Dept>, _> = rows
            .iter()
            .map(|row| DbMapper::map_to_dept(row))
            .collect();

        Ok(depts?)
    }
}