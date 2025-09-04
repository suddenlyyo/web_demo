//! 部门数据访问层 Diesel 实现

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;

use crate::models::Dept;
use crate::models::constants::DEPT_FIELDS;
use crate::repositories::dept::dept_repository::DeptRepository;

table! {
    sys_dept (id) {
        id -> Text,
        parent_id -> Nullable<Text>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        telephone -> Nullable<Text>,
        address -> Nullable<Text>,
        logo -> Nullable<Text>,
        dept_level -> Nullable<Text>,
        seq_no -> Nullable<Integer>,
        status -> Nullable<Integer>,
        create_by -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_by -> Nullable<Text>,
        update_time -> Nullable<Timestamp>,
        remark -> Nullable<Text>,
    }
}

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_dept)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
    create_time: Option<chrono::NaiveDateTime>,
    update_by: Option<String>,
    update_time: Option<chrono::NaiveDateTime>,
    remark: Option<String>,
}

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
                .create_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: row.remark,
        }
    }
}

impl From<&Dept> for DeptRow {
    fn from(dept: &Dept) -> Self {
        DeptRow {
            id: dept.id.clone(),
            parent_id: dept.parent_id.clone(),
            name: dept.name.clone(),
            email: dept.email.clone(),
            telephone: dept.telephone.clone(),
            address: dept.address.clone(),
            logo: dept.logo.clone(),
            dept_level: dept.dept_level.clone(),
            seq_no: dept.seq_no,
            status: dept.status,
            create_by: dept.create_by.clone(),
            create_time: dept.create_time.map(|t| t.naive_utc()),
            update_by: dept.update_by.clone(),
            update_time: dept.update_time.map(|t| t.naive_utc()),
            remark: dept.remark.clone(),
        }
    }
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
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

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
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

        let dept_row: DeptRow = row.into();
        diesel::insert_into(sys_dept)
            .values(&dept_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

        let dept_row: DeptRow = row.into();
        diesel::insert_into(sys_dept)
            .values(&dept_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据主键查询部门
    async fn select_dept_by_id(&self, dept_id: &str) -> Result<Option<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

        let result = sys_dept
            .filter(id.eq(dept_id))
            .first::<DeptRow>(&mut self.connection)
            .optional()?;

        Ok(result.map(Dept::from))
    }

    /// 查询部门列表
    async fn select_dept_list(&self, dept_param: crate::services::params::user_param::DeptParam) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

        let mut query = sys_dept.into_boxed();

        if let Some(name_filter) = dept_param.name {
            query = query.filter(name.like(format!("%{}%", name_filter)));
        }

        if let Some(status_filter) = dept_param.status {
            query = query.filter(status.eq(status_filter));
        }

        let result = query
            .order(id.asc())
            .load::<DeptRow>(&mut self.connection)?;

        Ok(result.into_iter().map(Dept::from).collect())
    }

    /// 根据主键更新部门
    async fn update_by_id(&self, row: &Dept) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

        let dept_row: DeptRow = row.into();
        let result = diesel::update(sys_dept.filter(id.eq(&row.id)))
            .set(&dept_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键选择性更新部门
    async fn update_by_id_selective(&self, row: &Dept) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

        let dept_row: DeptRow = row.into();
        let result = diesel::update(sys_dept.filter(id.eq(&row.id)))
            .set(&dept_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键删除部门
    async fn delete_by_id(&self, dept_id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::dept::diesel_impl::sys_dept::dsl::*;

        let result = diesel::delete(sys_dept.filter(id.eq(dept_id))).execute(&mut self.connection)?;
        Ok(result as u64)
    }
}
