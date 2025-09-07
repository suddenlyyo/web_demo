//! SQLx实现的部门数据访问层
//!
//! 该模块提供了基于SQLx的部门数据访问实现，支持异步数据库操作。
//! 实现了DeptRepository trait定义的所有方法。

use crate::config::Config;
use crate::models::Dept;
use crate::models::constants::DEPT_FIELDS;
use crate::repositories::dept::dept_repository::DeptRepository;
use rocket::async_trait;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::fmt::Debug;

/// SQLx实现的部门仓储
#[derive(Debug)]
pub struct DeptRepositorySqlxImpl {
    pool: MySqlPool,
}

impl DeptRepositorySqlxImpl {
    /// 创建新的SQLx部门仓储实例
    ///
    /// # 返回值
    /// 返回新的部门仓储实例
    pub async fn new() -> Result<Self, Box<dyn StdError + Send + Sync>> {
        // 从配置文件中读取数据库URL
        let config = Config::from_default_file().expect("无法加载配置文件");
        let database_url = config.database.url;
        let pool = MySqlPool::connect(&database_url)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl DeptRepository for DeptRepositorySqlxImpl {
    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_dept WHERE id = ?";
        sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 插入部门记录
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = format!(
            r#"
            INSERT INTO sys_dept ({DEPT_FIELDS})
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
        );

        sqlx::query(&sql)
            .bind(&row.id)
            .bind(&row.name)
            .bind(&row.email)
            .bind(&row.telephone)
            .bind(&row.address)
            .bind(&row.logo)
            .bind(&row.parent_id)
            .bind(&row.dept_level)
            .bind(&row.seq_no)
            .bind(&row.status)
            .bind(&row.create_by)
            .bind(&row.create_time)
            .bind(&row.update_by)
            .bind(&row.update_time)
            .bind(&row.remark)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = format!(
            r#"
            INSERT INTO sys_dept ({DEPT_FIELDS})
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
        );

        sqlx::query(&sql)
            .bind(&row.id)
            .bind(&row.name)
            .bind(&row.email)
            .bind(&row.telephone)
            .bind(&row.address)
            .bind(&row.logo)
            .bind(&row.parent_id)
            .bind(&row.dept_level)
            .bind(&row.seq_no)
            .bind(&row.status)
            .bind(&row.create_by)
            .bind(&row.create_time)
            .bind(&row.update_by)
            .bind(&row.update_time)
            .bind(&row.remark)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!(
            r#"
            SELECT {DEPT_FIELDS}
            FROM sys_dept 
            WHERE id = ?
        "#
        );

        sqlx::query_as::<_, Dept>(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 根据父部门ID查询部门
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!(
            r#"
            SELECT {DEPT_FIELDS}
            FROM sys_dept 
            WHERE parent_id = ?
            LIMIT 1
        "#
        );

        sqlx::query_as::<_, Dept>(&sql)
            .bind(parent_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!(
            r#"
            SELECT {DEPT_FIELDS}
            FROM sys_dept 
            WHERE 1=1
        "#
        );

        // 构建动态查询条件
        let mut conditions = String::new();
        if row.name.is_some() {
            conditions.push_str(" AND name LIKE ?");
        }
        if row.status.is_some() {
            conditions.push_str(" AND status = ?");
        }

        let final_sql = sql + &conditions;
        let mut query = sqlx::query_as::<_, Dept>(&final_sql);

        // 绑定参数
        if let Some(ref name) = row.name {
            query = query.bind(format!("%{}%", name));
        }
        if let Some(status) = row.status {
            query = query.bind(status);
        }

        query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 根据主键更新部门
    async fn update_by_primary_key(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = r#"
            UPDATE sys_dept 
            SET name = ?, email = ?, telephone = ?, address = ?, logo = ?, 
                parent_id = ?, dept_level = ?, seq_no = ?, status = ?, 
                create_by = ?, create_time = ?, update_by = ?, 
                update_time = ?, remark = ?
            WHERE id = ?
        "#;

        let result = sqlx::query(sql)
            .bind(&row.name)
            .bind(&row.email)
            .bind(&row.telephone)
            .bind(&row.address)
            .bind(&row.logo)
            .bind(&row.parent_id)
            .bind(&row.dept_level)
            .bind(&row.seq_no)
            .bind(&row.status)
            .bind(&row.create_by)
            .bind(&row.create_time)
            .bind(&row.update_by)
            .bind(&row.update_time)
            .bind(&row.remark)
            .bind(&row.id)
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        Ok(result.rows_affected())
    }

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let mut sql = "UPDATE sys_dept SET ".to_string();
        let mut setters = Vec::new();

        // 构建动态更新语句
        if row.name.is_some() {
            setters.push("name = ?");
        }
        if row.email.is_some() {
            setters.push("email = ?");
        }
        if row.telephone.is_some() {
            setters.push("telephone = ?");
        }
        if row.address.is_some() {
            setters.push("address = ?");
        }
        if row.logo.is_some() {
            setters.push("logo = ?");
        }
        if row.parent_id.is_some() {
            setters.push("parent_id = ?");
        }
        if row.dept_level.is_some() {
            setters.push("dept_level = ?");
        }
        if row.seq_no.is_some() {
            setters.push("seq_no = ?");
        }
        if row.status.is_some() {
            setters.push("status = ?");
        }
        if row.create_by.is_some() {
            setters.push("create_by = ?");
        }
        if row.create_time.is_some() {
            setters.push("create_time = ?");
        }
        if row.update_by.is_some() {
            setters.push("update_by = ?");
        }
        if row.update_time.is_some() {
            setters.push("update_time = ?");
        }
        if row.remark.is_some() {
            setters.push("remark = ?");
        }

        if setters.is_empty() {
            return Ok(0);
        }

        sql.push_str(&setters.join(", "));
        sql.push_str(" WHERE id = ?");

        let mut query = sqlx::query(&sql);

        // 绑定参数
        if let Some(ref name) = row.name {
            query = query.bind(name);
        }
        if let Some(ref email) = row.email {
            query = query.bind(email);
        }
        if let Some(ref telephone) = row.telephone {
            query = query.bind(telephone);
        }
        if let Some(ref address) = row.address {
            query = query.bind(address);
        }
        if let Some(ref logo) = row.logo {
            query = query.bind(logo);
        }
        if let Some(ref parent_id) = row.parent_id {
            query = query.bind(parent_id);
        }
        if let Some(ref dept_level) = row.dept_level {
            query = query.bind(dept_level);
        }
        if let Some(seq_no) = row.seq_no {
            query = query.bind(seq_no);
        }
        if let Some(status) = row.status {
            query = query.bind(status);
        }
        if let Some(ref create_by) = row.create_by {
            query = query.bind(create_by);
        }
        if let Some(create_time) = row.create_time {
            query = query.bind(create_time);
        }
        if let Some(ref update_by) = row.update_by {
            query = query.bind(update_by);
        }
        if let Some(update_time) = row.update_time {
            query = query.bind(update_time);
        }
        if let Some(ref remark) = row.remark {
            query = query.bind(remark);
        }

        query = query.bind(&row.id);

        let result = query
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        Ok(result.rows_affected())
    }
}
