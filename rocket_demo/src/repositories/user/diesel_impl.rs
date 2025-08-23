//! 用户数据访问层 Diesel 实现

use diesel::sql_types::{BigInt, Integer, Text, Timestamp};
use diesel::{QueryableByName, RunQueryDsl, sql_query};

use crate::models::{User, UserQuery};
use crate::repositories::user::UserRepository;
use common_wrapper::PageInfo;

/// 用户表的所有字段，用于SQL查询
const USER_FIELDS: &str = "id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark";

/// 用于获取COUNT查询结果的结构体
#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: u64,
}

/// 用户数据访问 Diesel 实现
#[derive(Debug)]
pub struct UserRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl UserRepositoryDieselImpl {
    /// 创建用户仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositoryDieselImpl {
    /// 根据ID获取用户信息
    async fn get_user_by_id(&self, id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询用户信息
        let user_query = sql_query("SELECT id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<User>(&mut self.connection)?;

        Ok(user_query)
    }

    /// 获取用户列表
    async fn list_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询用户列表
        let users_query = sql_query("SELECT id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark FROM sys_user").load::<User>(&mut self.connection)?;

        Ok(users_query)
    }

    /// 根据查询条件分页查询用户列表
    async fn list_users_by_query(&self, query: UserQuery) -> Result<(Vec<User>, u64, u64), Box<dyn std::error::Error + Send + Sync>> {
        // 直接使用已处理过的分页参数
        let current_page = query
            .current_page_num
            .unwrap_or(PageInfo::DEFAULT_CURRENT_PAGE);
        let page_size = query
            .page_size
            .unwrap_or(PageInfo::DEFAULT_PAGE_SIZE)
            .min(PageInfo::MAX_PAGE_SIZE);
        let offset = (current_page - 1) * page_size;

        // 构建动态查询SQL
        let mut sql = "SELECT id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE 1=1".to_string();
        let mut count_sql = "SELECT COUNT(*) as count FROM sys_user WHERE 1=1".to_string();
        let mut params: Vec<String> = Vec::new();

        // 添加查询条件
        if let Some(id) = &query.id {
            sql.push_str(" AND id = ?");
            count_sql.push_str(" AND id = ?");
            params.push(id.clone());
        }

        if let Some(name) = &query.name {
            sql.push_str(" AND name LIKE ?");
            count_sql.push_str(" AND name LIKE ?");
            params.push(format!("%{}%", name));
        }

        if let Some(dept_id) = &query.dept_id {
            sql.push_str(" AND dept_id = ?");
            count_sql.push_str(" AND dept_id = ?");
            params.push(dept_id.clone());
        }

        if let Some(email) = &query.email {
            sql.push_str(" AND email LIKE ?");
            count_sql.push_str(" AND email LIKE ?");
            params.push(format!("%{}%", email));
        }

        if let Some(phone_number) = &query.phone_number {
            sql.push_str(" AND phone_number LIKE ?");
            count_sql.push_str(" AND phone_number LIKE ?");
            params.push(format!("%{}%", phone_number));
        }

        if let Some(sex) = &query.sex {
            sql.push_str(" AND sex = ?");
            count_sql.push_str(" AND sex = ?");
            params.push(sex.clone());
        }

        if let Some(status) = query.status {
            sql.push_str(" AND status = ?");
            count_sql.push_str(" AND status = ?");
            params.push(status.to_string());
        }

        if let Some(remark) = &query.remark {
            sql.push_str(" AND remark LIKE ?");
            count_sql.push_str(" AND remark LIKE ?");
            params.push(format!("%{}%", remark));
        }

        if let Some(start_date) = query.start_date {
            sql.push_str(" AND create_time >= ?");
            count_sql.push_str(" AND create_time >= ?");
            params.push(start_date.to_rfc3339());
        }

        if let Some(end_date) = query.end_date {
            sql.push_str(" AND create_time <= ?");
            count_sql.push_str(" AND create_time <= ?");
            params.push(end_date.to_rfc3339());
        }

        sql.push_str(" ORDER BY create_time DESC LIMIT ? OFFSET ?");

        // 构建查询参数
        let mut query_builder = sql_query(&sql);
        // 绑定条件参数
        for param in &params {
            query_builder = query_builder.bind::<Text, _>(param);
        }
        // 绑定分页参数
        query_builder = query_builder
            .bind::<BigInt, _>(page_size as i64)
            .bind::<BigInt, _>(offset as i64);

        // 构建统计查询
        let mut count_query_builder = sql_query(&count_sql);
        for param in &params {
            count_query_builder = count_query_builder.bind::<Text, _>(param);
        }

        // 查询总记录数
        let total_count = match count_query {
            Ok(count_result) => count_result.count,
            Err(_) => {
                result
                    .base
                    .set_fail("Failed to fetch user count".to_string());
                return result;
            },
        };

        // 计算总页数
        let total_pages = (total_count + page_size - 1) / page_size;

        // 查询当前页数据
        let users_result = query_builder.load::<User>(&mut self.connection)?;

        Ok((users_result, total_count, total_pages))
    }

