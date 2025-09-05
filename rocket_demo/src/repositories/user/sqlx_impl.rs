use crate::models::constants::USER_FIELDS;
use crate::models::user::User;
use crate::repositories::user::user_repository::UserRepository;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::Arc;

/// 用户仓库SQLx实现
#[derive(Debug)]
pub struct UserRepositorySqlxImpl {
    pool: Arc<MySqlPool>,
}

/// SQLx的用户实体映射
#[derive(Debug, FromRow)]
struct UserRow {
    id: String,
    dept_id: Option<String>,
    name: Option<String>,
    email: Option<String>,
    phone_number: Option<String>,
    sex: Option<String>,
    password: Option<String>,
    avatar: Option<String>,
    status: Option<i32>,
    login_ip: Option<String>,
    #[sqlx(rename = "login_time")]
    login_time_raw: Option<NaiveDateTime>,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<NaiveDateTime>,
    update_by: Option<String>,
    #[sqlx(rename = "update_time")]
    update_time_raw: Option<NaiveDateTime>,
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
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
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

#[rocket::async_trait]
impl UserRepository for UserRepositorySqlxImpl {
    /// 根据ID获取用户信息
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn StdError + Send + Sync>> {
        let user_query = sqlx::query_as::<_, UserRow>(&format!("SELECT {} FROM sys_user WHERE id = ?", USER_FIELDS))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match user_query {
            Some(row) => Ok(Some(User::from(row))),
            None => Ok(None),
        }
    }

    /// 根据用户名查找用户
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn StdError + Send + Sync>> {
        let user_query = sqlx::query_as::<_, UserRow>(&format!("SELECT {} FROM sys_user WHERE name = ?", USER_FIELDS))
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;

        match user_query {
            Some(row) => Ok(Some(User::from(row))),
            None => Ok(None),
        }
    }

    /// 查询用户列表
    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn StdError + Send + Sync>> {
        let mut sql = format!("SELECT {} FROM sys_user WHERE 1=1", USER_FIELDS);
        let mut params: Vec<Box<(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)>> = vec![];

        if !user.id.is_empty() {
            sql.push_str(" AND id = ?");
            params.push(Box::new(user.id.clone()));
        }

        if let Some(name) = &user.name {
            sql.push_str(" AND name LIKE ?");
            params.push(Box::new(format!("%{}%", name)));
        }

        if let Some(dept_id) = &user.dept_id {
            sql.push_str(" AND dept_id = ?");
            params.push(Box::new(dept_id.clone()));
        }

        sql.push_str(" ORDER BY id");

        let mut query = sqlx::query_as::<_, UserRow>(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(&self.pool).await?;
        Ok(result.into_iter().map(User::from).collect())
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
        let users: Result<Vec<User>, _> = users_query
            .iter()
            .map(|row| {
                let user_row = UserRow {
                    id: row.get("id"),
                    dept_id: row.get("dept_id"),
                    name: row.get("name"),
                    email: row.get("email"),
                    phone_number: row.get("phone_number"),
                    sex: row.get("sex"),
                    password: row.get("password"),
                    avatar: row.get("avatar"),
                    status: row.get("status"),
                    login_ip: row.get("login_ip"),
                    login_time_raw: row.get("login_time"),
                    create_by: row.get("create_by"),
                    create_time_raw: row.get("create_time"),
                    update_by: row.get("update_by"),
                    update_time_raw: row.get("update_time"),
                    remark: row.get("remark"),
                };
                Ok(User::from(user_row))
            })
            .collect();
        Ok(users?)
    }

    /// 插入用户记录
    async fn insert(&self, user: &User) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let login_time: Option<chrono::NaiveDateTime> = user.login_time.map(|t| t.naive_utc());
        let create_time: Option<chrono::NaiveDateTime> = user.create_time.map(|t| t.naive_utc());
        let update_time: Option<chrono::NaiveDateTime> = user.update_time.map(|t| t.naive_utc());

        let result = sqlx::query(&format!("INSERT INTO sys_user ({}) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", USER_FIELDS))
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
        let sql = "DELETE FROM sys_user WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;
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
