use std::collections::HashMap;
use std::sync::Arc;

use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper, SingleWrapper};
use uuid::Uuid;

use crate::{
    models::{Role, RoleMenu},
    params::role_param::RoleParam,
    repositories::{role::role_repository::RoleRepository, role_menu::role_menu_repository::RoleMenuRepository},
    services::role::role_service::RoleService,
};

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::role::sqlx_impl::RoleRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::role::diesel_impl::RoleRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::role::seaorm_impl::RoleRepositorySeaormImpl;

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::role_menu::sqlx_impl::RoleMenuRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::role_menu::diesel_impl::RoleMenuRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::role_menu::seaorm_impl::RoleMenuRepositorySeaormImpl;

/// 角色服务实现
pub struct RoleServiceImpl {
    role_repository: Arc<dyn RoleRepository>,
    role_menu_repository: Arc<dyn RoleMenuRepository>,
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
    pub async fn new(_database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let role_repository = RoleRepositorySqlxImpl::new();
        #[cfg(feature = "sqlx_impl")]
        let role_menu_repository = RoleMenuRepositorySqlxImpl::new();

        #[cfg(feature = "diesel_impl")]
        let role_repository = RoleRepositoryDieselImpl::new(); // Diesel不需要数据库URL
        #[cfg(feature = "diesel_impl")]
        let role_menu_repository = RoleMenuRepositoryDieselImpl::new();

        #[cfg(feature = "seaorm_impl")]
        let role_repository = RoleRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现
        #[cfg(feature = "seaorm_impl")]
        let role_menu_repository = RoleMenuRepositorySeaormImpl::new().await.unwrap();

        Self {
            role_repository: Arc::new(role_repository),
            role_menu_repository: Arc::new(role_menu_repository),
        }
    }
}

#[rocket::async_trait]
impl RoleService for RoleServiceImpl {
    async fn role_set_menu(&self, role_id: &str, menu_ids: &[String]) -> ResponseWrapper {
        // 先删除角色的所有菜单关联
        if let Err(e) = self.role_menu_repository.delete_by_role_id(role_id).await {
            let mut response = ResponseWrapper::fail_default();
            response.set_fail(&format!("删除角色菜单关联失败: {}", e));
            return response;
        }

        // 批量插入新的角色菜单关联
        let role_menus: Vec<RoleMenu> = menu_ids
            .iter()
            .map(|menu_id| RoleMenu { role_id: role_id.to_string(), menu_id: menu_id.clone() })
            .collect();

        match self.role_menu_repository.batch_insert(&role_menus).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("批量插入角色菜单关联失败: {}", e));
                response
            },
        }
    }

    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> SingleWrapper<std::collections::HashSet<String>> {
        match self
            .role_menu_repository
            .select_role_menu_by_role_id(role_id)
            .await
        {
            Ok(role_menus) => {
                let menu_ids: std::collections::HashSet<String> = role_menus
                    .into_iter()
                    .map(|role_menu| role_menu.menu_id)
                    .collect();
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(menu_ids);
                wrapper
            },
            Err(e) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail(&format!("查询角色菜单关联失败: {}", e));
                wrapper
            },
        }
    }

    async fn select_role_infos(&self, user_id: Option<&str>, user_name: Option<&str>) -> ListWrapper<Role> {
        // 构建查询条件
        let role = Role::default();
        // TODO: 根据user_id和user_name构建查询条件

        match self.role_repository.select_roles(role).await {
            Ok(roles) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(roles);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询角色信息失败: {}", e));
                wrapper
            },
        }
    }
}
