//! 用户数据访问层 Diesel 实现

use diesel::sql_types::{BigInt, Integer, Text, Timestamp};
use diesel::{QueryableByName, RunQueryDsl, sql_query};

use crate::models::{User, UserQuery};
use crate::repositories::user::user_repository::UserRepository;
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

    /// 构建查询条件
    fn build_where_clause(query: &UserQuery) -> (String, Vec<String>) {
        let mut where_conditions = Vec::new();
        let mut params = Vec::new();

        // 添加ID查询条件
        if let Some(id) = &query.id {
            where_conditions.push("id = ?");
            params.push(id.clone());
        }

        // 添加名称查询条件
        if let Some(name) = &query.name {
            where_conditions.push("name LIKE ?");
            params.push(format!("%{}%", name));
        }

        // 添加部门ID查询条件
        if let Some(dept_id) = &query.dept_id {
            where_conditions.push("dept_id = ?");
            params.push(dept_id.clone());
        }

        // 添加邮箱查询条件
        if let Some(email) = &query.email {
            where_conditions.push("email LIKE ?");
            params.push(format!("%{}%", email));
        }

        // 添加手机号码查询条件
        if let Some(phone_number) = &query.phone_number {
            where_conditions.push("phone_number LIKE ?");
            params.push(format!("%{}%", phone_number));
        }

        // 添加性别查询条件
        if let Some(sex) = &query.sex {
            where_conditions.push("sex = ?");
            params.push(sex.clone());
        }

        // 添加状态查询条件
        if let Some(status) = query.status {
            where_conditions.push("status = ?");
            params.push(status.to_string());
        }

        // 添加备注查询条件
        if let Some(remark) = &query.remark {
            where_conditions.push("remark LIKE ?");
            params.push(format!("%{}%", remark));
        }

        // 添加日期范围查询条件
        if let (Some(start_date), Some(end_date)) = (&query.start_date, &query.end_date) {
            where_conditions.push("create_time BETWEEN ? AND ?");
            params.push(start_date.naive_utc().to_string());
            params.push(end_date.naive_utc().to_string());
        } else if let Some(start_date) = &query.start_date {
            where_conditions.push("create_time >= ?");
            params.push(start_date.naive_utc().to_string());
        } else if let Some(end_date) = &query.end_date {
            where_conditions.push("create_time <= ?");
            params.push(end_date.naive_utc().to_string());
        }

        let where_clause = if !where_conditions.is_empty() { format!("WHERE {}", where_conditions.join(" AND ")) } else { String::new() };

        (where_clause, params)
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositoryDieselImpl {
    /// 根据ID获取用户信息
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询用户信息
        let result = sql_query(format!("SELECT {} FROM sys_user WHERE id = ?", USER_FIELDS))
            .bind::<Text, _>(id)
            .get_result::<User>(&mut self.connection);

        match result {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// 根据用户名查找用户
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query(format!("SELECT {} FROM sys_user WHERE name = ?", USER_FIELDS))
            .bind::<Text, _>(name)
            .get_result::<User>(&mut self.connection);

        match result {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// 查询用户列表
    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let user_query: UserQuery = user.into();
        let (where_clause, _params) = Self::build_where_clause(&user_query);

        let sql = format!("SELECT {} FROM sys_user {}", USER_FIELDS, where_clause);
        let users_query = sql_query(&sql).load::<User>(&mut self.connection)?;
        Ok(users_query)
    }

    /// 获取用户列表数量
    async fn get_user_list_count(&self, query: &UserQuery) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let (where_clause, _params) = Self::build_where_clause(query);

        let sql = format!("SELECT COUNT(*) as count FROM sys_user {}", where_clause);
        let count_result = sql_query(&sql).get_result::<CountResult>(&mut self.connection)?;
        Ok(count_result.count)
    }

    /// 分页获取用户列表
    async fn get_user_list_by_page(&self, query: &UserQuery) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let page_info = PageInfo::new(query.current_page_num, query.page_size);
        let offset = page_info.get_page_offset();
        let limit = page_info.get_page_size();

        let (where_clause, _params) = Self::build_where_clause(query);

        let sql = format!("SELECT {} FROM sys_user {} ORDER BY create_time DESC LIMIT ? OFFSET ?", USER_FIELDS, where_clause);
        let users_query = sql_query(&sql)
            .bind::<Integer, _>(limit as i32)
            .bind::<Integer, _>(offset as i32)
            .load::<User>(&mut self.connection)?;

        Ok(users_query)
    }

    /// 插入用户记录
    async fn insert(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建插入语句
        let result = sql_query("INSERT INTO sys_user (id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind::<Text, _>(&user.id)
            .bind::<Text, _>(&user.dept_id.clone().unwrap_or_default())
            .bind::<Text, _>(&user.name.clone().unwrap_or_default())
            .bind::<Text, _>(&user.email.clone().unwrap_or_default())
            .bind::<Text, _>(&user.phone_number.clone().unwrap_or_default())
            .bind::<Text, _>(&user.sex.clone().unwrap_or_default())
            .bind::<Text, _>(&user.password.clone().unwrap_or_default())
            .bind::<Text, _>(&user.avatar.clone().unwrap_or_default())
            .bind::<Integer, _>(user.status.unwrap_or(0))
            .bind::<Text, _>(&user.login_ip.clone().unwrap_or_default())
            .bind::<Text, _>("") // login_time
            .bind::<Text, _>(&user.create_by.clone().unwrap_or_default())
            .bind::<Text, _>("") // create_time
            .bind::<Text, _>(&user.update_by.clone().unwrap_or_default())
            .bind::<Text, _>("") // update_time
            .bind::<Text, _>(&user.remark.clone().unwrap_or_default())
            .execute(&mut self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// 选择性插入用户记录
    async fn insert_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 与insert方法实现相同，在实际应用中可以根据需要进行区分
        self.insert(user).await
    }

    /// 根据ID更新用户信息
    async fn update_by_primary_key(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("UPDATE sys_user SET dept_id = ?, name = ?, email = ?, phone_number = ?, sex = ?, password = ?, avatar = ?, status = ?, login_ip = ?, login_time = ?, create_by = ?, create_time = ?, update_by = ?, update_time = datetime('now'), remark = ? WHERE id = ?")
            .bind::<Text, _>(&user.dept_id.clone().unwrap_or_default())
            .bind::<Text, _>(&user.name.clone().unwrap_or_default())
            .bind::<Text, _>(&user.email.clone().unwrap_or_default())
            .bind::<Text, _>(&user.phone_number.clone().unwrap_or_default())
            .bind::<Text, _>(&user.sex.clone().unwrap_or_default())
            .bind::<Text, _>(&user.password.clone().unwrap_or_default())
            .bind::<Text, _>(&user.avatar.clone().unwrap_or_default())
            .bind::<Integer, _>(user.status.unwrap_or(0))
            .bind::<Text, _>(&user.login_ip.clone().unwrap_or_default())
            .bind::<Text, _>("") // login_time
            .bind::<Text, _>(&user.create_by.clone().unwrap_or_default())
            .bind::<Text, _>("") // create_time
            .bind::<Text, _>(&user.update_by.clone().unwrap_or_default())
            .bind::<Text, _>(&user.remark.clone().unwrap_or_default())
            .bind::<Text, _>(&user.id)
            .execute(&mut self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// 根据ID选择性更新用户信息
    async fn update_by_primary_key_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 与update_by_primary_key方法实现相同，在实际应用中可以根据需要进行区分
        self.update_by_primary_key(user).await
    }

    /// 根据ID删除用户
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("DELETE FROM sys_user WHERE id = ?")
            .bind::<Text, _>(id)
            .execute(&mut self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
