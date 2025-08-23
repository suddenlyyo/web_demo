//! 角色服务实现

use std::sync::Arc;

use crate::models::role::Role;
use crate::repositories::role::role_repository::RoleRepository;
use crate::repositories::role::sqlx_impl::RoleRepositorySqlxImpl;
use crate::services::role::role_service::RoleService;
use common_wrapper::{ListWrapper, PageWrapper, ResponseTrait, SingleWrapper};

/// 角色服务实现
pub struct RoleServiceImpl {
    repository: Arc<dyn RoleRepository>,
}

impl RoleServiceImpl {
    /// 创建角色服务实例
    ///
    /// # 参数
    ///
    /// - `database_url`: 数据库连接URL
    ///
    /// # 返回值
    ///
    /// 返回新的角色服务实例
    pub async fn new(database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let repository = RoleRepositorySqlxImpl::from_database_url(database_url)
            .await
            .unwrap();

        #[cfg(feature = "diesel_impl")]
        let repository = RoleRepositoryDieselImpl::new(); // Diesel不需要数据库URL

        #[cfg(feature = "seaorm_impl")]
        let repository = RoleRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现

        Self { repository: Arc::new(repository) }
    }
}

#[rocket::async_trait]
impl RoleService for RoleServiceImpl {
    /// 根据ID获取角色信息
    async fn get_role_by_id(&self, id: &str) -> SingleWrapper<Role> {
        match self.repository.get_role_by_id(id).await {
            Ok(role) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(role);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("角色不存在");
                wrapper
            },
        }
    }

    /// 获取角色列表
    async fn list_roles(&self) -> ListWrapper<Role> {
        match self.repository.list_roles().await {
            Ok(roles) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(roles);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取角色列表失败");
                wrapper
            },
        }
    }

    /// 分页查询角色列表
    async fn list_roles_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> PageWrapper<Role> {
        match self
            .repository
            .list_roles_by_page(page_num, page_size)
            .await
        {
            Ok((roles, total, page_count)) => {
                let mut wrapper = PageWrapper::new();
                let current_page = page_num.unwrap_or(1);
                let page_size_value = page_size.unwrap_or(10);
                wrapper.set_success(roles, total, page_count, current_page, page_size_value);
                wrapper
            },
            Err(_) => {
                let mut wrapper = PageWrapper::new();
                wrapper.set_fail("获取角色列表失败");
                wrapper
            },
        }
    }

    /// 新增角色
    async fn add_role(&self, role: Role) -> SingleWrapper<Role> {
        match self.repository.add_role(role).await {
            Ok(role) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(role);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("新增角色失败");
                wrapper
            },
        }
    }

    /// 修改角色
    async fn update_role(&self, role: Role) -> SingleWrapper<Role> {
        match self.repository.update_role(role).await {
            Ok(role) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(role);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("修改角色失败");
                wrapper
            },
        }
    }

    /// 删除角色
    async fn delete_role(&self, id: &str) -> SingleWrapper<Role> {
        match self.repository.delete_role(id).await {
            Ok(role) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(role);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("删除角色失败");
                wrapper
            },
        }
    }

    /// 修改角色状态
    async fn update_role_status(&self, id: &str, status: i32) -> SingleWrapper<Role> {
        match self.repository.update_role_status(id, status).await {
            Ok(role) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(role);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("修改角色状态失败");
                wrapper
            },
        }
    }
}
