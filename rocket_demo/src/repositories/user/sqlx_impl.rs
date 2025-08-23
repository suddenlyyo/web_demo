/// 用户表的所有字段，用于SQL查询
const USER_FIELDS: &str = "id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark";

/// 数据库映射器
struct DbMapper;

impl DbMapper {
    /// 将数据库行映射为用户对象
    fn map_to_user(row: &sqlx::mysql::MySqlRow) -> Result<User, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            dept_id: row.try_get("dept_id")?,
            name: row.try_get("name")?,
            email: row.try_get("email")?,
            phone_number: row.try_get("phone_number")?,
            sex: row.try_get("sex")?,
            avatar: row.try_get("avatar")?,
            password: row.try_get("password")?,
            status: row.try_get("status")?,
            login_ip: row.try_get("login_ip")?,
            login_time: row
                .try_get::<Option<chrono::NaiveDateTime>, _>("login_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            create_by: row.try_get("create_by")?,
            create_time: row
                .try_get::<Option<chrono::NaiveDateTime>, _>("create_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            update_by: row.try_get("update_by")?,
            update_time: row
                .try_get::<Option<chrono::NaiveDateTime>, _>("update_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            remark: row.try_get("remark")?,
        })
    }
}

/// 用户数据访问层 SQLx 实现
use chrono::Utc;
use std::error::Error as StdError;

use sqlx::{MySqlPool, Row};

use crate::models::user::User;
use crate::repositories::user::user_repository::UserRepository;
use common_wrapper::PageInfo;

/// 用户仓库SQLx实现
#[derive(Debug)]
pub struct UserRepositorySqlxImpl {
    pool: MySqlPool,
}

