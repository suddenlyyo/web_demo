use std::sync::Arc;

use common_wrapper::{ListWrapper, PageInfo, PageWrapper, ResponseTrait, ResponseWrapper, SingleWrapper};
use rocket::data::N;
use uuid::Uuid;

use crate::{
    config::Config,
    controllers::user,
    models::{User, UserRole},
    params::{
        page_param::{self, PageParam},
        user_param::UserParam,
    },
    repositories::{user::user_repository::UserRepository, user_role::user_role_repository::UserRoleRepository},
    services::user::user_service::UserService,
};

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::user::sqlx_impl::UserRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::user::diesel_impl::UserRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::user::seaorm_impl::UserRepositorySeaormImpl;

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::user_role::sqlx_impl::UserRoleRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::user_role::diesel_impl::UserRoleRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::user_role::seaorm_impl::UserRoleRepositorySeaormImpl;

/// 用户服务实现
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository>,
    user_role_repository: Arc<dyn UserRoleRepository>,
}

impl UserServiceImpl {
    /// 创建用户服务实例
    ///
    /// # 返回值
    ///
    /// 返回新的用户服务实例
    pub async fn new() -> Self {
        #[cfg(feature = "sqlx_impl")]
        let user_repository = UserRepositorySqlxImpl::new().await.unwrap();
        #[cfg(feature = "sqlx_impl")]
        let user_role_repository = UserRoleRepositorySqlxImpl::new().await.unwrap();

        #[cfg(feature = "diesel_impl")]
        let user_repository = UserRepositoryDieselImpl::new();
        #[cfg(feature = "diesel_impl")]
        let user_role_repository = UserRoleRepositoryDieselImpl::new();

        #[cfg(feature = "seaorm_impl")]
        let user_repository = UserRepositorySeaormImpl::new().await.unwrap();
        #[cfg(feature = "seaorm_impl")]
        let user_role_repository = UserRoleRepositorySeaormImpl::new().await.unwrap();

        Self {
            user_repository: Arc::new(user_repository),
            user_role_repository: Arc::new(user_role_repository),
        }
    }
}

