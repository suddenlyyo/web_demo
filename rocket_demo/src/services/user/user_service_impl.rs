//! 用户服务实现

use common_wrapper::{ResponseWrapper, SingleWrapper};
use rocket::futures::StreamExt;
use uuid::Uuid;

use crate::{
    models::{User, UserParam, UserQuery, UserRole},
    repositories::{
        dept::dept_repository::DeptRepository,
        role::role_repository::RoleRepository,
        user::user_repository::UserRepository,
        user_role::user_role_repository::UserRoleRepository,
    },
};

use super::{UserService, DEPT_REPO, ROLE_REPO, USER_REPO, USER_ROLE_REPO};

// 根据启用的feature导入对应的实现
#[cfg(feature = "sqlx_impl")]
use crate::repositories::user::sqlx_impl::UserRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::user::diesel_impl::UserRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::user::seaorm_impl::UserRepositorySeaormImpl;

#[cfg(feature = "sqlx")]
use crate::repositories::user_role::sqlx_impl::UserRoleRepositorySqlxImpl;
#[cfg(feature = "diesel")]
use crate::repositories::user_role::diesel_impl::UserRoleRepositoryDieselImpl;
#[cfg(feature = "sea-orm")]
use crate::repositories::user_role::seaorm_impl::UserRoleRepositorySeaormImpl;

/// 用户服务实现
pub struct UserServiceImpl {
    repository: Box<dyn UserRepository>,
    user_role_repository: Box<dyn UserRoleRepository>,
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

        #[cfg(feature = "sqlx_impl")]
        let user_role_repository = UserRoleRepositorySqlxImpl::new(/* TODO: 添加数据库连接参数 */);

        #[cfg(feature = "diesel_impl")]
        let user_role_repository = UserRoleRepositoryDieselImpl::new(); // Diesel实现

        #[cfg(feature = "seaorm_impl")]
        let user_role_repository = UserRoleRepositorySeaormImpl::new(/* TODO: 添加数据库连接参数 */); // SeaORM实现

        Self { 
            repository: Box::new(repository),
            user_role_repository: Box::new(user_role_repository),
        }
    }
}

#[rocket::async_trait]
impl UserService for UserServiceImpl {
    /// 根据用户名查询用户信息
    async fn select_user_by_user_name(&self, user_name: &str) -> Option<User> {
        match self.repository.find_by_name(user_name).await {
            Ok(user) => user,
            Err(_) => None,
        }
    }

    /// 分页查询用户列表
    async fn get_user_list_by_page(&self, user_param: UserParam) -> PageWrapper<User> {
        let mut page_wrapper = PageWrapper::new();
        let user_query = UserQuery {
            id: user_param.id.clone(),
            user_name: user_param.user_name.clone(),
            phone_number: user_param.phone_number.clone(),
            status: user_param.status,
            page_num: user_param.page_num.unwrap_or(1),
            page_size: user_param.page_size.unwrap_or(10),
        };

        match self.repository.get_user_list_by_page(&user_query).await {
            Ok(users) => {
                match self.repository.get_user_list_count(&user_query).await {
                    Ok(total) => {
                        page_wrapper.set_success(
                            users,
                            total,
                            user_query.page_num,
                            user_query.page_size,
                        );
                    }
                    Err(_) => {
                        page_wrapper.set_fail("获取用户总数失败");
                    }
                }
            }
            Err(_) => {
                page_wrapper.set_fail("获取用户列表失败");
            }
        }

        page_wrapper
    }

    /// 新增用户
    async fn add_user(&self, user_param: UserParam) -> ResponseWrapper {
        let mut response = ResponseWrapper::success_default();
        let user = User {
            id: Uuid::new_v4().to_string(),
            user_name: user_param.user_name.unwrap_or_default(),
            password: user_param.password.unwrap_or_default(),
            email: user_param.email.unwrap_or_default(),
            phone_number: user_param.phone_number.unwrap_or_default(),
            sex: user_param.sex.unwrap_or_default(),
            avatar: user_param.avatar.unwrap_or_default(),
            status: user_param.status.unwrap_or(1),
            login_ip: user_param.login_ip.unwrap_or_default(),
            login_time: user_param.login_time,
            create_by: user_param.create_by.unwrap_or_default(),
            create_time: Some(chrono::Utc::now()),
            update_by: user_param.update_by.unwrap_or_default(),
            update_time: Some(chrono::Utc::now()),
            remark: user_param.remark.unwrap_or_default(),
        };

        if let Err(_) = self.repository.insert(&user).await {
            response.set_fail("新增用户失败");
        }

        response
    }

