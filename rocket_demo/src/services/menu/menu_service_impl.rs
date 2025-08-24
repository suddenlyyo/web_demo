use std::collections::HashMap;
use std::sync::Arc;

use common_wrapper::{ListWrapper, ResponseWrapper};
use uuid::Uuid;

use crate::{
    models::{Menu, MenuParam},
    repositories::menu::menu_repository::MenuRepository,
    services::menu::menu_service::{MenuService, RouterVO, TreeVO},
};

use super::MENU_REPO;

/// 菜单服务实现
pub struct MenuServiceImpl {
    repository: Arc<dyn MenuRepository>,
}

impl MenuServiceImpl {
    /// 创建菜单服务实例
    ///
    /// # 参数
    ///
    /// - `database_url`: 数据库连接URL
    ///
    /// # 返回值
    ///
    /// 返回新的菜单服务实例
    pub async fn new(database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let repository = MenuRepositorySqlxImpl::from_database_url(database_url)
            .await
            .unwrap();

        #[cfg(feature = "diesel_impl")]
        let repository = MenuRepositoryDieselImpl::new(); // Diesel不需要数据库URL

        #[cfg(feature = "seaorm_impl")]
        let repository = MenuRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现

        Self { repository: Arc::new(repository) }
    }
}

#[rocket::async_trait]
impl MenuService for MenuServiceImpl {
    async fn select_menu_tree_by_user_id(&self, user_id: &str) -> Vec<RouterVO> {
        match self.repository.select_menu_tree_by_user_id(user_id).await {
            Ok(menu_list) => self.build_menu_tree(menu_list),
            Err(_) => Vec::new(),
        }
    }

    async fn select_menu_list(&self, menu_param: MenuParam) -> ListWrapper<Menu> {
        match self.repository.select_menu_list(menu_param).await {
            Ok(menu_list) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(menu_list);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取菜单列表失败");
                wrapper
            },
        }
    }

    async fn add_menu(&self, menu_param: MenuParam) -> ResponseWrapper {
        let menu = Menu {
            id: Uuid::new_v4().to_string(),
            menu_name: menu_param.menu_name.clone(),
            parent_id: menu_param.parent_id.clone(),
            order_num: menu_param.order_num,
            path: menu_param.path.clone(),
            component: menu_param.component.clone(),
            is_frame: menu_param.is_frame,
            menu_type: menu_param.menu_type.clone(),
            visible: menu_param.visible,
            status: menu_param.status,
            perms: menu_param.perms.clone(),
            icon: menu_param.icon.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        match self.repository.add_menu(menu).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("新增菜单成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("新增菜单失败");
                wrapper
            },
        }
    }

    async fn edit_menu(&self, menu_param: MenuParam) -> ResponseWrapper {
        match self.repository.edit_menu(menu_param).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("编辑菜单成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("编辑菜单失败");
                wrapper
            },
        }
    }

    async fn edit_menu_status(&self, id: &str, status: i32) -> ResponseWrapper {
        match self.repository.edit_menu_status(id, status).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("修改菜单状态成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("修改菜单状态失败");
                wrapper
            },
        }
    }

    async fn delete_menu(&self, menu_id: &str) -> ResponseWrapper {
        match self.repository.delete_menu(menu_id).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("删除菜单成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("删除菜单失败");
                wrapper
            },
        }
    }

    async fn get_menu_tree(&self, menu_param: MenuParam) -> ListWrapper<TreeVO> {
        match self.repository.select_menu_list(menu_param).await {
            Ok(menu_list) => {
                let mut wrapper = ListWrapper::new();
                let tree_list = self.build_tree(menu_list, "0");
                wrapper.set_success(tree_list);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取菜单树失败");
                wrapper
            },
        }
    }

    async fn get_menu(&self) -> HashMap<String, Menu> {
        match self.repository.get_menu().await {
            Ok(menu_map) => menu_map,
            Err(_) => HashMap::new(),
        }
    }

    async fn select_sys_menu_infos(&self, user_id: Option<&str>, user_name: Option<&str>) -> ListWrapper<Menu> {
        match self
            .repository
            .select_sys_menu_infos(user_id, user_name)
            .await
        {
            Ok(menu_list) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(menu_list);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取系统菜单信息失败");
                wrapper
            },
        }
    }
}
