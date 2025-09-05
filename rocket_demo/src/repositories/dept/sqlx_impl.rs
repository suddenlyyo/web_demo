// ==================== 数据库连接 ====================
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::OnceLock;

use crate::models::Dept;
use crate::models::constants::DEPT_FIELDS;
use crate::repositories::dept::dept_repository::DeptRepository;
use crate::services::dept::dept_service::DeptParam;

// 数据库连接池
static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

/// SQLx实现的部门数据访问
#[derive(Debug)]
pub struct DeptRepositorySqlxImpl {
    pool: MySqlPool,
}

impl DeptRepositorySqlxImpl {
    /// 创建新的部门数据访问实例
    pub fn new() -> Self {
        let pool = DB_POOL.get().expect("数据库连接池未初始化").clone();
        Self { pool }
    }

    /// 初始化数据库连接池
    pub fn init_pool(pool: MySqlPool) {
        DB_POOL.set(pool).ok(); // 如果已经设置过，则忽略
    }
}

// ==================== 表结构体映射 ====================
/// SQLx的部门实体映射
#[derive(Debug, FromRow)]
struct DeptRow {
    id: String,
    parent_id: Option<String>,
    name: Option<String>,
    email: Option<String>,
    telephone: Option<String>,
    address: Option<String>,
    logo: Option<String>,
    dept_level: Option<String>,
    seq_no: Option<i32>,
    status: Option<i32>,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<NaiveDateTime>,
    update_by: Option<String>,
    #[sqlx(rename = "update_time")]
    update_time_raw: Option<NaiveDateTime>,
    remark: Option<String>,
}

// 实现从DeptRow到Dept的转换
impl From<DeptRow> for Dept {
    fn from(row: DeptRow) -> Self {
        Dept {
            id: row.id,
            parent_id: row.parent_id,
            name: row.name,
            email: row.email,
            telephone: row.telephone,
            address: row.address,
            logo: row.logo,
            dept_level: row.dept_level,
            seq_no: row.seq_no,
            status: row.status,
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

// ==================== SQL trait 实现 ====================
#[rocket::async_trait]
impl DeptRepository for DeptRepositorySqlxImpl {
    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_dept WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;

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
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

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

        let sql = format!("INSERT INTO sys_dept ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let query = sqlx::query(&sql).bind_all(params);
        let result = query.execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("部门插入失败"));
        }

        Ok(())
    }

    /// 根据主键查询部门
    async fn select_dept_by_id(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_dept WHERE id = ?", DEPT_FIELDS);
        let result: Option<DeptRow> = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(Dept::from))
    }

    /// 查询部门列表
    async fn select_dept_list(&self, dept_param: DeptParam) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let mut sql = format!("SELECT {} FROM sys_dept WHERE 1=1", DEPT_FIELDS);
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if let Some(name) = dept_param.name {
            sql.push_str(" AND name LIKE ?");
            params.push(Box::new(format!("%{}%", name)));
        }

        if let Some(status) = dept_param.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        sql.push_str(" ORDER BY id");

        // 构建查询
        let mut query = sqlx::query_as::<_, DeptRow>(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(&self.pool).await?;
        Ok(result.into_iter().map(Dept::from).collect())
    }

    /// 根据主键更新部门
    async fn update_by_id(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
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

        Ok(result.rows_affected())
    }

    /// 根据主键选择性更新部门
    async fn update_by_id_selective(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let mut sets = vec![];
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if row.parent_id.is_some() {
            sets.push("parent_id = ?");
            params.push(Box::new(&row.parent_id));
        }

        if row.name.is_some() {
            sets.push("name = ?");
            params.push(Box::new(&row.name));
        }

        if row.email.is_some() {
            sets.push("email = ?");
            params.push(Box::new(&row.email));
        }

        if row.telephone.is_some() {
            sets.push("telephone = ?");
            params.push(Box::new(&row.telephone));
        }

        if row.address.is_some() {
            sets.push("address = ?");
            params.push(Box::new(&row.address));
        }

        if row.logo.is_some() {
            sets.push("logo = ?");
            params.push(Box::new(&row.logo));
        }

        if row.dept_level.is_some() {
            sets.push("dept_level = ?");
            params.push(Box::new(&row.dept_level));
        }

        if row.seq_no.is_some() {
            sets.push("seq_no = ?");
            params.push(Box::new(&row.seq_no));
        }

        if row.status.is_some() {
            sets.push("status = ?");
            params.push(Box::new(&row.status));
        }

        if row.create_by.is_some() {
            sets.push("create_by = ?");
            params.push(Box::new(&row.create_by));
        }

        if row.create_time.is_some() {
            sets.push("create_time = ?");
            params.push(Box::new(&row.create_time.map(|t| t.naive_utc())));
        }

        if row.update_by.is_some() {
            sets.push("update_by = ?");
            params.push(Box::new(&row.update_by));
        }

        if row.update_time.is_some() {
            sets.push("update_time = ?");
            params.push(Box::new(&row.update_time.map(|t| t.naive_utc())));
        }

        if row.remark.is_some() {
            sets.push("remark = ?");
            params.push(Box::new(&row.remark));
        }

        if sets.is_empty() {
            return Ok(0);
        }

        let mut sql = format!("UPDATE sys_dept SET {}", sets.join(", "));
        sql.push_str(" WHERE id = ?");
        params.push(Box::new(&row.id));

        // 构建查询
        let mut query = sqlx::query(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }
        query = query.bind(&row.id);

        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    /// 根据主键删除部门
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_dept WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }
}
