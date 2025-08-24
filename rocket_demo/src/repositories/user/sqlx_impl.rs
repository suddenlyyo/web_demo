use chrono::{NaiveDateTime, Utc};
use sqlx::Row;
use sqlx::mysql::{MySqlPool, MySqlRow};
use std::error::Error as StdError;

use crate::models::{User, UserRole};
use crate::services::params::user_param::UserParam;
use common_wrapper::PageInfo;

/// 用户表的所有字段，用于SQL查询
const USER_FIELDS: &str = "id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark";

/// 数据库映射器
struct DbMapper;

impl DbMapper {
    /// 将数据库行映射为用户对象
    fn map_to_user(row: &MySqlRow) -> Result<User, sqlx::Error> {
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
                .try_get::<Option<NaiveDateTime>, _>("login_time")?
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            create_by: row.try_get("create_by")?,
            create_time: row
                .try_get::<Option<NaiveDateTime>, _>("create_time")?
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            update_by: row.try_get("update_by")?,
            update_time: row
                .try_get::<Option<NaiveDateTime>, _>("update_time")?
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            remark: row.try_get("remark")?,
        })
    }
}

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

    /// 构建查询条件
    fn build_where_clause(query: &UserParam) -> (String, Vec<String>) {
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
impl UserRepository for UserRepositorySqlxImpl {
    /// 根据ID获取用户信息
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn StdError + Send + Sync>> {
        let user_query = sqlx::query(&format!("SELECT {} FROM sys_user WHERE id = ?", USER_FIELDS))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match user_query {
            Some(row) => {
                let user = DbMapper::map_to_user(&row)?;
                Ok(Some(user))
            },
            None => Ok(None),
        }
    }

    /// 根据用户名查找用户
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn StdError + Send + Sync>> {
        let user_query = sqlx::query(&format!("SELECT {} FROM sys_user WHERE name = ?", USER_FIELDS))
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        match user_query {
            Some(row) => {
                let user = DbMapper::map_to_user(&row)?;
                Ok(Some(user))
            },
            None => Ok(None),
        }
    }

    /// 查询用户列表
    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn StdError + Send + Sync>> {
        let user_query: UserParam = user.into();
        let (where_clause, params) = Self::build_where_clause(&user_query);

        let sql = format!("SELECT {} FROM sys_user {}", USER_FIELDS, where_clause);
        let mut query = sqlx::query(&sql);

        for param in params {
            query = query.bind(param);
        }

        let users_query = query.fetch_all(&self.pool).await?;
        let users: Result<Vec<User>, _> = users_query.iter().map(DbMapper::map_to_user).collect();
        Ok(users?)
    }

    /// 获取用户列表数量
    async fn get_user_list_count(&self, query: &UserParam) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let (where_clause, params) = Self::build_where_clause(query);

        let sql = format!("SELECT COUNT(*) as count FROM sys_user {}", where_clause);
        let mut query_builder = sqlx::query(&sql);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        let count_row = query_builder.fetch_one(&self.pool).await?;
        let total: u64 = count_row.try_get("count")?;
        Ok(total)
    }

    /// 分页获取用户列表
    async fn get_user_list_by_page(&self, query: &UserParam) -> Result<Vec<User>, Box<dyn StdError + Send + Sync>> {
        let page_info = PageInfo::new(query.current_page_num, query.page_size);
        let offset = page_info.get_page_offset();
        let limit = page_info.get_page_size();

        let (where_clause, params) = Self::build_where_clause(query);

        let user_query = format!("SELECT {} FROM sys_user {} ORDER BY create_time DESC LIMIT ? OFFSET ?", USER_FIELDS, where_clause);
        let mut query_builder = sqlx::query(&user_query);

        for param in params {
            query_builder = query_builder.bind(param);
        }

        query_builder = query_builder.bind(limit as i64).bind(offset as i64);
        let users_query = query_builder.fetch_all(&self.pool).await?;
        let users: Result<Vec<User>, _> = users_query.iter().map(DbMapper::map_to_user).collect();
        Ok(users?)
    }

    /// 插入用户记录
    async fn insert(&self, user: &User) -> Result<(), Box<dyn StdError + Send + Sync>> {
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

        if result.rows_affected() > 0 { Ok(()) } else { Err("Failed to add user".into()) }
    }

    /// 选择性插入用户记录
    async fn insert_selective(&self, user: &User) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 与insert方法实现相同，在实际应用中可以根据需要进行区分
        self.insert(user).await
    }

    /// 根据ID更新用户信息
    async fn update_by_primary_key(&self, user: &User) -> Result<(), Box<dyn StdError + Send + Sync>> {
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

        if result.rows_affected() > 0 { Ok(()) } else { Err("Failed to update user".into()) }
    }

    /// 根据ID选择性更新用户信息
    async fn update_by_primary_key_selective(&self, user: &User) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 与update_by_primary_key方法实现相同，在实际应用中可以根据需要进行区分
        self.update_by_primary_key(user).await
    }

    /// 根据ID删除用户
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let result = sqlx::query("DELETE FROM sys_user WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 { Ok(()) } else { Err("Failed to delete user".into()) }
    }

    // ========== 用户角色相关方法 ==========

    /// 根据角色ID查询用户角色列表
    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn StdError + Send + Sync>> {
        let sql = "SELECT user_id, role_id FROM sys_user_role WHERE role_id = ?";
        let rows = sqlx::query(sql).bind(role_id).fetch_all(&self.pool).await?;

        let user_roles: Result<Vec<UserRole>, _> = rows
            .iter()
            .map(|row| {
                Ok(UserRole {
                    user_id: row.try_get("user_id")?,
                    role_id: row.try_get("role_id")?,
                })
            })
            .collect();

        Ok(user_roles?)
    }

    /// 根据用户ID查询用户角色列表
    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn StdError + Send + Sync>> {
        let sql = "SELECT user_id, role_id FROM sys_user_role WHERE user_id = ?";
        let rows = sqlx::query(sql).bind(user_id).fetch_all(&self.pool).await?;

        let user_roles: Result<Vec<UserRole>, _> = rows
            .iter()
            .map(|row| {
                Ok(UserRole {
                    user_id: row.try_get("user_id")?,
                    role_id: row.try_get("role_id")?,
                })
            })
            .collect();

        Ok(user_roles?)
    }

    /// 批量插入用户角色
    async fn batch_insert_user_role(&self, list: &[UserRole]) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        // 构建VALUES部分
        let values_placeholders: Vec<String> = (0..list.len()).map(|_| "(?, ?)".to_string()).collect();
        let sql = format!("INSERT INTO sys_user_role (user_id, role_id) VALUES {}", values_placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for user_role in list {
            query = query.bind(&user_role.user_id).bind(&user_role.role_id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    /// 根据用户ID和角色ID列表批量删除用户角色
    async fn batch_delete_user_role_by_user_and_role_ids(&self, user_id: &str, list: &[String]) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = (0..list.len()).map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM sys_user_role WHERE user_id = ? AND role_id IN ({})", placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        query = query.bind(user_id);
        for role_id in list {
            query = query.bind(role_id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    /// 根据用户ID删除用户角色
    async fn delete_user_role_by_user_id(&self, user_id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_user_role WHERE user_id = ?";
        sqlx::query(sql).bind(user_id).execute(&self.pool).await?;

        Ok(())
    }
}
