//! 菜单数据访问层 SeaORM 实现

use sea_orm::sea_query::Order;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::models::Menu;
use crate::repositories::menu::menu_repository::MenuRepository;
use common_wrapper::PageInfo;

// 导入SeaORM实体
use crate::entities::sys_menu;
use crate::entities::sys_menu::{Column, Entity};

/// 菜单表的所有字段，用于SQL查询
const MENU_FIELDS: &str = "id, name, menu_type, url, perms, icon, seq_no, status, create_by, create_time, update_by, update_time, remark, parent_id, hidden, always_show, redirect, component, href, no_cache, affix, breadcrumb, active_menu";

/// 菜单数据访问 SeaORM 实现
#[derive(Debug)]
pub struct MenuRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

impl MenuRepositorySeaormImpl {
    /// 创建菜单仓库 SeaORM 实例
    pub async fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite://data.db".to_string());
        let connection = sea_orm::Database::connect(&database_url)
            .await
            .expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl MenuRepository for MenuRepositorySeaormImpl {
    /// 根据ID获取菜单信息
    async fn get_menu_by_id(&self, id: &str) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询菜单信息
        let menu = Entity::find_by_id(id).one(&self.connection).await?;

        match menu {
            Some(menu) => Ok(menu.into()),
            None => Err("Menu not found".into()),
        }
    }

    /// 获取菜单列表
    async fn list_menus(&self) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询菜单列表
        let menus = Entity::find()
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?
            .into_iter()
            .map(|m| m.into())
            .collect();

        Ok(menus)
    }

    /// 分页查询菜单列表
    async fn list_menus_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Menu>, u64, u64), Box<dyn std::error::Error + Send + Sync>> {
        // 处理分页参数
        let current_page = page_num.unwrap_or(PageInfo::DEFAULT_CURRENT_PAGE);
        let page_size = page_size
            .unwrap_or(PageInfo::DEFAULT_PAGE_SIZE)
            .min(PageInfo::MAX_PAGE_SIZE);

        // 构建分页查询
        let paginator = Entity::find()
            .order_by(Column::SeqNo, Order::Asc)
            .paginate(&self.connection, page_size);

        // 获取分页数据
        let menus: Vec<Menu> = paginator
            .fetch_page(current_page - 1)
            .await?
            .into_iter()
            .map(|m| m.into())
            .collect();

        // 获取总数和总页数
        let total_count = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        Ok((menus, total_count, total_pages))
    }

    /// 新增菜单
    async fn add_menu(&self, menu: Menu) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 转换为实体
        let menu_model: sys_menu::ActiveModel = menu.into();

        // 使用SeaORM新增菜单
        let inserted = menu_model.insert(&self.connection).await?;

        Ok(inserted.into())
    }

    /// 修改菜单
    async fn update_menu(&self, menu: Menu) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 转换为实体
        let menu_model: sys_menu::ActiveModel = menu.into();

        // 使用SeaORM修改菜单
        let updated = menu_model.update(&self.connection).await?;

        Ok(updated.into())
    }

    /// 删除菜单
    async fn delete_menu(&self, id: &str) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 先查询菜单信息
        let menu = Entity::find_by_id(id).one(&self.connection).await?;

        match menu {
            Some(menu) => {
                // 使用SeaORM删除菜单
                let menu_model: sys_menu::ActiveModel = menu.into();
                menu_model.delete(&self.connection).await?;
                Ok(menu.into())
            },
            None => Err("Menu not found".into()),
        }
    }

    /// 修改菜单状态
    async fn update_menu_status(&self, id: &str, status: i32) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 先查询菜单信息
        let menu = Entity::find_by_id(id).one(&self.connection).await?;

        match menu {
            Some(mut menu) => {
                // 更新状态
                menu.status = status;

                // 转换为实体并更新
                let menu_model: sys_menu::ActiveModel = menu.into();
                let updated = menu_model.update(&self.connection).await?;

                Ok(updated.into())
            },
            None => Err("Menu not found".into()),
        }
    }
}
