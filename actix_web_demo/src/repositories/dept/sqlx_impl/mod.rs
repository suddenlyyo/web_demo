//! SQLx实现的部门数据访问层
//!
//! 该模块提供了基于SQLx的部门数据访问实现，支持异步数据库操作。
//! 实现了DeptRepository trait定义的所有方法。

use crate::config::Config;
use crate::models::Dept;
use crate::models::constants::DEPT_FIELDS;
use crate::repositories::dept::dept_repository::DeptRepository;
use async_trait::async_trait;
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

    /// 使用QueryBuilder构建查询部门列表的查询
    fn build_select_dept_list_query_with_builder<'a>(&'a self, query: &mut sqlx::QueryBuilder<'a, sqlx::MySql>, row: &'a Dept) {
        query.push(format!("SELECT {DEPT_FIELDS} FROM sys_dept WHERE 1=1"));

        // 添加所有可能的查询条件
        if !row.id.is_empty() {
            query.push(" AND id = ");
            query.push_bind(&row.id);
        }

        if let Some(ref name) = row.name {
            query.push(" AND name LIKE ");
            query.push_bind(format!("%{}%", name));
        }

        if let Some(ref email) = row.email {
            query.push(" AND email = ");
            query.push_bind(email);
        }

        if let Some(ref telephone) = row.telephone {
            query.push(" AND telephone = ");
            query.push_bind(telephone);
        }

        if let Some(ref address) = row.address {
            query.push(" AND address = ");
            query.push_bind(address);
        }

        if let Some(ref logo) = row.logo {
            query.push(" AND logo = ");
            query.push_bind(logo);
        }

        if let Some(ref parent_id) = row.parent_id {
            query.push(" AND parent_id = ");
            query.push_bind(parent_id);
        }

        if let Some(seq_no) = row.seq_no {
            query.push(" AND seq_no = ");
            query.push_bind(seq_no);
        }

        if let Some(status) = row.status {
            query.push(" AND status = ");
            query.push_bind(status);
        }

        if let Some(ref create_by) = row.create_by {
            query.push(" AND create_by = ");
            query.push_bind(create_by);
        }

        if let Some(create_time) = row.create_time {
            query.push(" AND create_time = ");
            query.push_bind(create_time);
        }

        if let Some(ref update_by) = row.update_by {
            query.push(" AND update_by = ");
            query.push_bind(update_by);
        }

        if let Some(update_time) = row.update_time {
            query.push(" AND update_time = ");
            query.push_bind(update_time);
        }

        if let Some(ref remark) = row.remark {
            query.push(" AND remark = ");
            query.push_bind(remark);
        }
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
        let mut query = sqlx::QueryBuilder::new("INSERT INTO sys_dept (");

        // 收集需要插入的字段名
        let mut fields = Vec::new();

        // 检查每个字段是否有值，如果有值则添加到插入列表中
        if !row.id.is_empty() {
            fields.push("id");
        }

        if row.name.is_some() {
            fields.push("name");
        }

        if row.email.is_some() {
            fields.push("email");
        }

        if row.telephone.is_some() {
            fields.push("telephone");
        }

        if row.address.is_some() {
            fields.push("address");
        }

        if row.logo.is_some() {
            fields.push("logo");
        }

        if row.parent_id.is_some() {
            fields.push("parent_id");
        }

        if row.seq_no.is_some() {
            fields.push("seq_no");
        }

        if row.status.is_some() {
            fields.push("status");
        }

        if row.create_by.is_some() {
            fields.push("create_by");
        }

        if row.create_time.is_some() {
            fields.push("create_time");
        }

        if row.update_by.is_some() {
            fields.push("update_by");
        }

        if row.update_time.is_some() {
            fields.push("update_time");
        }

        if row.remark.is_some() {
            fields.push("remark");
        }

        // 如果没有任何字段需要插入，则返回错误
        if fields.is_empty() {
            return Err("没有需要插入的字段".into());
        }

        // 构建字段列表
        query.push(fields.join(", "));
        query.push(") VALUES (");

        // 添加参数占位符
        let placeholders: Vec<String> = fields.iter().map(|_| "?".to_string()).collect();
        query.push(placeholders.join(", "));
        query.push(")");

        // 使用QueryBuilder绑定参数值
        if !row.id.is_empty() {
            query.push_bind(&row.id);
        }

        if let Some(ref name) = row.name {
            query.push_bind(name);
        }

        if let Some(ref email) = row.email {
            query.push_bind(email);
        }

        if let Some(ref telephone) = row.telephone {
            query.push_bind(telephone);
        }

        if let Some(ref address) = row.address {
            query.push_bind(address);
        }

        if let Some(ref logo) = row.logo {
            query.push_bind(logo);
        }

        if let Some(ref parent_id) = row.parent_id {
            query.push_bind(parent_id);
        }

        if let Some(seq_no) = row.seq_no {
            query.push_bind(seq_no);
        }

        if let Some(status) = row.status {
            query.push_bind(status);
        }

        if let Some(ref create_by) = row.create_by {
            query.push_bind(create_by);
        }

        if let Some(create_time) = row.create_time {
            query.push_bind(create_time);
        }

        if let Some(ref update_by) = row.update_by {
            query.push_bind(update_by);
        }

        if let Some(update_time) = row.update_time {
            query.push_bind(update_time);
        }

        if let Some(ref remark) = row.remark {
            query.push_bind(remark);
        }

        let sql = query.build();
        sql.execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let mut query = sqlx::QueryBuilder::new("INSERT INTO sys_dept (");

        // 收集需要插入的字段名
        let mut fields = Vec::new();

        // 检查每个字段是否有值，如果有值则添加到插入列表中
        if !row.id.is_empty() {
            fields.push("id");
        }

        if row.name.is_some() {
            fields.push("name");
        }

        if row.email.is_some() {
            fields.push("email");
        }

        if row.telephone.is_some() {
            fields.push("telephone");
        }

        if row.address.is_some() {
            fields.push("address");
        }

        if row.logo.is_some() {
            fields.push("logo");
        }

        if row.parent_id.is_some() {
            fields.push("parent_id");
        }

        if row.seq_no.is_some() {
            fields.push("seq_no");
        }

        if row.status.is_some() {
            fields.push("status");
        }

        if row.create_by.is_some() {
            fields.push("create_by");
        }

        if row.create_time.is_some() {
            fields.push("create_time");
        }

        if row.update_by.is_some() {
            fields.push("update_by");
        }

        if row.update_time.is_some() {
            fields.push("update_time");
        }

        if row.remark.is_some() {
            fields.push("remark");
        }

        // 构建字段列表
        query.push(fields.join(", "));
        query.push(") VALUES (");

        // 添加参数占位符
        let placeholders: Vec<String> = fields.iter().map(|_| "?".to_string()).collect();
        query.push(placeholders.join(", "));
        query.push(")");

        // 使用QueryBuilder绑定参数值
        if !row.id.is_empty() {
            query.push_bind(&row.id);
        }

        if let Some(ref name) = row.name {
            query.push_bind(name);
        }

        if let Some(ref email) = row.email {
            query.push_bind(email);
        }

        if let Some(ref telephone) = row.telephone {
            query.push_bind(telephone);
        }

        if let Some(ref address) = row.address {
            query.push_bind(address);
        }

        if let Some(ref logo) = row.logo {
            query.push_bind(logo);
        }

        if let Some(ref parent_id) = row.parent_id {
            query.push_bind(parent_id);
        }

        if let Some(seq_no) = row.seq_no {
            query.push_bind(seq_no);
        }

        if let Some(status) = row.status {
            query.push_bind(status);
        }

        if let Some(ref create_by) = row.create_by {
            query.push_bind(create_by);
        }

        if let Some(create_time) = row.create_time {
            query.push_bind(create_time);
        }

        if let Some(ref update_by) = row.update_by {
            query.push_bind(update_by);
        }

        if let Some(update_time) = row.update_time {
            query.push_bind(update_time);
        }

        if let Some(ref remark) = row.remark {
            query.push_bind(remark);
        }

        let sql = query.build();
        sql.execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {DEPT_FIELDS} FROM sys_dept WHERE id = ?");

        sqlx::query_as::<_, Dept>(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 根据父部门ID查询部门
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {DEPT_FIELDS} FROM sys_dept WHERE parent_id = ?");

        sqlx::query_as::<_, Dept>(&sql)
            .bind(parent_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let mut query = sqlx::QueryBuilder::new("");
        self.build_select_dept_list_query_with_builder(&mut query, row);

        let sql = query.build_query_as::<Dept>();
        sql.fetch_all(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
    }

    /// 根据主键更新部门
    async fn update_by_primary_key(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let mut query = sqlx::QueryBuilder::new("UPDATE sys_dept SET ");
        let mut first = true;

        // 处理所有字段的更新
        if let Some(ref name) = row.name {
            if !first {
                query.push(", ");
            }
            query.push("name = ");
            query.push_bind(name);
            first = false;
        }

        if let Some(ref email) = row.email {
            if !first {
                query.push(", ");
            }
            query.push("email = ");
            query.push_bind(email);
            first = false;
        }

        if let Some(ref telephone) = row.telephone {
            if !first {
                query.push(", ");
            }
            query.push("telephone = ");
            query.push_bind(telephone);
            first = false;
        }

        if let Some(ref address) = row.address {
            if !first {
                query.push(", ");
            }
            query.push("address = ");
            query.push_bind(address);
            first = false;
        }

        if let Some(ref logo) = row.logo {
            if !first {
                query.push(", ");
            }
            query.push("logo = ");
            query.push_bind(logo);
            first = false;
        }

        if let Some(ref parent_id) = row.parent_id {
            if !first {
                query.push(", ");
            }
            query.push("parent_id = ");
            query.push_bind(parent_id);
            first = false;
        }

        if let Some(seq_no) = row.seq_no {
            if !first {
                query.push(", ");
            }
            query.push("seq_no = ");
            query.push_bind(seq_no);
            first = false;
        }

        if let Some(status) = row.status {
            if !first {
                query.push(", ");
            }
            query.push("status = ");
            query.push_bind(status);
            first = false;
        }

        if let Some(ref create_by) = row.create_by {
            if !first {
                query.push(", ");
            }
            query.push("create_by = ");
            query.push_bind(create_by);
            first = false;
        }

        if let Some(create_time) = row.create_time {
            if !first {
                query.push(", ");
            }
            query.push("create_time = ");
            query.push_bind(create_time);
            first = false;
        }

        if let Some(ref update_by) = row.update_by {
            if !first {
                query.push(", ");
            }
            query.push("update_by = ");
            query.push_bind(update_by);
            first = false;
        }

        if let Some(update_time) = row.update_time {
            if !first {
                query.push(", ");
            }
            query.push("update_time = ");
            query.push_bind(update_time);
            first = false;
        }

        if let Some(ref remark) = row.remark {
            if !first {
                query.push(", ");
            }
            query.push("remark = ");
            query.push_bind(remark);
        }

        query.push(" WHERE id = ");
        query.push_bind(&row.id);

        let sql = query.build();
        let result = sql
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        Ok(result.rows_affected())
    }

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let mut query = sqlx::QueryBuilder::new("UPDATE sys_dept SET ");
        let mut first = true;

        // 构建动态更新语句
        // 处理Option类型的字段
        if let Some(ref name) = row.name {
            if !first {
                query.push(", ");
            }
            query.push("name = ");
            query.push_bind(name);
            first = false;
        }

        if let Some(ref email) = row.email {
            if !first {
                query.push(", ");
            }
            query.push("email = ");
            query.push_bind(email);
            first = false;
        }

        if let Some(ref telephone) = row.telephone {
            if !first {
                query.push(", ");
            }
            query.push("telephone = ");
            query.push_bind(telephone);
            first = false;
        }

        if let Some(ref address) = row.address {
            if !first {
                query.push(", ");
            }
            query.push("address = ");
            query.push_bind(address);
            first = false;
        }

        if let Some(ref logo) = row.logo {
            if !first {
                query.push(", ");
            }
            query.push("logo = ");
            query.push_bind(logo);
            first = false;
        }

        if let Some(ref parent_id) = row.parent_id {
            if !first {
                query.push(", ");
            }
            query.push("parent_id = ");
            query.push_bind(parent_id);
            first = false;
        }

        if let Some(ref create_by) = row.create_by {
            if !first {
                query.push(", ");
            }
            query.push("create_by = ");
            query.push_bind(create_by);
            first = false;
        }

        if let Some(ref update_by) = row.update_by {
            if !first {
                query.push(", ");
            }
            query.push("update_by = ");
            query.push_bind(update_by);
            first = false;
        }

        if let Some(ref remark) = row.remark {
            if !first {
                query.push(", ");
            }
            query.push("remark = ");
            query.push_bind(remark);
        }

        // 处理非Option类型的字段
        // 确保在添加这些字段前，`first` 标志被正确更新
        let mut field_added = false;

        // 检查seq_no字段是否需要更新
        if !field_added && first {
            // 如果这是第一个字段
            field_added = true;
            first = false;
        } else if !field_added {
            // 如果这不是第一个字段且有其他字段已添加
            query.push(", ");
        }

        query.push("seq_no = ");
        query.push_bind(&row.seq_no);

        if !first {
            query.push(", ");
        }
        query.push("status = ");
        query.push_bind(&row.status);

        if !first {
            query.push(", ");
        }
        query.push("create_time = ");
        query.push_bind(&row.create_time);

        if !first {
            query.push(", ");
        }
        query.push("update_time = ");
        query.push_bind(&row.update_time);

        if !field_added {
            // 没有更新任何字段
            return Ok(0);
        }

        query.push(" WHERE id = ");
        query.push_bind(&row.id);

        let sql = query.build();
        let result = sql
            .execute(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

        Ok(result.rows_affected())
    }
}
