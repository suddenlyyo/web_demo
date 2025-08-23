use chrono::{NaiveDateTime, Utc};
use sqlx::Row;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;

use crate::models::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;
use common_wrapper::PageInfo;

/// 部门表的所有字段，用于SQL查询
const DEPT_FIELDS: &str = "id, parent_id, name, email, telephone, address, logo, dept_level, seq_no, status, create_by, create_time, update_by, update_time, remark";

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
    /// 根据ID获取部门信息
    async fn get_dept_by_id(&self, id: &str) -> Result<Dept, Box<dyn StdError + Send + Sync>> {
        // 从数据库查询部门信息
        let dept_query = sqlx::query(&format!("SELECT {} FROM sys_dept WHERE id = ?", DEPT_FIELDS))
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        // 使用部门专用映射函数
        let dept = DbMapper::map_to_dept(&dept_query)?;

        Ok(dept)
    }

    /// 获取部门列表
    async fn list_depts(&self) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        // 从数据库查询部门列表
        let depts_query = sqlx::query(&format!("SELECT {} FROM sys_dept ORDER BY seq_no", DEPT_FIELDS))
            .fetch_all(&self.pool)
            .await?;

        // 使用部门专用映射函数映射所有部门
        let depts: Result<Vec<Dept>, sqlx::Error> = depts_query
            .into_iter()
            .map(|row| DbMapper::map_to_dept(&row))
            .collect();

        Ok(depts?)
    }

    /// 分页查询部门列表
    async fn list_depts_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Dept>, u64, u64), Box<dyn StdError + Send + Sync>> {
        let page_info = PageInfo::new(page_num, page_size);
        let _current_page = page_info.get_current_page_num();
        let page_size_value = page_info.get_page_size();
        let offset = page_info.get_page_offset();

        // 查询总数
        let count_query = sqlx::query("SELECT COUNT(*) as count FROM sys_dept")
            .fetch_one(&self.pool)
            .await?;

        let total_count = u64::try_from(count_query.get::<i64, &str>("count"))?;
        let total_pages = (total_count + page_size_value - 1) / page_size_value;

        // 查询数据
        let depts_query = sqlx::query(&format!("SELECT {} FROM sys_dept ORDER BY seq_no LIMIT ? OFFSET ?", DEPT_FIELDS))
            .bind(page_size_value as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;

        let depts: Result<Vec<Dept>, _> = depts_query
            .iter()
            .map(|row| DbMapper::map_to_dept(row))
            .collect();
        let depts = depts?;

        Ok((depts, total_count, total_pages))
    }

    /// 根据父部门ID获取子部门列表
    async fn list_children_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        // 从数据库查询子部门列表
        let children_query = sqlx::query(&format!("SELECT {} FROM sys_dept WHERE parent_id = ? ORDER BY seq_no", DEPT_FIELDS))
            .bind(parent_id)
            .fetch_all(&self.pool)
            .await?;

        let children: Result<Vec<Dept>, _> = children_query
            .into_iter()
            .map(|row| DbMapper::map_to_dept(&row))
            .collect();
        let children = children?;

        Ok(children)
    }

    /// 获取部门树结构
    async fn list_dept_tree(&self) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        // 从数据库查询所有部门
        let all_depts_query = sqlx::query(&format!("SELECT {} FROM sys_dept ORDER BY parent_id, seq_no", DEPT_FIELDS))
            .fetch_all(&self.pool)
            .await?;

        let all_depts: Result<Vec<Dept>, _> = all_depts_query
            .into_iter()
            .map(|row| DbMapper::map_to_dept(&row))
            .collect();
        let all_depts = all_depts?;

        Ok(all_depts)
    }

    /// 添加部门
    async fn add_dept(&self, dept: Dept) -> Result<Dept, Box<dyn StdError + Send + Sync>> {
        let sql = "INSERT INTO sys_dept (id, parent_id, name, email, telephone, address, seq_no, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let _result = sqlx::query(sql)
            .bind(&dept.id)
            .bind(&dept.parent_id)
            .bind(&dept.name)
            .bind(&dept.email)
            .bind(&dept.telephone)
            .bind(&dept.address)
            .bind(dept.seq_no)
            .bind(dept.status)
            .bind(&dept.create_by)
            .bind(dept.create_time.map(|t| t.naive_utc()))
            .bind(&dept.update_by)
            .bind(dept.update_time.map(|t| t.naive_utc()))
            .bind(&dept.remark)
            .execute(&self.pool)
            .await?;

        // 验证影响行数
        if _result.rows_affected() == 0 {
            return Err(Box::from("部门插入失败"));
        }

        // 返回插入的部门
        Ok(dept)
    }

    /// 修改部门
    async fn update_dept(&self, dept: Dept) -> Result<Dept, Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_dept SET parent_id = ?, name = ?, email = ?, telephone = ?, address = ?, seq_no = ?, status = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";

        let _result = sqlx::query(sql)
            .bind(&dept.parent_id)
            .bind(&dept.name)
            .bind(&dept.email)
            .bind(&dept.telephone)
            .bind(&dept.address)
            .bind(dept.seq_no)
            .bind(dept.status)
            .bind(&dept.update_by)
            .bind(dept.update_time.map(|t| t.naive_utc()))
            .bind(&dept.remark)
            .bind(&dept.id)
            .execute(&self.pool)
            .await?;

        // 验证影响行数
        if _result.rows_affected() == 0 {
            return Err(Box::from("部门更新失败"));
        }

        // 返回更新的部门
        Ok(dept)
    }

    /// 删除部门
    async fn delete_dept(&self, id: &str) -> Result<Dept, Box<dyn StdError + Send + Sync>> {
        // 获取要删除的部门
        let dept_to_delete = self.get_dept_by_id(id).await?;

        // 构建SQL查询
        let sql = "DELETE FROM sys_dept WHERE id = ?";

        // 执行删除操作
        let _result = sqlx::query(sql).bind(id).execute(&self.pool).await?;

        // 验证影响行数
        if _result.rows_affected() == 0 {
            return Err(Box::from("部门删除失败"));
        }

        // 返回删除的部门
        Ok(dept_to_delete)
    }

    /// 更新部门状态
    async fn update_dept_status(&self, id: &str, status: i32) -> Result<Dept, Box<dyn StdError + Send + Sync>> {
        // 先获取部门信息
        let mut dept = self.get_dept_by_id(id).await?;

        // 执行更新状态操作
        let _result = sqlx::query("UPDATE sys_dept SET status = ?, update_time = NOW() WHERE id = ?")
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await?;

        // 更新内存中的部门状态
        dept.status = Some(status);
        Ok(dept)
    }
}
