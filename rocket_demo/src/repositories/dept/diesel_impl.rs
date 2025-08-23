//! 部门数据访问层 Diesel 实现

use diesel::sql_types::{BigInt, Integer, Text, Timestamp};
use diesel::{QueryableByName, RunQueryDsl, sql_query};

use crate::models::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;
use common_wrapper::PageInfo;

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
    /// 根据ID获取部门信息
    async fn get_dept_by_id(&self, id: &str) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询部门信息
        let dept_query = sql_query("SELECT id, parent_id, name, sort, leader, phone, email, status, create_by, create_time, update_by, update_time, remark FROM sys_dept WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<Dept>(&mut self.connection)?;

        Ok(dept_query)
    }

    /// 获取部门列表
    async fn list_depts(&self) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询部门列表
        let depts_query = sql_query("SELECT id, parent_id, name, sort, leader, phone, email, status, create_by, create_time, update_by, update_time, remark FROM sys_dept ORDER BY sort").load::<Dept>(&mut self.connection)?;

        Ok(depts_query)
    }

    /// 分页查询部门列表
    async fn list_depts_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Dept>, u64, u64), Box<dyn std::error::Error + Send + Sync>> {
        // 处理分页参数
        let current_page = page_num.unwrap_or(PageInfo::DEFAULT_CURRENT_PAGE);
        let page_size = page_size
            .unwrap_or(PageInfo::DEFAULT_PAGE_SIZE)
            .min(PageInfo::MAX_PAGE_SIZE);
        let offset = (current_page - 1) * page_size;

        // 构建查询SQL
        let sql = "SELECT id, parent_id, name, sort, leader, phone, email, status, create_by, create_time, update_by, update_time, remark FROM sys_dept ORDER BY sort LIMIT ? OFFSET ?";

        // 构建统计查询
        let count_sql = "SELECT COUNT(*) as count FROM sys_dept";

        // 查询总记录数
        let count_result = sql_query(count_sql).get_result::<CountResult>(&mut self.connection)?;
        let total_count = count_result.count;

        // 计算总页数
        let total_pages = (total_count + page_size - 1) / page_size;

        // 查询当前页数据
        let depts_result = sql_query(&sql)
            .bind::<BigInt, _>(page_size as i64)
            .bind::<BigInt, _>(offset as i64)
            .load::<Dept>(&mut self.connection)?;

        Ok((depts_result, total_count, total_pages))
    }

    /// 新增部门
    async fn add_dept(&self, dept: Dept) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel新增部门
        let insert_query = sql_query("INSERT INTO sys_dept (id, parent_id, name, sort, leader, phone, email, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind::<Text, _>(dept.id)
            .bind::<Text, _>(dept.parent_id)
            .bind::<Text, _>(dept.name)
            .bind::<Integer, _>(dept.sort)
            .bind::<Text, _>(dept.leader.unwrap_or_default())
            .bind::<Text, _>(dept.phone.unwrap_or_default())
            .bind::<Text, _>(dept.email.unwrap_or_default())
            .bind::<Integer, _>(dept.status)
            .bind::<Text, _>(dept.create_by.unwrap_or_default())
            .bind::<Timestamp, _>(dept.create_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(dept.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(dept.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(dept.remark.unwrap_or_default());

        insert_query.execute(&mut self.connection)?;

        Ok(dept)
    }

    /// 修改部门
    async fn update_dept(&self, dept: Dept) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改部门
        let update_query = sql_query("UPDATE sys_dept SET parent_id = ?, name = ?, sort = ?, leader = ?, phone = ?, email = ?, status = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?")
            .bind::<Text, _>(dept.parent_id)
            .bind::<Text, _>(dept.name)
            .bind::<Integer, _>(dept.sort)
            .bind::<Text, _>(dept.leader.unwrap_or_default())
            .bind::<Text, _>(dept.phone.unwrap_or_default())
            .bind::<Text, _>(dept.email.unwrap_or_default())
            .bind::<Integer, _>(dept.status)
            .bind::<Text, _>(dept.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(dept.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(dept.remark.unwrap_or_default())
            .bind::<Text, _>(dept.id);

        update_query.execute(&mut self.connection)?;

        Ok(dept)
    }

    /// 删除部门
    async fn delete_dept(&self, id: &str) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel删除部门
        let delete_query = sql_query("DELETE FROM sys_dept WHERE id = ?").bind::<Text, _>(id);
        delete_query.execute(&mut self.connection)?;

        // 查询删除的部门信息（模拟返回）
        let dept = Dept { id: id.to_string(), ..Default::default() };

        Ok(dept)
    }

    /// 修改部门状态
    async fn update_dept_status(&self, id: &str, status: i32) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改部门状态
        let update_query = sql_query("UPDATE sys_dept SET status = ?, update_time = CURRENT_TIMESTAMP WHERE id = ?")
            .bind::<Integer, _>(status)
            .bind::<Text, _>(id);

        update_query.execute(&mut self.connection)?;

        // 查询更新后的部门信息
        let dept_query = sql_query("SELECT id, parent_id, name, sort, leader, phone, email, status, create_by, create_time, update_by, update_time, remark FROM sys_dept WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<Dept>(&mut self.connection)?;

        Ok(dept_query)
    }
}
