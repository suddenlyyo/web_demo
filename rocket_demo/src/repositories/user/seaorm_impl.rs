use sea_orm::ActiveValue::Set;
use sea_orm::{EntityTrait, QueryFilter};

use crate::models::User;
use crate::repositories::user::user_repository::UserRepository;

// ==================== 表结构体映射 ====================
use crate::entities::sys_user;
use crate::entities::sys_user::{ActiveModel, Column, Entity, Model};

impl From<&User> for ActiveModel {
    fn from(user: &User) -> Self {
        ActiveModel {
            id: Set(user.id.clone()),
            username: Set(user.username.clone()),
            password: Set(user.password.clone()),
            salt: Set(user.salt.clone()),
            nickname: Set(user.nickname.clone()),
            phone: Set(user.phone.clone()),
            email: Set(user.email.clone()),
            avatar: Set(user.avatar.clone()),
            sex: Set(user.sex.clone()),
            status: Set(user.status),
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
            username: model.username,
            password: model.password,
            salt: model.salt,
            nickname: model.nickname,
            phone: model.phone,
            email: model.email,
            avatar: model.avatar,
            sex: model.sex,
            status: model.status,
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

// ==================== SQL trait 实现 ====================
#[derive(Debug)]
pub struct UserRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

impl UserRepositorySeaormImpl {
    pub async fn new() -> Self {
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() {
            config.database.url
        } else {
            panic!("无法从配置文件获取数据库连接信息");
        };

        let connection = sea_orm::Database::connect(&database_url)
            .await
            .expect("连接MySQL数据库时出错");

        Self { connection }
    }
}

#[rocket::async_trait]
impl UserRepository for UserRepositorySeaormImpl {
    async fn select_user_by_id(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(User::from))
    }

    async fn select_user_list(&self, user_param: crate::params::user_param::UserParam) -> Result<common_wrapper::PageWrapper<Vec<User>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = Entity::find();

        if let Some(username) = user_param.username {
            query = query.filter(Column::Username.contains(username));
        }

        if let Some(phone) = user_param.phone {
            query = query.filter(Column::Phone.contains(phone));
        }

        if let Some(status) = user_param.status {
            query = query.filter(Column::Status.eq(status));
        }

        let total = query.clone().count(&self.connection).await?;
        let result = query.all(&self.connection).await?;
        let users: Vec<User> = result.into_iter().map(User::from).collect();

        Ok(common_wrapper::PageWrapper::new(users, common_wrapper::PageInfo::new(user_param.page_num, user_param.page_size, total)))
    }

    async fn insert(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    async fn update_by_id(&self, row: &User) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.update(&self.connection).await?;
        Ok(1)
    }

    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }
}
