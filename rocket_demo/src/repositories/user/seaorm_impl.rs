//! 用户数据访问层 SeaORM 实现

use sea_orm::sea_query::{Condition, Order};
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::models::{User, UserQuery};
use crate::repositories::user::user_repository::UserRepository;
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

    /// 构建查询条件
    fn build_condition(query: &UserQuery) -> Condition {
        let mut condition = Condition::all();

        // 添加ID查询条件
        if let Some(id) = &query.id {
            condition = condition.add(Column::Id.eq(id));
        }

        // 添加名称查询条件
        if let Some(name) = &query.name {
            condition = condition.add(Column::Name.contains(name));
        }

        // 添加部门ID查询条件
        if let Some(dept_id) = &query.dept_id {
            condition = condition.add(Column::DeptId.eq(dept_id));
        }

        // 添加邮箱查询条件
        if let Some(email) = &query.email {
            condition = condition.add(Column::Email.contains(email));
        }

        // 添加手机号码查询条件
        if let Some(phone_number) = &query.phone_number {
            condition = condition.add(Column::PhoneNumber.contains(phone_number));
        }

        // 添加性别查询条件
        if let Some(sex) = &query.sex {
            condition = condition.add(Column::Sex.eq(sex));
        }

        // 添加状态查询条件
        if let Some(status) = query.status {
            condition = condition.add(Column::Status.eq(status));
        }

        // 添加备注查询条件
        if let Some(remark) = &query.remark {
            condition = condition.add(Column::Remark.contains(remark));
        }

        // 添加日期范围查询条件
        if let (Some(start_date), Some(end_date)) = (&query.start_date, &query.end_date) {
            condition = condition.add(Column::CreateTime.between(start_date.naive_utc(), end_date.naive_utc()));
        } else if let Some(start_date) = &query.start_date {
            condition = condition.add(Column::CreateTime.gte(start_date.naive_utc()));
        } else if let Some(end_date) = &query.end_date {
            condition = condition.add(Column::CreateTime.lte(end_date.naive_utc()));
        }

        condition
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositorySeaormImpl {
    /// 根据ID获取用户信息
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询用户信息
        let user = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(user.map(|u| u.into()))
    }

    /// 根据用户名查找用户
    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let user = Entity::find()
            .filter(Column::Name.eq(name))
            .one(&self.connection)
            .await?;
        Ok(user.map(|u| u.into()))
    }

    /// 查询用户列表
    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let user_query: UserQuery = user.into();
        let condition = Self::build_condition(&user_query);

        let users = Entity::find()
            .filter(condition)
            .all(&self.connection)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok(users)
    }

    /// 获取用户列表数量
    async fn get_user_list_count(&self, query: &UserQuery) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let condition = Self::build_condition(query);

        let count = Entity::find()
            .filter(condition)
            .count(&self.connection)
            .await?;

        Ok(count)
    }

    /// 分页获取用户列表
    async fn get_user_list_by_page(&self, query: &UserQuery) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let page_info = PageInfo::new(query.current_page_num, query.page_size);
        let page_num = page_info.get_current_page_num();
        let page_size = page_info.get_page_size();

        let condition = Self::build_condition(query);

        let paginator = Entity::find()
            .filter(condition)
            .order_by(Column::CreateTime, Order::Desc)
            .paginate(&self.connection, page_size);

        let users = paginator
            .fetch_page(page_num - 1)
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect();

        Ok(users)
    }

    /// 插入用户记录
    async fn insert(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use sea_orm::ActiveModelTrait;

        let active_model: sys_user::ActiveModel = user.into();
        active_model.insert(&self.connection).await?;
        Ok(())
    }

    /// 选择性插入用户记录
    async fn insert_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 与insert方法实现相同，在实际应用中可以根据需要进行区分
        self.insert(user).await
    }

    /// 根据ID更新用户信息
    async fn update_by_primary_key(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use sea_orm::ActiveModelTrait;

        let active_model: sys_user::ActiveModel = user.into();
        active_model.update(&self.connection).await?;
        Ok(())
    }

    /// 根据ID选择性更新用户信息
    async fn update_by_primary_key_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 与update_by_primary_key方法实现相同，在实际应用中可以根据需要进行区分
        self.update_by_primary_key(user).await
    }

    /// 根据ID删除用户
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use sea_orm::ActiveModelTrait;
        use sea_orm::prelude::*;

        let user: sys_user::ActiveModel = Entity::find_by_id(id)
            .one(&self.connection)
            .await?
            .ok_or("User not found")?
            .into();

        user.delete(&self.connection).await?;
        Ok(())
    }
}