    /// 根据用户名查找用户
    async fn get_user_by_name(&self, name: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询用户信息
        let user_query = sql_query("SELECT id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE name = ?")
            .bind::<Text, _>(name)
            .get_result::<User>(&mut self.connection)?;

        Ok(user_query)
    }

    /// 根据部门ID查找用户列表
    async fn list_users_by_dept_id(&self, dept_id: &str) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询用户列表
        let users_query = sql_query("SELECT id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE dept_id = ?")
            .bind::<Text, _>(dept_id)
            .load::<User>(&mut self.connection)?;

        Ok(users_query)
    }

    /// 新增用户
    async fn add_user(&self, user: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel新增用户
        let insert_query = sql_query("INSERT INTO sys_user (id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind::<Text, _>(user.id)
            .bind::<Text, _>(user.dept_id.unwrap_or_default())
            .bind::<Text, _>(user.name.unwrap_or_default())
            .bind::<Text, _>(user.email.unwrap_or_default())
            .bind::<Text, _>(user.phone_number.unwrap_or_default())
            .bind::<Text, _>(user.sex.unwrap_or_default())
            .bind::<Text, _>(user.password.unwrap_or_default())
            .bind::<Text, _>(user.avatar.unwrap_or_default())
            .bind::<Integer, _>(user.status.unwrap_or_default())
            .bind::<Text, _>(user.login_ip.unwrap_or_default())
            .bind::<Timestamp, _>(user.login_time.unwrap_or_default())
            .bind::<Text, _>(user.create_by.unwrap_or_default())
            .bind::<Timestamp, _>(user.create_time.unwrap_or_default())
            .bind::<Text, _>(user.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(user.update_time.unwrap_or_default())
            .bind::<Text, _>(user.remark.unwrap_or_default());

        insert_query.execute(&mut self.connection)?;

        Ok(user)
    }

    /// 修改用户
    async fn update_user(&self, user: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改用户
        let update_query = sql_query("UPDATE sys_user SET dept_id = ?, name = ?, email = ?, phone_number = ?, sex = ?, password = ?, avatar = ?, status = ?, login_ip = ?, login_time = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?")
            .bind::<Text, _>(user.dept_id.unwrap_or_default())
            .bind::<Text, _>(user.name.unwrap_or_default())
            .bind::<Text, _>(user.email.unwrap_or_default())
            .bind::<Text, _>(user.phone_number.unwrap_or_default())
            .bind::<Text, _>(user.sex.unwrap_or_default())
            .bind::<Text, _>(user.password.unwrap_or_default())
            .bind::<Text, _>(user.avatar.unwrap_or_default())
            .bind::<Integer, _>(user.status.unwrap_or_default())
            .bind::<Text, _>(user.login_ip.unwrap_or_default())
            .bind::<Timestamp, _>(user.login_time.unwrap_or_default())
            .bind::<Text, _>(user.create_by.unwrap_or_default())
            .bind::<Timestamp, _>(user.create_time.unwrap_or_default())
            .bind::<Text, _>(user.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(user.update_time.unwrap_or_default())
            .bind::<Text, _>(user.remark.unwrap_or_default())
            .bind::<Text, _>(user.id);

        update_query.execute(&mut self.connection)?;

        Ok(user)
    }

    /// 删除用户
    async fn delete_user(&self, id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel删除用户
        let delete_query = sql_query("DELETE FROM sys_user WHERE id = ?").bind::<Text, _>(id);
        delete_query.execute(&mut self.connection)?;

        // 查询删除的用户信息（模拟返回）
        let user = User { id: id.to_string(), ..Default::default() };

        Ok(user)
    }

    /// 修改用户状态
    async fn update_user_status(&self, id: &str, status: i32) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改用户状态
        let update_query = sql_query("UPDATE sys_user SET status = ?, update_time = CURRENT_TIMESTAMP WHERE id = ?")
            .bind::<Integer, _>(status)
            .bind::<Text, _>(id);

        update_query.execute(&mut self.connection)?;

        // 查询更新后的用户信息
        let user_query = sql_query("SELECT id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<User>(&mut self.connection)?;

        Ok(user_query)
    }
}