#[rocket::async_trait]
impl UserService for UserServiceImpl {
    async fn add_user(&self, user_param: UserParam) -> ResponseWrapper {
        // 构造用户对象
        let user = User {
            id: Uuid::new_v4().to_string(),
            dept_id: user_param.dept_id.clone(),
            name: user_param.name.clone(),
            email: user_param.email.clone(),
            phone_number: user_param.phone_number.clone(),
            sex: user_param.sex.clone(),
            password: user_param.password.clone(),
            avatar: user_param.avatar.clone(),
            status: user_param.status,
            login_ip: None,   //创建时不设置登录IP
            login_time: None, //创建时不设置登录时间
            create_by: user_param.create_by.clone(),
            create_time: Some(chrono::Utc::now()),
            update_by: None,   //创建时不设置更新者
            update_time: None, //创建时不设置更新时间
            remark: user_param.remark.clone(),
        };

        match self.user_repository.insert_selective(&user).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("添加用户失败: {}", e));
                response
            },
        }
    }

    async fn edit_user(&self, user_param: UserParam) -> ResponseWrapper {
        if let Some(user_id) = &user_param.id {
            // 构造用户对象
            let user = User {
                id: user_id.clone(),
                dept_id: user_param.dept_id.clone(),
                name: user_param.name.clone(),
                email: user_param.email.clone(),
                phone_number: user_param.phone_number.clone(),
                sex: user_param.sex.clone(),
                password: user_param.password.clone(),
                avatar: user_param.avatar.clone(),
                status: user_param.status,
                login_ip: None,    // 不更新登录IP
                login_time: None,  // 不更新登录时间
                create_by: None,   // 不更新创建者
                create_time: None, // 不更新创建时间
                update_by: user_param.update_by.clone(),
                update_time: Some(chrono::Utc::now()),
                remark: user_param.remark.clone(),
            };

            match self
                .user_repository
                .update_by_primary_key_selective(&user)
                .await
            {
                Ok(_) => ResponseWrapper::success_default(),
                Err(e) => {
                    let mut response = ResponseWrapper::fail_default();
                    response.set_fail(&format!("更新用户失败: {}", e));
                    response
                },
            }
        } else {
            let mut response = ResponseWrapper::fail_default();
            response.set_fail("用户ID不能为空");
            response
        }
    }

    async fn edit_user_status(&self, id: &str, status: i32) -> ResponseWrapper {
        // 构造只更新状态的用户对象
        let user = User {
            id: id.to_string(),
            status: Some(status),
            update_time: Some(chrono::Utc::now()),
            // 其他字段设置为None或默认值，因为是选择性更新
            dept_id: None,
            name: None,
            email: None,
            phone_number: None,
            sex: None,
            password: None,
            avatar: None,
            login_ip: None,
            login_time: None,
            create_by: None,
            create_time: None,
            update_by: user_param.update_by.clone(),
            remark: None,
        };

        match self
            .user_repository
            .update_by_primary_key_selective(&user)
            .await
        {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("修改用户状态失败: {}", e));
                response
            },
        }
    }

    async fn delete_user(&self, user_id: &str) -> ResponseWrapper {
        match self.user_repository.delete_by_primary_key(user_id).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("删除用户失败: {}", e));
                response
            },
        }
    }

    async fn select_user_by_user_name(&self, user_name: &str) -> Option<User> {
        match self.user_repository.find_by_name(user_name).await {
            Ok(user) => user,
            Err(_) => None,
        }
    }

    async fn get_user_list_by_page(&self, mut user_param: UserParam) -> PageWrapper<User> {
        // 设置分页参数
        let page_param = PageInfo::new(user_param.page_param.page_num, user_param.page_param.page_size);
        user_param.page_param = page_param;

        // 获取用户总数
        let count_result = self.user_repository.get_user_list_count(&user_param).await;
        let count = match count_result {
            Ok(count) => count,
            Err(e) => {
                let mut wrapper = PageWrapper::new();
                wrapper.set_fail(&format!("获取用户总数失败: {}", e));
                return wrapper;
            },
        };

        // 获取用户列表
        let user_list_result = self
            .user_repository
            .get_user_list_by_page(&user_param)
            .await;
        match user_list_result {
            Ok(user_list) => {
                // 创建分页包装器
                let mut wrapper = PageWrapper::new();
                let total = count;
                let current_page = page_param::get_current_page_num(&user_param.page_param);
                let page_size = page_param::get_page_size(&user_param.page_param);
                wrapper.set_success(user_list, total, current_page, page_size);
                wrapper
            },
            Err(e) => {
                let mut wrapper = PageWrapper::new();
                wrapper.set_fail(&format!("查询用户列表失败: {}", e));
                wrapper
            },
        }
    }

    async fn reset_user_pwd(&self, user_param: UserParam) -> ResponseWrapper {
        if let Some(user_id) = &user_param.id {
            if let Some(password) = &user_param.password {
                // 构造只更新密码的用户对象
                let user = User {
                    id: user_id.to_string(),
                    password: Some(password.to_string()),
                    update_time: Some(chrono::Utc::now()),
                    // 其他字段设置为None或默认值，因为是选择性更新
                    dept_id: None,
                    name: None,
                    email: None,
                    phone_number: None,
                    sex: None,
                    avatar: None,
                    status: None,
                    login_ip: None,
                    login_time: None,
                    create_by: None,
                    create_time: None,
                    update_by: None,
                    remark: None,
                };

                match self
                    .user_repository
                    .update_by_primary_key_selective(&user)
                    .await
                {
                    Ok(_) => ResponseWrapper::success_default(),
                    Err(e) => {
                        let mut response = ResponseWrapper::fail_default();
                        response.set_fail(&format!("重置用户密码失败: {}", e));
                        response
                    },
                }
            } else {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail("密码不能为空");
                response
            }
        } else {
            let mut response = ResponseWrapper::fail_default();
            response.set_fail("用户ID不能为空");
            response
        }
    }

    async fn set_user_role(&self, user_id: &str, role_ids: &[String]) -> ResponseWrapper {
        // 先删除用户的所有角色
        match self.user_role_repository.delete_by_user_id(user_id).await {
            Ok(_) => {
                // 添加新角色
                for role_id in role_ids {
                    let user_role = UserRole {
                        user_id: user_id.to_string(),
                        role_id: role_id.to_string(),
                    };

                    if let Err(e) = self.user_role_repository.insert(&user_role).await {
                        let mut response = ResponseWrapper::fail_default();
                        response.set_fail(&format!("分配用户角色失败: {}", e));
                        return response;
                    }
                }
                ResponseWrapper::success_default()
            },
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("删除用户原有角色失败: {}", e));
                response
            },
        }
    }

    async fn select_role_ids_by_user_id(&self, user_id: &str) -> SingleWrapper<std::collections::HashSet<String>> {
        match self
            .user_role_repository
            .select_role_ids_by_user_id(user_id)
            .await
        {
            Ok(role_ids) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(role_ids);
                wrapper
            },
            Err(e) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail(&format!("查询用户角色失败: {}", e));
                wrapper
            },
        }
    }
}
