//! User data access layer Diesel implementation

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Integer, Text};

use crate::models::{User, UserQuery};
use crate::repositories::user::user_repository::UserRepository;
use common_wrapper::PageInfo;

/// User table fields, used for SQL queries
const USER_FIELDS: &str = "id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark";

/// Struct for getting COUNT query result
#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: u64,
}

/// User data access Diesel implementation
#[derive(Debug)]
pub struct UserRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl UserRepositoryDieselImpl {
    /// Create user repository Diesel instance
    pub fn new() -> Self {
        // Initialize database connection
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }

    /// Build query conditions
    fn build_where_clause(query: &UserQuery) -> (String, Vec<String>) {
        let mut where_conditions = Vec::new();
        let mut params = Vec::new();

        // Add ID query condition
        if let Some(id) = &query.id {
            where_conditions.push("id = ?");
            params.push(id.clone());
        }

        // Add name query condition
        if let Some(name) = &query.name {
            where_conditions.push("name LIKE ?");
            params.push(format!("%{}%", name));
        }

        // Add department ID query condition
        if let Some(dept_id) = &query.dept_id {
            where_conditions.push("dept_id = ?");
            params.push(dept_id.clone());
        }

        // Add email query condition
        if let Some(email) = &query.email {
            where_conditions.push("email LIKE ?");
            params.push(format!("%{}%", email));
        }

        // Add phone number query condition
        if let Some(phone_number) = &query.phone_number {
            where_conditions.push("phone_number LIKE ?");
            params.push(format!("%{}%", phone_number));
        }

        // Add sex query condition
        if let Some(sex) = &query.sex {
            where_conditions.push("sex = ?");
            params.push(sex.clone());
        }

        // Add status query condition
        if let Some(status) = query.status {
            where_conditions.push("status = ?");
            params.push(status.to_string());
        }

        // Add remark query condition
        if let Some(remark) = &query.remark {
            where_conditions.push("remark LIKE ?");
            params.push(format!("%{}%", remark));
        }

        // Add date range query condition
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
    /// Get user information by ID
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        // Use Diesel to query user information
        let result = sql_query(format!("SELECT {} FROM sys_user WHERE id = ?", USER_FIELDS))
            .bind::<Text, _>(id)
            .get_result::<User>(&mut self.connection);

        match result {
            Ok(user) => Ok(Some(user)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(Box::new(e)),
        }
    }

    /// Find user by name
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

    /// Query user list
    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let user_query: UserQuery = user.into();
        let (where_clause, _params) = Self::build_where_clause(&user_query);

        let sql = format!("SELECT {} FROM sys_user {}", USER_FIELDS, where_clause);
        let users_query = sql_query(&sql).load::<User>(&mut self.connection)?;
        Ok(users_query)
    }

    /// Get user list count
    async fn get_user_list_count(&self, query: &UserQuery) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let (where_clause, _params) = Self::build_where_clause(query);

        let sql = format!("SELECT COUNT(*) as count FROM sys_user {}", where_clause);
        let count_result = sql_query(&sql).get_result::<CountResult>(&mut self.connection)?;
        Ok(count_result.count)
    }

    /// Paginate user list
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

    /// Insert user record
    async fn insert(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Build insert statement
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

    /// Selective insert user record
    async fn insert_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Same implementation as insert method, can be differentiated based on needs in actual application
        self.insert(user).await
    }

    /// Update user information by ID
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

    /// Selective update user information by ID
    async fn update_by_primary_key_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Same implementation as update_by_primary_key method, can be differentiated based on needs in actual application
        self.update_by_primary_key(user).await
    }

    /// Delete user by ID
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