    /// 编辑用户
    async fn edit_user(&self, user_param: UserParam) -> ResponseWrapper {
        let mut response = ResponseWrapper::success_default();

        if let Some(id) = user_param.id.clone() {
            match self.repository.select_by_primary_key(&id).await {
                Ok(Some(mut user)) => {
                    // 更新用户信息
                    if let Some(user_name) = user_param.user_name {
                        user.user_name = user_name;
                    }
                    if let Some(email) = user_param.email {
                        user.email = email;
                    }
                    if let Some(phone_number) = user_param.phone_number {
                        user.phone_number = phone_number;
                    }
                    if let Some(sex) = user_param.sex {
                        user.sex = sex;
                    }
                    if let Some(avatar) = user_param.avatar {
                        user.avatar = avatar;
                    }
                    if let Some(status) = user_param.status {
                        user.status = status;
                    }
                    if let Some(login_ip) = user_param.login_ip {
                        user.login_ip = login_ip;
                    }
                    if let Some(login_time) = user_param.login_time {
                        user.login_time = Some(login_time);
                    }
                    if let Some(update_by) = user_param.update_by {
                        user.update_by = update_by;
                    }
                    user.update_time = Some(chrono::Utc::now());
                    if let Some(remark) = user_param.remark {
                        user.remark = remark;
                    }

                    if let Err(_) = self.repository.update_by_primary_key_selective(&user).await {
                        response.set_fail("编辑用户失败");
                    }
                }
                Ok(None) => {
                    response.set_fail("用户不存在");
                }
                Err(_) => {
                    response.set_fail("查询用户失败");
                }
            }
        } else {
            response.set_fail("用户ID不能为空");
        }

        response
    }

    /// 编辑用户状态
    async fn edit_user_status(&self, id: &str, status: i32) -> ResponseWrapper {
        let mut response = ResponseWrapper::success_default();

        match self.repository.select_by_primary_key(id).await {
            Ok(Some(mut user)) => {
                user.status = status;
                user.update_time = Some(chrono::Utc::now());

                if let Err(_) = self.repository.update_by_primary_key_selective(&user).await {
                    response.set_fail("编辑用户状态失败");
                }
            }
            Ok(None) => {
                response.set_fail("用户不存在");
            }
            Err(_) => {
                response.set_fail("查询用户失败");
            }
        }

        response
    }

    /// 删除用户
    async fn delete_user(&self, user_id: &str) -> ResponseWrapper {
        let mut response = ResponseWrapper::success_default();

        match self.repository.delete_by_primary_key(user_id).await {
            Ok(_) => {}
            Err(_) => {
                response.set_fail("删除用户失败");
            }
        }

        response
    }

    /// 重置密码
    async fn reset_user_pwd(&self, user_param: UserParam) -> ResponseWrapper {
        let mut response = ResponseWrapper::success_default();

        if let Some(id) = user_param.id.clone() {
            match self.repository.select_by_primary_key(&id).await {
                Ok(Some(mut user)) => {
                    if let Some(password) = user_param.password {
                        user.password = password;
                    }
                    user.update_time = Some(chrono::Utc::now());

                    if let Err(_) = self.repository.update_by_primary_key_selective(&user).await {
                        response.set_fail("重置密码失败");
                    }
                }
                Ok(None) => {
                    response.set_fail("用户不存在");
                }
                Err(_) => {
                    response.set_fail("查询用户失败");
                }
            }
        } else {
            response.set_fail("用户ID不能为空");
        }

        response
    }

    /// 分配角色
    async fn set_user_role(&self, user_id: &str, role_ids: &[String]) -> ResponseWrapper {
        let mut response = ResponseWrapper::success_default();
        
        // 先删除用户的所有角色
        if let Err(e) = self.user_role_repository.delete_by_user_id(user_id).await {
            response.set_fail(&format!("删除用户角色失败: {}", e));
            return response;
        }
        
        // 然后添加新的角色
        let user_roles: Vec<UserRole> = role_ids
            .iter()
            .map(|role_id| UserRole {
                user_id: user_id.to_string(),
                role_id: role_id.clone(),
            })
            .collect();
            
        if let Err(e) = self.user_role_repository.batch_insert(&user_roles).await {
            response.set_fail(&format!("分配角色失败: {}", e));
            return response;
        }
        
        response
    }

    /// 查询用户的角色id列表
    async fn select_role_ids_by_user_id(&self, user_id: &str) -> SingleWrapper<HashSet<String>> {
        let mut wrapper = SingleWrapper::new();
        
        match self.user_role_repository.select_user_role_by_user_id(user_id).await {
            Ok(user_roles) => {
                let role_ids: HashSet<String> = user_roles.into_iter().map(|ur| ur.role_id).collect();
                wrapper.set_success(role_ids);
            }
            Err(e) => {
                wrapper.set_fail(&format!("查询用户角色失败: {}", e));
            }
        }
        
        wrapper
    }
}