impl UserRepositorySqlxImpl {
    /// 从数据库URL创建用户仓库实例
    ///
    /// # 参数
    ///
    /// - `database_url`: 数据库连接URL
    ///
    /// # 返回值
    ///
    /// 返回用户仓库实例
    pub async fn from_database_url(database_url: &str) -> Self {
        let pool = MySqlPool::connect(database_url).await.unwrap();
        Self { pool }
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositorySqlxImpl {
    /// 根据ID获取用户信息
    async fn get_user_by_id(&self, id: &str) -> Result<User, Box<dyn StdError + Send + Sync>> {
        let user_query = sqlx::query(&format!("SELECT {} FROM sys_user WHERE id = ?", USER_FIELDS))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match user_query {
            Some(row) => {
                let user = DbMapper::map_to_user(&row)?;
                Ok(user)
            },
            None => Err("User not found".into()),
        }
    }

    /// 获取所有用户列表
    async fn list_users(&self) -> Result<Vec<User>, Box<dyn StdError + Send + Sync>> {
        let users_query = sqlx::query(&format!("SELECT {} FROM sys_user", USER_FIELDS))
            .fetch_all(&self.pool)
            .await?;

        let users: Result<Vec<User>, _> = users_query.iter().map(DbMapper::map_to_user).collect();
        Ok(users?)
    }

    /// 分页获取用户列表
    async fn list_users_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<User>, u64), Box<dyn StdError + Send + Sync>> {
        let page_info = PageInfo::new(page_num, page_size);
        let offset = page_info.get_page_offset();
        let limit = page_info.get_page_size();

        let user_query = format!("SELECT {} FROM sys_user ORDER BY create_time DESC LIMIT ? OFFSET ?", USER_FIELDS);
        let users_query = sqlx::query(&user_query)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;

        let users: Result<Vec<User>, _> = users_query.iter().map(DbMapper::map_to_user).collect();

        // 获取总记录数
        let count_query = "SELECT COUNT(*) as count FROM sys_user";
        let count_row = sqlx::query(count_query).fetch_one(&self.pool).await?;

        let total: u64 = count_row.try_get("count")?;
        Ok((users?, total))
    }

    /// 新增用户
    async fn add_user(&self, user: User) -> Result<User, Box<dyn StdError + Send + Sync>> {
        // 构建插入语句
        let login_time: Option<chrono::NaiveDateTime> = user.login_time.map(|t| t.naive_utc());
        let create_time: Option<chrono::NaiveDateTime> = user.create_time.map(|t| t.naive_utc());
        let update_time: Option<chrono::NaiveDateTime> = user.update_time.map(|t| t.naive_utc());

        let result = sqlx::query("INSERT INTO sys_user (id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&user.id)
            .bind(&user.dept_id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.phone_number)
            .bind(&user.sex)
            .bind(&user.password)
            .bind(&user.avatar)
            .bind(&user.status)
            .bind(&user.login_ip)
            .bind(&login_time)
            .bind(&user.create_by)
            .bind(&create_time)
            .bind(&user.update_by)
            .bind(&update_time)
            .bind(&user.remark)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 { Ok(user) } else { Err("Failed to add user".into()) }
    }

    /// 修改用户
    async fn update_user(&self, user: User) -> Result<User, Box<dyn StdError + Send + Sync>> {
        // 构建更新语句
        let login_time: Option<chrono::NaiveDateTime> = user.login_time.map(|t| t.naive_utc());
        let create_time: Option<chrono::NaiveDateTime> = user.create_time.map(|t| t.naive_utc());

        let result = sqlx::query("UPDATE sys_user SET dept_id = ?, name = ?, email = ?, phone_number = ?, sex = ?, password = ?, avatar = ?, status = ?, login_ip = ?, login_time = ?, create_by = ?, create_time = ?, update_by = ?, update_time = NOW(), remark = ? WHERE id = ?")
            .bind(&user.dept_id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.phone_number)
            .bind(&user.sex)
            .bind(&user.password)
            .bind(&user.avatar)
            .bind(&user.status)
            .bind(&user.login_ip)
            .bind(&login_time)
            .bind(&user.create_by)
            .bind(&create_time)
            .bind(&user.update_by)
            .bind(&user.remark)
            .bind(&user.id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 { Ok(user) } else { Err("Failed to update user".into()) }
    }

    /// 删除用户
    async fn delete_user(&self, id: &str) -> Result<User, Box<dyn StdError + Send + Sync>> {
        // 先查询用户信息
        let user = self.get_user_by_id(id).await?;

        // 执行删除操作
        let result = sqlx::query("DELETE FROM sys_user WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 { Ok(user) } else { Err("Failed to delete user".into()) }
    }

    /// 修改用户状态
    async fn update_user_status(&self, id: &str, status: i32) -> Result<User, Box<dyn StdError + Send + Sync>> {
        // 先查询用户信息
        let mut user = self.get_user_by_id(id).await?;

        // 执行更新操作
        let result = sqlx::query("UPDATE sys_user SET status = ?, update_time = NOW() WHERE id = ?")
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 {
            user.status = Some(status);
            Ok(user)
        } else {
            Err("Failed to update user status".into())
        }
    }

    /// 分页查询用户列表
    async fn list_users_by_page_with_conditions(&self, page_num: Option<u64>, page_size: Option<u64>, where_clause: String) -> Result<(Vec<User>, u64, u64), Box<dyn StdError + Send + Sync>> {
        let page_info = PageInfo::new(page_num, page_size);
        let offset = page_info.get_page_offset();
        let limit = page_info.get_page_size();

        let mut query_builder = format!("SELECT {} FROM sys_user {}", USER_FIELDS, where_clause);
        query_builder.push_str(" ORDER BY create_time DESC LIMIT ? OFFSET ?");

        let query = sqlx::query(&query_builder)
            .bind(limit as i64)
            .bind(offset as i64);

        let users_query = query.fetch_all(&self.pool).await?;

        let users: Result<Vec<User>, _> = users_query.iter().map(DbMapper::map_to_user).collect();

        // 构建 COUNT 查询
        let count_query = format!("SELECT COUNT(*) as count FROM sys_user {}", where_clause);
        let count_query_builder = sqlx::query(&count_query);

        let count_row = count_query_builder.fetch_one(&self.pool).await?;
        let total: u64 = count_row.try_get("count")?;
        let total_page = (total + limit - 1) / limit; // 向上取整计算总页数

        Ok((users?, total, total_page))
    }
}
