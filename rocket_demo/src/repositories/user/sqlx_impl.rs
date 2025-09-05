use crate::models::constants::USER_FIELDS;
use crate::models::user::User;
use crate::models::user_role::UserRole;
use crate::params::user_param::UserParam;
use crate::repositories::user::user_repository::UserRepository;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::sync::OnceLock;

// 数据库连接池
static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

#[derive(Debug)]
pub struct UserRepositorySqlxImpl {
    pool: MySqlPool,
}

impl UserRepositorySqlxImpl {
    pub fn new() -> Self {
        let pool = DB_POOL.get().expect("数据库连接池未初始化").clone();
        Self { pool }
    }

    pub fn init_pool(pool: MySqlPool) {
        DB_POOL.set(pool).ok(); // 如果已经设置过，则忽略
    }
}

// ==================== 表结构体映射 ====================
#[derive(Debug, FromRow)]
struct UserRow {
    id: String,
    name: Option<String>,
    password: Option<String>,
    dept_id: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    sex: Option<String>,
    avatar: Option<String>,
    status: Option<i32>,
    login_ip: Option<String>,
    #[sqlx(rename = "login_time")]
    login_time_raw: Option<chrono::NaiveDateTime>,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<chrono::NaiveDateTime>,
    update_by: Option<String>,
    #[sqlx(rename = "update_time")]
    update_time_raw: Option<chrono::NaiveDateTime>,
    remark: Option<String>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            dept_id: row.dept_id,
            name: row.name,
            email: row.email,
            phone_number: row.phone_number,
            sex: row.sex,
            password: row.password,
            avatar: row.avatar,
            status: row.status,
            login_ip: row.login_ip,
            login_time: row
                .login_time_raw
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            create_by: row.create_by,
            create_time: row
                .create_time_raw
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time_raw
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: row.remark,
        }
    }
}

// 用户角色映射
#[derive(Debug, FromRow)]
struct UserRoleRow {
    id: String,
    user_id: String,
    role_id: String,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<NaiveDateTime>,
}

impl From<UserRoleRow> for UserRole {
    fn from(row: UserRoleRow) -> Self {
        UserRole { user_id: row.user_id, role_id: row.role_id }
    }
}

