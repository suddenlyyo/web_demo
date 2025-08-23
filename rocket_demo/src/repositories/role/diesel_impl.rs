//! 角色数据访问层 Diesel 实现

use diesel::sql_types::{BigInt, Integer, Text, Timestamp};
use diesel::{QueryableByName, RunQueryDsl, sql_query};

use crate::models::Role;
use crate::repositories::role::role_repository::RoleRepository;
use common_wrapper::PageInfo;

/// 角色表的所有字段，用于SQL查询
const ROLE_FIELDS: &str = "id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark";

/// 用于获取COUNT查询结果的结构体
#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: u64,
}

/// 角色数据访问 Diesel 实现
#[derive(Debug)]
pub struct RoleRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl RoleRepositoryDieselImpl {
    /// 创建角色仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositoryDieselImpl {
    /// 根据ID获取角色信息
    async fn get_role_by_id(&self, id: &str) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询角色信息
        let role_query = sql_query("SELECT id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark FROM sys_role WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<Role>(&mut self.connection)?;

        Ok(role_query)
    }

    /// 获取角色列表
    async fn list_roles(&self) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询角色列表
        let roles_query = sql_query("SELECT id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark FROM sys_role ORDER BY seq_no").load::<Role>(&mut self.connection)?;

        Ok(roles_query)
    }

    /// 分页查询角色列表
    async fn list_roles_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Role>, u64, u64), Box<dyn std::error::Error + Send + Sync>> {
        // 处理分页参数
        let current_page = page_num.unwrap_or(PageInfo::DEFAULT_CURRENT_PAGE);
        let page_size = page_size
            .unwrap_or(PageInfo::DEFAULT_PAGE_SIZE)
            .min(PageInfo::MAX_PAGE_SIZE);
        let offset = (current_page - 1) * page_size;

        // 构建查询SQL
        let sql = "SELECT id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark FROM sys_role ORDER BY seq_no LIMIT ? OFFSET ?";

        // 构建统计查询
        let count_sql = "SELECT COUNT(*) as count FROM sys_role";

        // 查询总记录数
        let count_result = sql_query(count_sql).get_result::<CountResult>(&mut self.connection)?;
        let total_count = count_result.count;

        // 计算总页数
        let total_pages = (total_count + page_size - 1) / page_size;

        // 查询当前页数据
        let roles_result = sql_query(&sql)
            .bind::<BigInt, _>(page_size as i64)
            .bind::<BigInt, _>(offset as i64)
            .load::<Role>(&mut self.connection)?;

        Ok((roles_result, total_count, total_pages))
    }

    /// 新增角色
    async fn add_role(&self, role: Role) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel新增角色
        let insert_query = sql_query("INSERT INTO sys_role (id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind::<Text, _>(role.id)
            .bind::<Text, _>(role.name)
            .bind::<Text, _>(role.role_key)
            .bind::<Integer, _>(role.seq_no)
            .bind::<Integer, _>(role.status)
            .bind::<Text, _>(role.create_by.unwrap_or_default())
            .bind::<Timestamp, _>(role.create_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(role.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(role.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(role.remark.unwrap_or_default());

        insert_query.execute(&mut self.connection)?;

        Ok(role)
    }

    /// 修改角色
    async fn update_role(&self, role: Role) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改角色
        let update_query = sql_query("UPDATE sys_role SET name = ?, role_key = ?, seq_no = ?, status = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?")
            .bind::<Text, _>(role.name)
            .bind::<Text, _>(role.role_key)
            .bind::<Integer, _>(role.seq_no)
            .bind::<Integer, _>(role.status)
            .bind::<Text, _>(role.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(role.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(role.remark.unwrap_or_default())
            .bind::<Text, _>(role.id);

        update_query.execute(&mut self.connection)?;

        Ok(role)
    }

    /// 删除角色
    async fn delete_role(&self, id: &str) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel删除角色
        let delete_query = sql_query("DELETE FROM sys_role WHERE id = ?").bind::<Text, _>(id);
        delete_query.execute(&mut self.connection)?;

        // 查询删除的角色信息（模拟返回）
        let role = Role { id: id.to_string(), ..Default::default() };

        Ok(role)
    }

    /// 修改角色状态
    async fn update_role_status(&self, id: &str, status: i32) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改角色状态
        let update_query = sql_query("UPDATE sys_role SET status = ?, update_time = CURRENT_TIMESTAMP WHERE id = ?")
            .bind::<Integer, _>(status)
            .bind::<Text, _>(id);

        update_query.execute(&mut self.connection)?;

        // 查询更新后的角色信息
        let role_query = sql_query("SELECT id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark FROM sys_role WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<Role>(&mut self.connection)?;

        Ok(role_query)
    }
}
