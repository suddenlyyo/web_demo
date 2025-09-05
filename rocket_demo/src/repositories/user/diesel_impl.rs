//! 用户模块 Diesel 实现

// ==================== 导入模块 ====================
use diesel::prelude::*;

use crate::models::User;
use crate::models::constants::USER_FIELDS;
use crate::repositories::user::user_repository::UserRepository;

table! {
    sys_user (id) {
        id -> Text,
        dept_id -> Nullable<Text>,
        name -> Nullable<Text>,
        email -> Nullable<Text>,
        phone_number -> Nullable<Text>,
        sex -> Nullable<Text>,
        password -> Nullable<Text>,
        avatar -> Nullable<Text>,
        status -> Nullable<Integer>,
        login_ip -> Nullable<Text>,
        login_time -> Nullable<Timestamp>,
        create_by -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_by -> Nullable<Text>,
        update_time -> Nullable<Timestamp>,
        remark -> Nullable<Text>,
    }
}

// ==================== 表结构体映射 ====================
#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// 用户表结构体
struct UserRow {
    /// 用户ID
    id: String,
    /// 部门ID
    dept_id: Option<String>,
    /// 用户名
    name: Option<String>,
    /// 电子邮箱
    email: Option<String>,
    /// 手机号码
    phone_number: Option<String>,
    /// 性别
    sex: Option<String>,
    /// 密码
    password: Option<String>,
    /// 头像地址
    avatar: Option<String>,
    /// 用户状态 (0-禁用 1-启用)
    status: Option<i32>,
    /// 最后登录IP
    login_ip: Option<String>,
    /// 最后登录时间
    login_time: Option<chrono::NaiveDateTime>,
    /// 创建者
    create_by: Option<String>,
    /// 创建时间
    create_time: Option<chrono::NaiveDateTime>,
    /// 更新者
    update_by: Option<String>,
    /// 更新时间
    update_time: Option<chrono::NaiveDateTime>,
    /// 备注
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
                .login_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            create_by: row.create_by,
            create_time: row
                .create_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: row.remark,
        }
    }
}

impl From<&User> for UserRow {
    fn from(user: &User) -> Self {
        UserRow {
            id: user.id.clone(),
            dept_id: user.dept_id.clone(),
            name: user.name.clone(),
            email: user.email.clone(),
            phone_number: user.phone_number.clone(),
            sex: user.sex.clone(),
            password: user.password.clone(),
            avatar: user.avatar.clone(),
            status: user.status,
            login_ip: user.login_ip.clone(),
            login_time: user.login_time.map(|t| t.naive_utc()),
            create_by: user.create_by.clone(),
            create_time: user.create_time.map(|t| t.naive_utc()),
            update_by: user.update_by.clone(),
            update_time: user.update_time.map(|t| t.naive_utc()),
            remark: user.remark.clone(),
        }
    }
}

/// 用户数据访问 Diesel 实现
#[derive(Debug)]
pub struct UserRepositoryDieselImpl {
    /// 数据库连接
    connection: diesel::pg::PgConnection,
}

impl UserRepositoryDieselImpl {
    // ==================== 数据库连接 ====================
    /// 创建用户仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() {
            config.database.url
        } else {
            panic!("无法从配置文件获取数据库连接信息");
        };

        let connection = diesel::pg::PgConnection::establish(&database_url).expect("Error connecting to PostgreSQL database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositoryDieselImpl {
    // ==================== SQL trait 实现 ====================
    /// 根据主键删除用户
    async fn delete_by_primary_key(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        diesel::delete(sys_user.filter(id.eq(user_id))).execute(&mut self.connection)?;
        Ok(())
    }

    /// 插入用户记录
    async fn insert(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let user_row: UserRow = row.into();
        diesel::insert_into(sys_user)
            .values(&user_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 选择性插入用户记录
    async fn insert_selective(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let user_row: UserRow = row.into();
        diesel::insert_into(sys_user)
            .values(&user_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据主键查询用户
    async fn select_user_by_id(&self, user_id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let result = sys_user
            .filter(id.eq(user_id))
            .first::<UserRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(User::from))
    }

    /// 查询用户列表
    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let mut query = sys_user.into_boxed();

        /// 按用户名模糊匹配
        if let Some(name_filter) = &user.name {
            query = query.filter(name.like(format!("%{}%", name_filter)));
        }

        /// 按手机号码模糊匹配
        if let Some(phone_number_filter) = &user.phone_number {
            query = query.filter(phone_number.like(format!("%{}%", phone_number_filter)));
        }

        /// 按状态过滤
        if let Some(status_filter) = user.status {
            query = query.filter(status.eq(status_filter));
        }

        let result = query
            .order(id.asc())
            .load::<UserRow>(&mut self.connection)?;
        Ok(result.into_iter().map(User::from).collect())
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let result = sys_user
            .filter(name.eq(name))
            .first::<UserRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(User::from))
    }

    /// 根据主键更新用户
    async fn update_by_id(&self, row: &User) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let user_row: UserRow = row.into();
        let result = diesel::update(sys_user.filter(id.eq(&row.id)))
            .set(&user_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键选择性更新用户
    async fn update_by_id_selective(&self, row: &User) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let user_row: UserRow = row.into();
        let result = diesel::update(sys_user.filter(id.eq(&row.id)))
            .set(&user_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键删除用户
    async fn delete_by_id(&self, user_id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let result = diesel::delete(sys_user.filter(id.eq(user_id))).execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键查询用户
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let result = sys_user
            .filter(id.eq(id))
            .first::<UserRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(User::from))
    }

    /// 根据字段条件查询用户列表（MyBatis风格）
    async fn select_user_list_by_fields(&self, username: Option<String>, phone: Option<String>, status: Option<i32>) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let mut query = sys_user.into_boxed();

        /// 按用户名模糊匹配
        if let Some(name_filter) = username {
            query = query.filter(name.like(format!("%{}%", name_filter)));
        }

        /// 按手机号码模糊匹配
        if let Some(phone_filter) = phone {
            query = query.filter(phone_number.like(format!("%{}%", phone_filter)));
        }

        /// 按状态过滤
        if let Some(status_filter) = status {
            query = query.filter(status.eq(status_filter));
        }

        let result = query
            .order(id.asc())
            .load::<UserRow>(&mut self.connection)?;
        Ok(result.into_iter().map(User::from).collect())
    }

    /// 根据角色ID查询用户角色列表
    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现根据角色ID查询用户角色列表逻辑
        let _ = role_id;
        Ok(vec![])
    }

    /// 根据用户ID查询用户角色列表
    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现根据用户ID查询用户角色列表逻辑
        let _ = user_id;
        Ok(vec![])
    }

    /// 分页查询用户列表
    async fn get_user_list_by_page(
        &self, name: Option<String>, dept_id: Option<String>, email: Option<String>, phone_number: Option<String>, status: Option<i32>, start_date: Option<chrono::DateTime<chrono::Utc>>, end_date: Option<chrono::DateTime<chrono::Utc>>, page_num: u64, page_size: u64,
    ) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现分页查询逻辑
        let _ = (name, dept_id, email, phone_number, status, start_date, end_date, page_num, page_size);
        Ok(vec![])
    }

    /// 查询用户列表总数
    async fn get_user_list_count(
        &self, name: Option<String>, dept_id: Option<String>, email: Option<String>, phone_number: Option<String>, status: Option<i32>, start_date: Option<chrono::DateTime<chrono::Utc>>, end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现查询总数逻辑
        let _ = (name, dept_id, email, phone_number, status, start_date, end_date);
        Ok(0)
    }
}
