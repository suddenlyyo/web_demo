//! 角色服务实现

use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper, SingleWrapper};
use std::collections::HashSet;

use crate::{
    models::{Role, RoleMenu, RoleParam},
    repositories::{role::role_repository::RoleRepository, role_menu::role_menu_repository::RoleMenuRepository},
};

use super::{ROLE_MENU_REPO, ROLE_REPO, RoleService};

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::{role::sqlx_impl::RoleRepositorySqlxImpl, role_menu::sqlx_impl::RoleMenuRepositorySqlxImpl};

#[cfg(feature = "diesel_impl")]
use crate::repositories::{role::diesel_impl::RoleRepositoryDieselImpl, role_menu::diesel_impl::RoleMenuRepositoryDieselImpl};

#[cfg(feature = "seaorm_impl")]
use crate::repositories::{role::seaorm_impl::RoleRepositorySeaormImpl, role_menu::seaorm_impl::RoleMenuRepositorySeaormImpl};

/// 角色服务实现
pub struct RoleServiceImpl {
    repository: Box<dyn RoleRepository>,
    role_menu_repository: Box<dyn RoleMenuRepository>,
}

impl RoleServiceImpl {
    /// 创建新的角色服务实例
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

        #[cfg(feature = "sqlx_impl")]
        let role_menu_repository = RoleMenuRepositorySqlxImpl::from_database_url(database_url)
            .await
            .unwrap();

        #[cfg(feature = "diesel_impl")]
        let role_menu_repository = RoleMenuRepositoryDieselImpl::new(); // Diesel实现

        #[cfg(feature = "seaorm_impl")]
        let role_menu_repository = RoleMenuRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现

        Self {
            repository: Box::new(repository),
            role_menu_repository: Box::new(role_menu_repository),
        }
    }
}

#[rocket::async_trait]
impl RoleService for RoleServiceImpl {
    /// 角色设置权限
    async fn role_set_menu(&self, role_id: &str, menu_ids: &[String]) -> ResponseWrapper {
        let mut response = ResponseWrapper::success_default();

        // 先删除角色的所有菜单
        if let Err(e) = self.role_menu_repository.delete_by_role_id(role_id).await {
            response.set_fail(&format!("删除角色菜单失败: {}", e));
            return response;
        }

        // 然后添加新的菜单
        let role_menus: Vec<RoleMenu> = menu_ids
            .iter()
            .map(|menu_id| RoleMenu { role_id: role_id.to_string(), menu_id: menu_id.clone() })
            .collect();

        if let Err(e) = self.role_menu_repository.batch_insert(&role_menus).await {
            response.set_fail(&format!("设置角色菜单失败: {}", e));
            return response;
        }

        response
    }

    /// 查询角色的菜单id列表
    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> SingleWrapper<HashSet<String>> {
        let mut wrapper = SingleWrapper::new();

        match self
            .role_menu_repository
            .select_role_menu_by_role_id(role_id)
            .await
        {
            Ok(role_menus) => {
                let menu_ids: HashSet<String> = role_menus.into_iter().map(|rm| rm.menu_id).collect();
                wrapper.set_success(menu_ids);
            },
            Err(e) => {
                wrapper.set_fail(&format!("查询角色菜单失败: {}", e));
            },
        }

        wrapper
    }

    /// 查询角色信息
    async fn select_role_infos(&self, user_id: Option<&str>, user_name: Option<&str>) -> ListWrapper<Role> {
        let mut wrapper = ListWrapper::new();

        // 构建查询条件
        let mut role = Role::default();
        // TODO: 根据user_id和user_name构建查询条件

        match self.repository.select_role_list(&role).await {
            Ok(roles) => {
                wrapper.set_success(roles);
            },
            Err(e) => {
                wrapper.set_fail(&format!("查询角色信息失败: {}", e));
            },
        }

        wrapper
    }
}
