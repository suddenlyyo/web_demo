//! SQLx实现的部门数据访问层
//!
//! 该模块提供了基于SQLx的部门数据访问实现，支持异步数据库操作。
//! 实现了DeptRepository trait定义的所有方法。

use crate::models::Dept;
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
    /// # 参数
    /// * `pool` - MySQL连接池
    ///
    /// # 返回值
    /// 返回新的部门仓储实例
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
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
        let sql = r#"
            INSERT INTO sys_dept (
                id, name, email, telephone, address, logo, parent_id, 
                dept_level, seq_no, status, create_by, create_time, 
                update_by, update_time, remark
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(sql)
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
        let sql = r#"
            INSERT INTO sys_dept (
                id, name, email, telephone, address, logo, parent_id, 
                dept_level, seq_no, status, create_by, create_time, 
                update_by, update_time, remark
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(sql)
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
        let sql = r#"
            SELECT id, name, email, telephone, address, logo, parent_id, 
                    dept_level, seq_no, status, create_by, create_time, 
                    update_by, update_time, remark
            FROM sys_dept 
            WHERE id = ?
        "#;

        sqlx::query_as::<_, Dept>(sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 根据父部门ID查询部门
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = r#"
            SELECT id, name, email, telephone, address, logo, parent_id, 
                   dept_level, seq_no, status, create_by, create_time, 
                   update_by, update_time, remark
            FROM sys_dept 
            WHERE parent_id = ?
            LIMIT 1
        "#;

        sqlx::query_as::<_, Dept>(sql)
            .bind(parent_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let mut sql = r#"
            SELECT id, name, email, telephone, address, logo, parent_id, 
                   dept_level, seq_no, status, create_by, create_time, 
                   update_by, update_time, remark
            FROM sys_dept 
            WHERE 1=1
        "#
        .to_string();

        // 构建动态查询条件
        if let Some(ref _name) = row.name {
            sql.push_str(" AND name LIKE ?");
        }
        if let Some(_status) = row.status {
            sql.push_str(" AND status = ?");
        }

        let mut query = sqlx::query_as::<_, Dept>(&sql);

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
        let mut params: Vec<Box<dyn sqlx::Encode<sqlx::MySql> + Send + Sync>> = Vec::new();

        // 构建动态更新语句
        if let Some(ref name) = row.name {
            sql.push_str("name = ?, ");
            params.push(Box::new(name.clone()));
        }
        if let Some(ref email) = row.email {
            sql.push_str("email = ?, ");
            params.push(Box::new(email.clone()));
        }
        if let Some(ref telephone) = row.telephone {
            sql.push_str("telephone = ?, ");
            params.push(Box::new(telephone.clone()));
        }
        if let Some(ref address) = row.address {
            sql.push_str("address = ?, ");
            params.push(Box::new(address.clone()));
        }
        if let Some(ref logo) = row.logo {
            sql.push_str("logo = ?, ");
            params.push(Box::new(logo.clone()));
        }
        if let Some(ref parent_id) = row.parent_id {
            sql.push_str("parent_id = ?, ");
            params.push(Box::new(parent_id.clone()));
        }
        if let Some(ref dept_level) = row.dept_level {
            sql.push_str("dept_level = ?, ");
            params.push(Box::new(dept_level.clone()));
        }
        if let Some(seq_no) = row.seq_no {
            sql.push_str("seq_no = ?, ");
            params.push(Box::new(seq_no));
        }
        if let Some(status) = row.status {
            sql.push_str("status = ?, ");
            params.push(Box::new(status));
        }
        if let Some(ref create_by) = row.create_by {
            sql.push_str("create_by = ?, ");
            params.push(Box::new(create_by.clone()));
        }
        if let Some(create_time) = row.create_time {
            sql.push_str("create_time = ?, ");
            params.push(Box::new(create_time));
        }
        if let Some(ref update_by) = row.update_by {
            sql.push_str("update_by = ?, ");
            params.push(Box::new(update_by.clone()));
        }
        if let Some(update_time) = row.update_time {
            sql.push_str("update_time = ?, ");
            params.push(Box::new(update_time));
        }
        if let Some(ref remark) = row.remark {
            sql.push_str("remark = ?, ");
            params.push(Box::new(remark.clone()));
        }

        // 移除最后的逗号和空格
        if sql.ends_with(", ") {
            sql.pop();
            sql.pop();
        }

        sql.push_str(" WHERE id = ?");
        params.push(Box::new(row.id.clone()));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        Ok(result.rows_affected())
    }
}
