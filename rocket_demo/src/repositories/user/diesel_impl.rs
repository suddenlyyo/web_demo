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
    async fn select_user_list(&self, user_param: crate::services::params::user_param::UserParam) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let mut query = sys_user.into_boxed();

        /// 按主键过滤
        if let Some(id_filter) = &user_param.id {
            query = query.filter(id.eq(id_filter));
        }

        /// 按用户名模糊匹配
        if let Some(name_filter) = &user_param.name {
            query = query.filter(name.like(format!("%{}%", name_filter)));
        }

        /// 按部门ID过滤
        if let Some(dept_id_filter) = &user_param.dept_id {
            query = query.filter(dept_id.eq(dept_id_filter));
        }

        let result = query
            .order(id.asc())
            .load::<UserRow>(&mut self.connection)?;
        Ok(result.into_iter().map(User::from).collect())
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

    /// 根据用户名查询用户
    async fn select_user_by_name(&self, username: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::user::diesel_impl::sys_user::dsl::*;

        let result = sys_user
            .filter(name.eq(username))
            .first::<UserRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(User::from))
    }
}
