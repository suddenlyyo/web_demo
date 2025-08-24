//! 部门数据访问层 Diesel 实现

use diesel::prelude::*;
use diesel::sql_types::Text;

use crate::models::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;

/// 部门表的所有字段，用于SQL查询
const DEPT_FIELDS: &str = "id, parent_id, name, email, telephone, address, logo, dept_level, seq_no, status, create_by, create_time, update_by, update_time, remark";

/// 用于获取COUNT查询结果的结构体
#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: u64,
}

/// 部门数据访问 Diesel 实现
#[derive(Debug)]
pub struct DeptRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl DeptRepositoryDieselImpl {
    /// 创建部门仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl DeptRepository for DeptRepositoryDieselImpl {
    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("DELETE FROM sys_dept WHERE id = ?")
            .bind::<Text, _>(id)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("部门删除失败"));
        }

        Ok(())
    }

    /// 插入部门记录
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("INSERT INTO sys_dept (id, parent_id, name, email, telephone, address, logo, dept_level, seq_no, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind::<Text, _>(&row.id)
            .bind::<Text, _>(&row.parent_id.clone().unwrap_or_default())
            .bind::<Text, _>(&row.name.clone().unwrap_or_default())
            .bind::<Text, _>(&row.email.clone().unwrap_or_default())
            .bind::<Text, _>(&row.telephone.clone().unwrap_or_default())
            .bind::<Text, _>(&row.address.clone().unwrap_or_default())
            .bind::<Text, _>(&row.logo.clone().unwrap_or_default())
            .bind::<Text, _>(&row.dept_level.clone().unwrap_or_default())
            .bind::<Integer, _>(row.seq_no.unwrap_or_default())
            .bind::<Integer, _>(row.status.unwrap_or_default())
            .bind::<Text, _>(&row.create_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.create_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.update_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.remark.clone().unwrap_or_default())
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("部门插入失败"));
        }

        Ok(())
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec!["id".to_string()];
        let mut placeholders = vec!["?".to_string()];
        let mut bindings: Vec<Box<dyn std::any::Any>> = vec![];
        
        bindings.push(Box::new(row.id.clone()) as Box<dyn std::any::Any>);

        if row.parent_id.is_some() {
            fields.push("parent_id".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.parent_id.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.name.is_some() {
            fields.push("name".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.name.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.email.is_some() {
            fields.push("email".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.email.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.telephone.is_some() {
            fields.push("telephone".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.telephone.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.address.is_some() {
            fields.push("address".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.address.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.logo.is_some() {
            fields.push("logo".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.logo.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.dept_level.is_some() {
            fields.push("dept_level".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.dept_level.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.seq_no.is_some() {
            fields.push("seq_no".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.seq_no.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.status.is_some() {
            fields.push("status".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.status.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_by.is_some() {
            fields.push("create_by".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.create_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_time.is_some() {
            fields.push("create_time".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.create_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.update_by.is_some() {
            fields.push("update_by".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.update_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.update_time.is_some() {
            fields.push("update_time".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.update_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.remark.is_some() {
            fields.push("remark".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.remark.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        let sql = format!(
            "INSERT INTO sys_dept ({}) VALUES ({})",
            fields.join(", "),
            placeholders.join(", ")
        );

        let result = sql_query(&sql)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("部门插入失败"));
        }

        Ok(())
    }

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        match sql_query(&format!("SELECT {} FROM sys_dept WHERE id = ?", DEPT_FIELDS))
            .bind::<Text, _>(id)
            .get_result::<Dept>(&mut self.connection) {
                Ok(dept) => Ok(Some(dept)),
                Err(_) => Ok(None),
            }
    }

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut bindings: Vec<Box<dyn std::any::Any>> = vec![];

        if row.parent_id.is_some() {
            updates.push("parent_id = ?".to_string());
            bindings.push(Box::new(row.parent_id.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.name.is_some() {
            updates.push("name = ?".to_string());
            bindings.push(Box::new(row.name.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.email.is_some() {
            updates.push("email = ?".to_string());
            bindings.push(Box::new(row.email.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.telephone.is_some() {
            updates.push("telephone = ?".to_string());
            bindings.push(Box::new(row.telephone.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.address.is_some() {
            updates.push("address = ?".to_string());
            bindings.push(Box::new(row.address.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.logo.is_some() {
            updates.push("logo = ?".to_string());
            bindings.push(Box::new(row.logo.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.dept_level.is_some() {
            updates.push("dept_level = ?".to_string());
            bindings.push(Box::new(row.dept_level.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.seq_no.is_some() {
            updates.push("seq_no = ?".to_string());
            bindings.push(Box::new(row.seq_no.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.status.is_some() {
            updates.push("status = ?".to_string());
            bindings.push(Box::new(row.status.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_by.is_some() {
            updates.push("create_by = ?".to_string());
            bindings.push(Box::new(row.create_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_time.is_some() {
            updates.push("create_time = ?".to_string());
            bindings.push(Box::new(row.create_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.update_by.is_some() {
            updates.push("update_by = ?".to_string());
            bindings.push(Box::new(row.update_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.update_time.is_some() {
            updates.push("update_time = ?".to_string());
            bindings.push(Box::new(row.update_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.remark.is_some() {
            updates.push("remark = ?".to_string());
            bindings.push(Box::new(row.remark.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if updates.is_empty() {
            return Ok(());
        }

        let sql = format!(
            "UPDATE sys_dept SET {} WHERE id = ?",
            updates.join(", ")
        );

        let mut query = sql_query(&sql);
        query = query.bind::<Text, _>(&row.id);

        let result = query.execute(&mut self.connection)?;
        if result == 0 {
            return Err(Box::from("部门更新失败"));
        }

        Ok(())
    }

    /// 根据主键更新部门
    async fn update_by_primary_key(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("UPDATE sys_dept SET parent_id = ?, name = ?, email = ?, telephone = ?, address = ?, logo = ?, dept_level = ?, seq_no = ?, status = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?")
            .bind::<Text, _>(&row.parent_id.clone().unwrap_or_default())
            .bind::<Text, _>(&row.name.clone().unwrap_or_default())
            .bind::<Text, _>(&row.email.clone().unwrap_or_default())
            .bind::<Text, _>(&row.telephone.clone().unwrap_or_default())
            .bind::<Text, _>(&row.address.clone().unwrap_or_default())
            .bind::<Text, _>(&row.logo.clone().unwrap_or_default())
            .bind::<Text, _>(&row.dept_level.clone().unwrap_or_default())
            .bind::<Integer, _>(row.seq_no.unwrap_or_default())
            .bind::<Integer, _>(row.status.unwrap_or_default())
            .bind::<Text, _>(&row.create_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.create_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.update_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.remark.clone().unwrap_or_default())
            .bind::<Text, _>(&row.id)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("部门更新失败"));
        }

        Ok(())
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut conditions = vec![];
        let mut bindings: Vec<Box<dyn std::any::Any>> = vec![];

        if let Some(parent_id) = &row.parent_id {
            conditions.push("parent_id = ?".to_string());
            bindings.push(Box::new(parent_id.clone()) as Box<dyn std::any::Any>);
        }

        if let Some(name) = &row.name {
            conditions.push("name = ?".to_string());
            bindings.push(Box::new(name.clone()) as Box<dyn std::any::Any>);
        }

        if let Some(status) = row.status {
            conditions.push("status = ?".to_string());
            bindings.push(Box::new(status) as Box<dyn std::any::Any>);
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!("SELECT {} FROM sys_dept {} ORDER BY seq_no", DEPT_FIELDS, where_clause);

        let result = sql_query(&sql)
            .load::<Dept>(&mut self.connection)?;

        Ok(result)
    }

    /// 根据父部门ID查询子部门列表
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query(&format!("SELECT {} FROM sys_dept WHERE parent_id = ? ORDER BY seq_no", DEPT_FIELDS))
            .bind::<Text, _>(parent_id)
            .load::<Dept>(&mut self.connection)?;

        Ok(result)
    }
}