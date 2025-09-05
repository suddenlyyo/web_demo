//! 菜单数据访问层 SeaORM 实现

// ==================== 数据库连接 ====================
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Order};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::models::Menu;
use crate::models::constants::MENU_FIELDS;
use crate::repositories::menu::menu_repository::MenuRepository;
use crate::services::params::user_param::MenuParam;

// ==================== 表结构体映射 ====================
// 导入SeaORM实体
use crate::entities::sys_menu;
use crate::entities::sys_menu::{ActiveModel, Column, Entity, Model};

impl From<&Menu> for ActiveModel {
    fn from(menu: &Menu) -> Self {
        ActiveModel {
            id: Set(menu.id.clone()),
            menu_name: Set(menu.menu_name.clone()),
            menu_level: Set(menu.menu_level.clone()),
            menu_type: Set(menu.menu_type.clone()),
            parent_id: Set(menu.parent_id.clone()),
            seq_no: Set(menu.seq_no),
            path: Set(menu.path.clone()),
            component: Set(menu.component.clone()),
            query: Set(menu.query.clone()),
            is_frame: Set(menu.is_frame),
            is_cache: Set(menu.is_cache),
            visible: Set(menu.visible.clone()),
            status: Set(menu.status),
            perms: Set(menu.perms.clone()),
            icon: Set(menu.icon.clone()),
            create_by: Set(menu.create_by.clone()),
            create_time: Set(menu.create_time.map(|t| t.naive_utc())),
            update_by: Set(menu.update_by.clone()),
            update_time: Set(menu.update_time.map(|t| t.naive_utc())),
            remark: Set(menu.remark.clone()),
        }
    }
}

impl From<Model> for Menu {
    fn from(model: Model) -> Self {
        Menu {
            id: model.id,
            menu_name: model.menu_name,
            menu_level: model.menu_level,
            menu_type: model.menu_type,
            parent_id: model.parent_id,
            seq_no: model.seq_no,
            path: model.path,
            component: model.component,
            query: model.query,
            is_frame: model.is_frame,
            is_cache: model.is_cache,
            visible: model.visible,
            status: model.status,
            perms: model.perms,
            icon: model.icon,
            create_by: model.create_by,
            create_time: model
                .create_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: model.update_by,
            update_time: model
                .update_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: model.remark,
        }
    }
}

/// 菜单数据访问 SeaORM 实现
#[derive(Debug)]
pub struct MenuRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

// ==================== SQL trait 实现 ====================
impl MenuRepositorySeaormImpl {
    /// 创建菜单仓库 SeaORM 实例
    pub async fn new() -> Self {
        // 初始化数据库连接
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() {
            config.database.url
        } else {
            panic!("无法从配置文件获取数据库连接信息");
        };

        let connection = sea_orm::Database::connect(&database_url)
            .await
            .expect("Error connecting to MySQL database");

        Self { connection }
    }

    /// 构建查询条件
    fn build_query_condition(menu_param: &MenuParam) -> Condition {
        let mut condition = Condition::all();

        if let Some(status) = menu_param.status {
            condition = condition.add(Column::Status.eq(status));
        }

        if let Some(menu_name) = &menu_param.menu_name {
            condition = condition.add(Column::MenuName.like(format!("%{}%", menu_name)));
        }

        condition
    }
}

#[rocket::async_trait]
impl MenuRepository for MenuRepositorySeaormImpl {
    async fn select_menu_by_id(&self, id: &str) -> Result<Option<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(Menu::from))
    }

    async fn select_menu_list(&self, menu_param: MenuParam) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let condition = Self::build_condition(&menu_param);
        let result = Entity::find()
            .filter(condition)
            .order_by(Column::Id, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(result.into_iter().map(Menu::from).collect())
    }

    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    async fn update_by_id(&self, row: &Menu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        let result = Entity::update(model)
            .filter(Column::Id.eq(&row.id))
            .exec(&self.connection)
            .await?;

        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }
}
