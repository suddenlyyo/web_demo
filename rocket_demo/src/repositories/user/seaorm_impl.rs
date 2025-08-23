//! 用户数据访问层 SeaORM 实现

use sea_orm::sea_query::{Condition, Order};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::models::{User, UserQuery};
use crate::repositories::user::UserRepository;
use common_wrapper::PageInfo;

// 导入SeaORM实体
use crate::entities::sys_user;
use crate::entities::sys_user::{Column, Entity};

/// 用户表的所有字段，用于SQL查询
const USER_FIELDS: &str = "id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark";

/// 用户数据访问 SeaORM 实现
#[derive(Debug)]
pub struct UserRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

impl UserRepositorySeaormImpl {
    /// 创建用户仓库 SeaORM 实例
    pub async fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite://data.db".to_string());
        let connection = sea_orm::Database::connect(&database_url)
            .await
            .expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositorySeaormImpl {
    /// 根据ID获取用户信息
    async fn get_user_by_id(&self, id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询用户信息
        let user = Entity::find_by_id(id).one(&self.connection).await?;

        match user {
            Some(user) => Ok(user.into()),
            None => Err("User not found".into()),
        }
    }

    /// 获取用户列表
    async fn list_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询用户列表
        let users = Entity::find()
            .limit(100)
            .all(&self.connection)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok(users)
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

        // 构建查询条件
        let mut condition = Condition::all();

        if let Some(id) = &query.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(name) = &query.name {
            condition = condition.add(Column::Name.contains(name));
        }

        if let Some(dept_id) = &query.dept_id {
            condition = condition.add(Column::DeptId.eq(dept_id));
        }

        if let Some(email) = &query.email {
            condition = condition.add(Column::Email.contains(email));
        }

        if let Some(phone_number) = &query.phone_number {
            condition = condition.add(Column::PhoneNumber.contains(phone_number));
        }

        if let Some(sex) = &query.sex {
            condition = condition.add(Column::Sex.eq(sex));
        }

        if let Some(status) = query.status {
            condition = condition.add(Column::Status.eq(status));
        }

        if let Some(remark) = &query.remark {
            condition = condition.add(Column::Remark.contains(remark));
        }

        if let Some(start_date) = query.start_date {
            condition = condition.add(Column::CreateTime.gte(start_date));
        }

        if let Some(end_date) = query.end_date {
            condition = condition.add(Column::CreateTime.lte(end_date));
        }

        // 查询总记录数
        let total_count = Entity::find()
            .filter(condition.clone())
            .count(&self.connection)
            .await? as u64;

        // 计算总页数
        let total_pages = (total_count + page_size - 1) / page_size;

        // 查询当前页数据
        let users = Entity::find()
            .filter(condition)
            .order_by(Column::CreateTime, Order::Desc)
            .paginate(&self.connection, page_size)
            .fetch_page(current_page - 1) // SeaORM的分页从0开始
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok((users, total_count, total_pages))
    }

    /// 根据用户名查找用户
    async fn get_user_by_name(&self, name: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询用户信息
        let user = Entity::find()
            .filter(Column::Name.eq(name))
            .one(&self.connection)
            .await?;

        match user {
            Some(user) => Ok(user.into()),
            None => Err("User not found".into()),
        }
    }

    /// 根据部门ID查找用户列表
    async fn list_users_by_dept_id(&self, dept_id: &str) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询用户列表
        let users = Entity::find()
            .filter(Column::DeptId.eq(dept_id))
            .all(&self.connection)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok(users)
    }

    /// 新增用户
    async fn add_user(&self, user: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM新增用户
        let user_model: sys_user::ActiveModel = user.into();
        let inserted_user = sys_user::Entity::insert(user_model)
            .exec_with_returning(&self.connection)
            .await?;

        Ok(inserted_user.into())
    }

    /// 修改用户
    async fn update_user(&self, user: User) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM修改用户
        let user_model: sys_user::ActiveModel = user.into();
        let updated_user = sys_user::Entity::update(user_model)
            .filter(Column::Id.eq(user_model.id.clone().unwrap()))
            .exec(&self.connection)
            .await?;

        Ok(updated_user.into())
    }

    /// 删除用户
    async fn delete_user(&self, id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM删除用户
        let deleted_user = sys_user::Entity::delete_by_id(id)
            .exec(&self.connection)
            .await?;

        // 模拟返回被删除的用户信息
        let user = User { id: id.to_string(), ..Default::default() };

        Ok(user)
    }

    /// 修改用户状态
    async fn update_user_status(&self, id: &str, status: i32) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM修改用户状态
        let user: sys_user::ActiveModel = sys_user::Entity::find_by_id(id)
            .one(&self.connection)
            .await?
            .ok_or("User not found")?
            .into();

        let mut user: sys_user::ActiveModel = user;
        user.status = sea_orm::Set(status);
        user.update_time = sea_orm::Set(chrono::Utc::now());

        let updated_user = sys_user::Entity::update(user)
            .filter(Column::Id.eq(id))
            .exec(&self.connection)
            .await?;

        Ok(updated_user.into())
    }
}
