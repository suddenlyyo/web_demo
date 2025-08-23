use std::sync::Arc;

use crate::models::menu::Menu;
use crate::repositories::menu::menu_repository::MenuRepository;
use crate::repositories::menu::sqlx_impl::MenuRepositorySqlxImpl;
use crate::services::menu::menu_service::MenuService;
use common_wrapper::{ListWrapper, PageWrapper, ResponseTrait, SingleWrapper};

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
    /// 根据ID获取菜单信息
    async fn get_menu_by_id(&self, id: &str) -> SingleWrapper<Menu> {
        match self.repository.get_menu_by_id(id).await {
            Ok(menu) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(menu);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("菜单不存在");
                wrapper
            },
        }
    }

    /// 获取菜单列表
    async fn list_menus(&self) -> ListWrapper<Menu> {
        match self.repository.list_menus().await {
            Ok(menus) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(menus);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取菜单列表失败");
                wrapper
            },
        }
    }

    /// 分页查询菜单列表
    async fn list_menus_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> PageWrapper<Menu> {
        match self
            .repository
            .list_menus_by_page(page_num, page_size)
            .await
        {
            Ok((menus, total, page_count)) => {
                let mut wrapper = PageWrapper::new();
                let current_page = page_num.unwrap_or(1);
                let page_size_value = page_size.unwrap_or(10);
                wrapper.set_success(menus, total, page_count, current_page, page_size_value);
                wrapper
            },
            Err(_) => {
                let mut wrapper = PageWrapper::new();
                wrapper.set_fail("获取菜单列表失败");
                wrapper
            },
        }
    }

    /// 新增菜单
    async fn add_menu(&self, menu: Menu) -> SingleWrapper<Menu> {
        match self.repository.add_menu(menu).await {
            Ok(menu) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(menu);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("新增菜单失败");
                wrapper
            },
        }
    }

    /// 修改菜单
    async fn update_menu(&self, menu: Menu) -> SingleWrapper<Menu> {
        match self.repository.update_menu(menu).await {
            Ok(menu) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(menu);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("修改菜单失败");
                wrapper
            },
        }
    }

    /// 删除菜单
    async fn delete_menu(&self, id: &str) -> SingleWrapper<Menu> {
        match self.repository.delete_menu(id).await {
            Ok(menu) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(menu);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("删除菜单失败");
                wrapper
            },
        }
    }

    /// 修改菜单状态
    async fn update_menu_status(&self, id: &str, status: i32) -> SingleWrapper<Menu> {
        match self.repository.update_menu_status(id, status).await {
            Ok(menu) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(menu);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("更新菜单状态失败");
                wrapper
            },
        }
    }
}
