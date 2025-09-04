use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Order};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::models::User;
use crate::models::constants::USER_FIELDS;
use crate::repositories::user::user_repository::UserRepository;
use crate::services::params::user_param::UserParam;
use common_wrapper::PageInfo;

// 导入SeaORM实体
use crate::entities::sys_user;
use crate::entities::sys_user::{ActiveModel, Column, Entity, Model};

impl From<&User> for ActiveModel {
    fn from(user: &User) -> Self {
        ActiveModel {
            id: Set(user.id.clone()),
            dept_id: Set(user.dept_id.clone()),
            name: Set(user.name.clone()),
            email: Set(user.email.clone()),
            phone_number: Set(user.phone_number.clone()),
            sex: Set(user.sex.clone()),
            password: Set(user.password.clone()),
            avatar: Set(user.avatar.clone()),
            status: Set(user.status),
            login_ip: Set(user.login_ip.clone()),
            login_time: Set(user.login_time.map(|t| t.naive_utc())),
            create_by: Set(user.create_by.clone()),
            create_time: Set(user.create_time.map(|t| t.naive_utc())),
            update_by: Set(user.update_by.clone()),
            update_time: Set(user.update_time.map(|t| t.naive_utc())),
            remark: Set(user.remark.clone()),
        }
    }
}

impl From<Model> for User {
    fn from(model: Model) -> Self {
        User {
            id: model.id,
            dept_id: model.dept_id,
            name: model.name,
            email: model.email,
            phone_number: model.phone_number,
            sex: model.sex,
            password: model.password,
            avatar: model.avatar,
            status: model.status,
            login_ip: model.login_ip,
            login_time: model
                .login_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            create_by: model.create_by,
            create_time: model
                .create_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: model.update_by,
            update_time: model
                .update_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: model.remark,
        }
    }
}

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
    fn build_condition(query: &UserParam) -> Condition {
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

        condition
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositorySeaormImpl {
    /// 根据主键删除用户
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;

        if result.rows_affected == 0 {
            return Err(Box::from("用户删除失败"));
        }

        Ok(())
    }

    /// 插入用户记录
    async fn insert(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 选择性插入用户记录
    async fn insert_selective(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 根据主键查询用户
    async fn select_user_by_id(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(User::from))
    }

    /// 查询用户列表
    async fn select_user_list(&self, user_param: UserParam) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let condition = Self::build_condition(&user_param);
        let result = Entity::find()
            .filter(condition)
            .order_by(Column::Id, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(result.into_iter().map(User::from).collect())
    }

    /// 根据主键更新用户
    async fn update_by_id(&self, row: &User) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键选择性更新用户
    async fn update_by_id_selective(&self, row: &User) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut model: ActiveModel = row.into();
        // 将主键设置为未修改，因为我们使用它进行查找而不是更新
        model.id = sea_orm::ActiveValue::Unchanged(row.id.clone());
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键删除用户
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }

    /// 根据用户名查询用户
    async fn select_user_by_name(&self, username: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find()
            .filter(Column::Name.eq(username))
            .one(&self.connection)
            .await?;
        Ok(result.map(User::from))
    }
}
