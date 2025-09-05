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
    async fn insert(&self, row: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    async fn select_by_primary_key(&self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(User::from))
    }

    async fn select_user_list(&self, user: &User) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = Entity::find();

        if let Some(name) = &user.name {
            query = query.filter(Column::Name.contains(name));
        }

        if let Some(phone_number) = &user.phone_number {
            query = query.filter(Column::PhoneNumber.contains(phone_number));
        }

        if let Some(status) = user.status {
            query = query.filter(Column::Status.eq(status));
        }

        let result = query.all(&self.connection).await?;
        let users: Vec<User> = result.into_iter().map(User::from).collect();
        Ok(users)
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find()
            .filter(Column::Name.eq(name))
            .one(&self.connection)
            .await?;
        Ok(result.map(User::from))
    }

    async fn get_user_list_by_page(
        &self, name: Option<String>, dept_id: Option<String>, email: Option<String>, phone_number: Option<String>, status: Option<i32>, start_date: Option<chrono::DateTime<chrono::Utc>>, end_date: Option<chrono::DateTime<chrono::Utc>>, page_num: u64, page_size: u64,
    ) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现分页查询逻辑
        let _ = (name, dept_id, email, phone_number, status, start_date, end_date, page_num, page_size);
        Ok(vec![])
    }

    async fn get_user_list_count(
        &self, name: Option<String>, dept_id: Option<String>, email: Option<String>, phone_number: Option<String>, status: Option<i32>, start_date: Option<chrono::DateTime<chrono::Utc>>, end_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现查询总数逻辑
        let _ = (name, dept_id, email, phone_number, status, start_date, end_date);
        Ok(0)
    }

    async fn update_by_primary_key(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = user.into();
        model.update(&self.connection).await?;
        Ok(())
    }

    async fn update_by_primary_key_selective(&self, user: &User) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut query = Entity::find_by_id(user.id.clone());

        if let Some(name) = &user.name {
            query = query.filter(Column::Name.eq(name));
        }

        if let Some(phone_number) = &user.phone_number {
            query = query.filter(Column::PhoneNumber.eq(phone_number));
        }

        if let Some(status) = user.status {
            query = query.filter(Column::Status.eq(status));
        }

        // TODO: 实现选择性更新逻辑
        let _ = query;
        Ok(())
    }

    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        if result.rows_affected == 0 {
            return Err(Box::from("用户删除失败"));
        }
        Ok(())
    }

    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现根据角色ID查询用户角色列表逻辑
        let _ = role_id;
        Ok(vec![])
    }

    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现根据用户ID查询用户角色列表逻辑
        let _ = user_id;
        Ok(vec![])
    }

    async fn batch_insert_user_role(&self, list: &[UserRole]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现批量插入用户角色逻辑
        let _ = list;
        Ok(())
    }

    async fn batch_delete_user_role_by_user_and_role_ids(&self, user_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现根据用户ID和角色ID列表批量删除用户角色逻辑
        let _ = (user_id, list);
        Ok(())
    }

    async fn delete_user_role_by_user_id(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // TODO: 实现根据用户ID删除用户角色逻辑
        let _ = user_id;
        Ok(())
    }
}
