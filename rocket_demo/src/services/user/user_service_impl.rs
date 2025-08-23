//! 用户服务实现层

use crate::{
    models::{User, user::UserQuery},
    repositories::user::user_repository::UserRepository,
    services::user::user_service::UserService,
};
use common_wrapper::{ListWrapper, PageWrapper, ResponseTrait, SingleWrapper};

// 根据启用的feature导入对应的实现
#[cfg(feature = "sqlx_impl")]
use crate::repositories::user::sqlx_impl::UserRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::user::diesel_impl::UserRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::user::seaorm_impl::UserRepositorySeaormImpl;

/// 用户服务实现
pub struct UserServiceImpl {
    repository: Box<dyn UserRepository>,
}

impl UserServiceImpl {
    /// 创建新的用户服务实例
    ///
    /// # 参数
    ///
    /// - `database_url`: 数据库连接URL
    ///
    /// # 返回值
    ///
    /// 返回新的用户服务实例
    pub async fn new(database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let repository = UserRepositorySqlxImpl::from_database_url(database_url).await;

        #[cfg(feature = "diesel_impl")]
        let repository = UserRepositoryDieselImpl::new(); // Diesel不需要数据库URL

        #[cfg(feature = "seaorm_impl")]
        let repository = UserRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现

        Self { repository: Box::new(repository) }
    }
}

#[rocket::async_trait]
impl UserService for UserServiceImpl {
    /// 根据ID获取用户信息
    ///
    /// # 参数
    ///
    /// - `id`: 用户ID
    ///
    /// # 返回值
    ///
    /// 返回包装后的用户信息
    async fn get_user_by_id(&self, id: &str) -> SingleWrapper<User> {
        match self.repository.get_user_by_id(id).await {
            Ok(user) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(user);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("用户不存在");
                wrapper
            },
        }
    }

    /// 获取用户列表
    ///
    /// # 返回值
    ///
    /// 返回包装后的用户列表
    async fn list_users(&self) -> ListWrapper<User> {
        match self.repository.list_users().await {
            Ok(users) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(users);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取用户列表失败");
                wrapper
            },
        }
    }

    /// 分页查询用户列表
    async fn list_users_by_page_with_conditions(&self, query: UserQuery) -> PageWrapper<User> {
        match self
            .repository
            .list_users_by_page_with_conditions(query)
            .await
        {
            Ok((users, total_count)) => {
                let total_page = (total_count as f64 / page_size as f64).ceil() as u64;
                let mut wrapper = PageWrapper::new();
                wrapper.set_success(users, total_count, total_page, page_num, page_size);
                wrapper
            },
            Err(_) => {
                let mut wrapper = PageWrapper::new();
                wrapper.set_fail("获取用户列表失败");
                wrapper
            },
        }
    }

    /// 新增用户
    async fn add_user(&self, user: User) -> SingleWrapper<User> {
        match self.repository.add_user(user).await {
            Ok(user) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(user);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("新增用户失败");
                wrapper
            },
        }
    }

    /// 修改用户
    async fn update_user(&self, user: User) -> SingleWrapper<User> {
        match self.repository.update_user(user).await {
            Ok(user) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(user);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("修改用户失败");
                wrapper
            },
        }
    }

    /// 删除用户
    async fn delete_user(&self, id: &str) -> SingleWrapper<User> {
        match self.repository.delete_user(id).await {
            Ok(user) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(user);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("删除用户失败");
                wrapper
            },
        }
    }

    /// 修改用户状态
    async fn update_user_status(&self, id: &str, status: i32) -> SingleWrapper<User> {
        match self.repository.update_user_status(id, status).await {
            Ok(user) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(user);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("修改用户状态失败");
                wrapper
            },
        }
    }
}