// ==================== SQL trait 实现 ====================
#[rocket::async_trait]
impl UserRepository for UserRepositorySqlxImpl {
    async fn insert(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "INSERT INTO sys_user (id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.dept_id)
            .bind(&row.name)
            .bind(&row.email)
            .bind(&row.phone_number)
            .bind(&row.sex)
            .bind(&row.password)
            .bind(&row.avatar)
            .bind(row.status)
            .bind(&row.login_ip)
            .bind(row.login_time.map(|t| t.naive_utc()))
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("用户插入失败"));
        }

        Ok(())
    }

    async fn insert_selective(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

        fields.push("id");
        placeholders.push("?");
        params.push(&row.id);

        if row.username.is_some() {
            fields.push("username");
            placeholders.push("?");
            params.push(&row.username);
        }

        if row.password.is_some() {
            fields.push("password");
            placeholders.push("?");
            params.push(&row.password);
        }

        if row.salt.is_some() {
            fields.push("salt");
            placeholders.push("?");
            params.push(&row.salt);
        }

        if row.nickname.is_some() {
            fields.push("nickname");
            placeholders.push("?");
            params.push(&row.nickname);
        }

        if row.phone.is_some() {
            fields.push("phone");
            placeholders.push("?");
            params.push(&row.phone);
        }

        if row.email.is_some() {
            fields.push("email");
            placeholders.push("?");
            params.push(&row.email);
        }

        if row.avatar.is_some() {
            fields.push("avatar");
            placeholders.push("?");
            params.push(&row.avatar);
        }

        if row.sex.is_some() {
            fields.push("sex");
            placeholders.push("?");
            params.push(&row.sex);
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

        let sql = format!("INSERT INTO sys_user ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("用户插入失败"));
        }

        Ok(())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, username, password, salt, nickname, phone, email, avatar, sex, status, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE username = ?";
        let result: Option<UserRow> = sqlx::query_as(sql)
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(User::from))
    }

    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, username, password, salt, nickname, phone, email, avatar, sex, status, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE id = ?";
        let result: Option<UserRow> = sqlx::query_as(sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(User::from))
    }

    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let mut sql = "SELECT id, username, password, salt, nickname, phone, email, avatar, sex, status, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE 1=1".to_string();
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if let Some(username) = &user.username {
            sql.push_str(" AND username LIKE ?");
            params.push(Box::new(format!("%{}%", username)));
        }

        if let Some(phone) = &user.phone {
            sql.push_str(" AND phone LIKE ?");
            params.push(Box::new(format!("%{}%", phone)));
        }

        if let Some(status) = user.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        sql.push_str(" ORDER BY id");

        // 构建查询
        let mut query = sqlx::query_as::<_, UserRow>(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(&self.pool).await?;
        Ok(result.into_iter().map(User::from).collect())
    }

    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, user_id, role_id, create_by, create_time FROM sys_user_role WHERE role_id = ?";
        let result: Vec<UserRoleRow> = sqlx::query_as(sql)
            .bind(role_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(UserRole::from).collect())
    }

    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, user_id, role_id, create_by, create_time FROM sys_user_role WHERE user_id = ?";
        let result: Vec<UserRoleRow> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(UserRole::from).collect())
    }

    async fn get_user_list_by_page(&self, query: &UserParam) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let mut sql = "SELECT id, username, password, salt, nickname, phone, email, avatar, sex, status, create_by, create_time, update_by, update_time, remark FROM sys_user WHERE 1=1".to_string();
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if let Some(username) = &query.username {
            sql.push_str(" AND username LIKE ?");
            params.push(Box::new(format!("%{}%", username)));
        }

        if let Some(phone) = &query.phone {
            sql.push_str(" AND phone LIKE ?");
            params.push(Box::new(format!("%{}%", phone)));
        }

        if let Some(status) = query.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        // 添加排序和分页
        sql.push_str(" ORDER BY id LIMIT ? OFFSET ?");
        params.push(Box::new(query.page_size));
        params.push(Box::new((query.page_num - 1) * query.page_size));

        // 构建查询列表的语句
        let mut query_builder = sqlx::query_as::<_, UserRow>(&sql);
        for param in &params {
            query_builder = query_builder.bind(param.as_ref());
        }

        let result = query_builder.fetch_all(&self.pool).await?;
        Ok(result.into_iter().map(User::from).collect())
    }

    async fn get_user_list_count(&self, query: &UserParam) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut sql = "SELECT COUNT(*) FROM sys_user WHERE 1=1".to_string();
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if let Some(username) = &query.username {
            sql.push_str(" AND username LIKE ?");
            params.push(Box::new(format!("%{}%", username)));
        }

        if let Some(phone) = &query.phone {
            sql.push_str(" AND phone LIKE ?");
            params.push(Box::new(format!("%{}%", phone)));
        }

        if let Some(status) = query.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        // 构建查询总数的语句
        let mut count_query = sqlx::query_scalar(&sql);
        for param in &params {
            count_query = count_query.bind(param.as_ref());
        }

        let total: u64 = count_query.fetch_one(&self.pool).await?;
        Ok(total)
    }

    async fn update_by_primary_key(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "UPDATE sys_user SET username = ?, password = ?, salt = ?, nickname = ?, phone = ?, email = ?, avatar = ?, sex = ?, status = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";

        let result = sqlx::query(sql)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.salt)
            .bind(&user.nickname)
            .bind(&user.phone)
            .bind(&user.email)
            .bind(&user.avatar)
            .bind(&user.sex)
            .bind(user.status)
            .bind(&user.create_by)
            .bind(user.create_time.map(|t| t.naive_utc()))
            .bind(&user.update_by)
            .bind(user.update_time.map(|t| t.naive_utc()))
            .bind(&user.remark)
            .bind(&user.id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("用户更新失败"));
        }

        Ok(())
    }

    async fn update_by_primary_key_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

        if user.username.is_some() {
            updates.push("username = ?");
            params.push(&user.username);
        }

        if user.password.is_some() {
            updates.push("password = ?");
            params.push(&user.password);
        }

        if user.salt.is_some() {
            updates.push("salt = ?");
            params.push(&user.salt);
        }

        if user.nickname.is_some() {
            updates.push("nickname = ?");
            params.push(&user.nickname);
        }

        if user.phone.is_some() {
            updates.push("phone = ?");
            params.push(&user.phone);
        }

        if user.email.is_some() {
            updates.push("email = ?");
            params.push(&user.email);
        }

        if user.avatar.is_some() {
            updates.push("avatar = ?");
            params.push(&user.avatar);
        }

        if user.sex.is_some() {
            updates.push("sex = ?");
            params.push(&user.sex);
        }

        if user.status.is_some() {
            updates.push("status = ?");
            params.push(&user.status);
        }

        if user.create_by.is_some() {
            updates.push("create_by = ?");
            params.push(&user.create_by);
        }

        if user.create_time.is_some() {
            updates.push("create_time = ?");
            params.push(&user.create_time.map(|t| t.naive_utc()));
        }

        if user.update_by.is_some() {
            updates.push("update_by = ?");
            params.push(&user.update_by);
        }

        if user.update_time.is_some() {
            updates.push("update_time = ?");
            params.push(&user.update_time.map(|t| t.naive_utc()));
        }

        if user.remark.is_some() {
            updates.push("remark = ?");
            params.push(&user.remark);
        }

        if updates.is_empty() {
            return Ok(());
        }

        let sql = format!("UPDATE sys_user SET {} WHERE id = ?", updates.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }
        query = query.bind(&user.id);

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("用户更新失败"));
        }

        Ok(())
    }

    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "DELETE FROM sys_user WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("用户删除失败"));
        }

        Ok(())
    }

    async fn batch_insert_user_role(&self, list: &[UserRole]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let mut fields = vec![];
        let mut placeholders = vec![];

        fields.push("id");
        fields.push("user_id");
        fields.push("role_id");
        fields.push("create_by");
        fields.push("create_time");

        let mut query_params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        for user_role in list {
            placeholders.push("(?, ?, ?, ?, ?)".to_string());
            query_params.push(Box::new(&user_role.id));
            query_params.push(Box::new(&user_role.user_id));
            query_params.push(Box::new(&user_role.role_id));
            query_params.push(Box::new(&user_role.create_by));
            query_params.push(Box::new(user_role.create_time.map(|t| t.naive_utc())));
        }

        let sql = format!("INSERT INTO sys_user_role ({}) VALUES {}", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in &query_params {
            query = query.bind(param.as_ref());
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("用户角色插入失败"));
        }

        Ok(())
    }

    async fn batch_delete_user_role_by_user_and_role_ids(&self, user_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = list.iter().map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM sys_user_role WHERE user_id = ? AND role_id IN ({})", placeholders.join(","));

        let mut query = sqlx::query(&sql);
        query = query.bind(user_id);
        for role_id in list {
            query = query.bind(role_id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_user_role_by_user_id(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "DELETE FROM sys_user_role WHERE user_id = ?";
        sqlx::query(sql).bind(user_id).execute(&self.pool).await?;
        Ok(())
    }
}
