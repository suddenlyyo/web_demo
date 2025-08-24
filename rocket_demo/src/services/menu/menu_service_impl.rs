use std::collections::HashMap;
use std::sync::Arc;

use common_wrapper::{ListWrapper, ResponseWrapper, SingleWrapper};
use uuid::Uuid;

use crate::{
    models::{Menu, MenuParam},
    repositories::menu::menu_repository::MenuRepository,
    services::menu::menu_service::{MenuService, RouterVO, TreeVO},
};

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::menu::sqlx_impl::MenuRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::menu::diesel_impl::MenuRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::menu::seaorm_impl::MenuRepositorySeaormImpl;

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
    pub async fn new(_database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let repository = MenuRepositorySqlxImpl::new();

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
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询菜单列表失败: {}", e));
                wrapper
            },
        }
    }

    async fn get_menu_by_id(&self, menu_id: &str) -> SingleWrapper<Menu> {
        match self.repository.select_menu_by_id(menu_id).await {
            Ok(Some(menu)) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(menu);
                wrapper
            },
            Ok(None) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("未找到指定菜单");
                wrapper
            },
            Err(e) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail(&format!("查询菜单失败: {}", e));
                wrapper
            },
        }
    }

    async fn add_menu(&self, menu_param: MenuParam) -> ResponseWrapper {
        let mut menu = Menu::default();
        // 从参数构建菜单对象
        menu.parent_id = menu_param.parent_id.unwrap_or_default();
        menu.name = menu_param.menu_name.unwrap_or_default();
        menu.url = menu_param.url.unwrap_or_default();
        menu.menu_type = menu_param.menu_type.unwrap_or('0');
        menu.icon = menu_param.icon.unwrap_or_default();
        menu.seq_no = menu_param.seq_no.unwrap_or(0);
        menu.status = menu_param.status.unwrap_or(0);
        menu.create_by = menu_param.create_by.unwrap_or_default();
        menu.remark = menu_param.remark.unwrap_or_default();

        // 生成唯一ID
        menu.id = Uuid::new_v4().to_string();
        // 设置创建时间
        menu.create_time = chrono::Utc::now().naive_utc();

        match self.repository.insert_selective(&menu).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("添加菜单失败: {}", e));
                response
            },
        }
    }

    async fn update_menu(&self, menu_param: MenuParam) -> ResponseWrapper {
        if let Some(menu_id) = menu_param.menu_id {
            let mut menu = Menu::default();
            menu.id = menu_id;
            menu.parent_id = menu_param.parent_id.unwrap_or_default();
            menu.name = menu_param.menu_name.unwrap_or_default();
            menu.url = menu_param.url.unwrap_or_default();
            menu.menu_type = menu_param.menu_type.unwrap_or('0');
            menu.icon = menu_param.icon.unwrap_or_default();
            menu.seq_no = menu_param.seq_no.unwrap_or(0);
            menu.status = menu_param.status.unwrap_or(0);
            menu.update_by = menu_param.update_by.unwrap_or_default();
            menu.remark = menu_param.remark.unwrap_or_default();

            match self.repository.update_by_id_selective(&menu).await {
                Ok(1) => ResponseWrapper::success_default(),
                Ok(_) => {
                    let mut response = ResponseWrapper::fail_default();
                    response.set_fail("未找到要更新的菜单");
                    response
                },
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

    async fn remove_menu(&self, menu_id: &str) -> ResponseWrapper {
        match self.repository.delete_by_id(menu_id).await {
            Ok(1) => ResponseWrapper::success_default(),
            Ok(_) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail("未找到要删除的菜单");
                response
            },
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("删除菜单失败: {}", e));
                response
            },
        }
    }

    fn build_menu_tree(&self, menus: Vec<Menu>) -> Vec<RouterVO> {
        // 创建一个HashMap来存储所有节点，key为id，value为对应的Menu
        let mut menu_map: HashMap<String, Menu> = HashMap::new();
        // 存储根节点的id
        let mut root_ids: Vec<String> = Vec::new();

        // 第一次遍历，创建所有节点
        for menu in &menus {
            menu_map.insert(menu.id.clone(), menu.clone());
            if menu.parent_id.is_empty() || menu.parent_id == "0" {
                // 根节点
                root_ids.push(menu.id.clone());
            }
        }

        // 递归构建菜单树
        fn build_children(parent_id: &str, menu_map: &HashMap<String, Menu>) -> Vec<RouterVO> {
            let mut children = Vec::new();
            for menu in menu_map.values() {
                if menu.parent_id == parent_id {
                    let child = RouterVO {
                        id: menu.id.clone(),
                        parent_id: menu.parent_id.clone(),
                        name: menu.name.clone(),
                        url: menu.url.clone(),
                        menu_type: menu.menu_type,
                        icon: menu.icon.clone(),
                        seq_no: menu.seq_no,
                        status: menu.status,
                        create_by: menu.create_by.clone(),
                        create_time: menu.create_time,
                        update_by: menu.update_by.clone(),
                        update_time: menu.update_time,
                        remark: menu.remark.clone(),
                        children: build_children(&menu.id, menu_map),
                    };
                    children.push(child);
                }
            }
            children.sort_by_key(|c| c.seq_no);
            children
        }

        // 构建结果
        let mut result: Vec<RouterVO> = Vec::new();
        for id in root_ids {
            if let Some(menu) = menu_map.get(&id) {
                let router_vo = RouterVO {
                    id: menu.id.clone(),
                    parent_id: menu.parent_id.clone(),
                    name: menu.name.clone(),
                    url: menu.url.clone(),
                    menu_type: menu.menu_type,
                    icon: menu.icon.clone(),
                    seq_no: menu.seq_no,
                    status: menu.status,
                    create_by: menu.create_by.clone(),
                    create_time: menu.create_time,
                    update_by: menu.update_by.clone(),
                    update_time: menu.update_time,
                    remark: menu.remark.clone(),
                    children: build_children(&menu.id, &menu_map),
                };
                result.push(router_vo);
            }
        }

        result.sort_by_key(|r| r.seq_no);
        result
    }

    async fn select_menu_tree(&self, menu_param: MenuParam) -> ListWrapper<TreeVO> {
        match self.repository.select_menu_tree(menu_param).await {
            Ok(menu_list) => {
                // 构建菜单树
                let mut menu_map: HashMap<String, TreeVO> = HashMap::new();
                let mut root_ids: Vec<String> = Vec::new();

                // 第一次遍历，创建所有节点
                for menu in &menu_list {
                    let tree_vo = TreeVO {
                        id: menu.id.clone(),
                        parent_id: menu.parent_id.clone(),
                        name: menu.name.clone(),
                        url: menu.url.clone(),
                        menu_type: menu.menu_type,
                        icon: menu.icon.clone(),
                        seq_no: menu.seq_no,
                        status: menu.status,
                        create_by: menu.create_by.clone(),
                        create_time: menu.create_time,
                        update_by: menu.update_by.clone(),
                        update_time: menu.update_time,
                        remark: menu.remark.clone(),
                        children: Vec::new(),
                    };
                    menu_map.insert(menu.id.clone(), tree_vo);
                    if menu.parent_id.is_empty() || menu.parent_id == "0" {
                        // 根节点
                        root_ids.push(menu.id.clone());
                    }
                }

                // 第二次遍历，建立父子关系
                for menu in &menu_list {
                    if menu.parent_id.is_empty() || menu.parent_id == "0" {
                        continue;
                    } else {
                        // 非根节点，将其添加到父节点的children中
                        if let Some(parent) = menu_map.get_mut(&menu.parent_id) {
                            if let Some(child) = menu_map.get(&menu.id) {
                                parent.children.push(child.clone());
                            }
                        }
                    }
                }

                // 构建结果
                let mut result: Vec<TreeVO> = Vec::new();
                for id in root_ids {
                    if let Some(node) = menu_map.get(&id) {
                        result.push(node.clone());
                    }
                }

                let mut wrapper = ListWrapper::new();
                wrapper.set_success(result);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询菜单树失败: {}", e));
                wrapper
            },
        }
    }
}
