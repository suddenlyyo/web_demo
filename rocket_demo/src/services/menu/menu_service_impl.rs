use std::collections::HashMap;
use std::sync::Arc;

use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper};
use uuid::Uuid;

use crate::{
    models::menu::Menu,
    params::{menu_param::MenuParam, page_param::PageParam},
    repositories::menu::menu_repository::MenuRepository,
    services::menu::menu_service::{MenuService, RouterVO, TreeVO},
};

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::menu::sqlx_impl::MenuRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::menu::diesel_impl::MenuRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::menu::seaorm_impl::MenuRepositorySeaormImpl;

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
    pub async fn new(_database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let repository = MenuRepositorySqlxImpl::new();

        #[cfg(feature = "diesel_impl")]
        let repository = MenuRepositoryDieselImpl::new(); // Diesel不需要数据库URL

        #[cfg(feature = "seaorm_impl")]
        let repository = MenuRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现

        Self { repository: Arc::new(repository) }
    }

    /// 构建菜单树
    fn build_menu_tree(&self, menu_list: Vec<Menu>) -> Vec<RouterVO> {
        // TODO: 实现构建菜单树的逻辑
        Vec::new()
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
        match self
            .repository
            .select_menu_list(menu_param.name, menu_param.status)
            .await
        {
            Ok(menu_list) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(menu_list);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询菜单列表失败: {}", e));
                wrapper
            },
        }
    }

    async fn add_menu(&self, menu_param: MenuParam) -> ResponseWrapper {
        let menu = Menu {
            id: Uuid::new_v4().to_string(),
            name: menu_param.name,
            parent_id: menu_param.parent_id,
            seq_no: menu_param.seq_no,
            menu_type: menu_param.menu_type,
            url: menu_param.url,
            perms: menu_param.perms,
            status: menu_param.status,
            hidden: menu_param.hidden,
            always_show: menu_param.always_show,
            redirect: menu_param.redirect,
            component: menu_param.component,
            href: menu_param.href,
            icon: menu_param.icon,
            no_cache: menu_param.no_cache,
            affix: menu_param.affix,
            breadcrumb: menu_param.breadcrumb,
            active_menu: menu_param.active_menu,
            create_by: menu_param.create_by,
            create_time: Some(chrono::Utc::now()),
            update_by: menu_param.update_by,
            update_time: Some(chrono::Utc::now()),
            remark: menu_param.remark,
        };

        match self.repository.insert_selective(&menu).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("添加菜单失败: {}", e));
                response
            },
        }
    }

    async fn edit_menu(&self, menu_param: MenuParam) -> ResponseWrapper {
        if let Some(menu_id) = &menu_param.id {
            let menu = Menu {
                id: menu_id.clone(),
                name: menu_param.name,
                parent_id: menu_param.parent_id,
                seq_no: menu_param.seq_no,
                menu_type: menu_param.menu_type,
                url: menu_param.url,
                perms: menu_param.perms,
                status: menu_param.status,
                hidden: menu_param.hidden,
                always_show: menu_param.always_show,
                redirect: menu_param.redirect,
                component: menu_param.component,
                href: menu_param.href,
                icon: menu_param.icon,
                no_cache: menu_param.no_cache,
                affix: menu_param.affix,
                breadcrumb: menu_param.breadcrumb,
                active_menu: menu_param.active_menu,
                create_by: menu_param.create_by,
                create_time: None, // 不更新创建时间
                update_by: menu_param.update_by,
                update_time: Some(chrono::Utc::now()),
                remark: menu_param.remark,
            };

            match self.repository.update_by_id_selective(&menu).await {
                Ok(_) => ResponseWrapper::success_default(),
                Err(e) => {
                    let mut response = ResponseWrapper::fail_default();
                    response.set_fail(&format!("更新菜单失败: {}", e));
                    response
                },
            }
        } else {
            let mut response = ResponseWrapper::fail_default();
            response.set_fail("菜单ID不能为空");
            response
        }
    }

    async fn edit_menu_status(&self, id: &str, status: i32) -> ResponseWrapper {
        // 构造只更新状态的菜单对象
        let menu = Menu {
            id: id.to_string(),
            status: Some(status),
            update_time: Some(chrono::Utc::now()),
            // 其他字段设置为None或默认值，因为是选择性更新
            name: None,
            parent_id: None,
            seq_no: None,
            menu_type: None,
            url: None,
            perms: None,
            hidden: None,
            always_show: None,
            redirect: None,
            component: None,
            href: None,
            icon: None,
            no_cache: None,
            affix: None,
            breadcrumb: None,
            active_menu: None,
            create_by: None,
            create_time: None,
            update_by: None,
            remark: None,
        };

        match self.repository.update_by_id_selective(&menu).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("修改菜单状态失败: {}", e));
                response
            },
        }
    }

    async fn delete_menu(&self, menu_id: &str) -> ResponseWrapper {
        match self.repository.delete_by_id(menu_id).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("删除菜单失败: {}", e));
                response
            },
        }
    }

    async fn get_menu_tree(&self, menu_param: MenuParam) -> ListWrapper<TreeVO> {
        match self
            .repository
            .select_menu_list(menu_param.name, menu_param.status)
            .await
        {
            Ok(menu_list) => {
                let mut wrapper = ListWrapper::new();
                let tree_vo_list: Vec<TreeVO> = menu_list
                    .into_iter()
                    .map(|menu| TreeVO {
                        id: Some(menu.id),
                        name: menu.name,
                        parent_id: menu.parent_id,
                        seq_no: menu.seq_no,
                        menu_type: menu.menu_type,
                        url: menu.url,
                        perms: menu.perms,
                        status: menu.status,
                        hidden: menu.hidden,
                        always_show: menu.always_show,
                        redirect: menu.redirect,
                        component: menu.component,
                        href: menu.href,
                        icon: menu.icon,
                        no_cache: menu.no_cache,
                        affix: menu.affix,
                        breadcrumb: menu.breadcrumb,
                        active_menu: menu.active_menu,
                        create_by: menu.create_by,
                        create_time: menu.create_time,
                        update_by: menu.update_by,
                        update_time: menu.update_time,
                        remark: menu.remark,
                        children: vec![],
                    })
                    .collect();
                wrapper.set_success(tree_vo_list);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询菜单树失败: {}", e));
                wrapper
            },
        }
    }

    async fn get_menu(&self) -> HashMap<String, Menu> {
        // TODO: 实现获取所有菜单并构建成HashMap的逻辑
        HashMap::new()
    }

    async fn select_sys_menu_infos(&self, user_id: Option<&str>, user_name: Option<&str>) -> ListWrapper<Menu> {
        match self
            .repository
            .select_menu_list(user_name.map(|s| s.to_string()), user_id.map(|s| s.to_string()))
            .await
        {
            Ok(menus) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(menus);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询菜单信息失败: {}", e));
                wrapper
            },
        }
    }
}
