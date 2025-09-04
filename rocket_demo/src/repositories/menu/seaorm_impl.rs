//! 菜单数据访问层 SeaORM 实现

use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Order};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::models::Menu;
use crate::models::constants::MENU_FIELDS;
use crate::repositories::menu::menu_repository::MenuRepository;
use crate::services::params::user_param::MenuParam;

// 导入SeaORM实体
use crate::entities::sys_menu;
use crate::entities::sys_menu::{ActiveModel, Column, Entity, Model};

impl From<&Menu> for ActiveModel {
    fn from(menu: &Menu) -> Self {
        ActiveModel {
            id: Set(menu.id.clone()),
            menu_name: Set(menu.menu_name.clone()),
            menu_level: Set(menu.menu_level.clone()),
            parent_id: Set(menu.parent_id.clone()),
            seq_no: Set(menu.seq_no),
            path: Set(menu.path.clone()),
            component: Set(menu.component.clone()),
            query: Set(menu.query.clone()),
            is_frame: Set(menu.is_frame),
            is_cache: Set(menu.is_cache),
            menu_type: Set(menu.menu_type.clone()),
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
            parent_id: model.parent_id,
            seq_no: model.seq_no,
            path: model.path,
            component: model.component,
            query: model.query,
            is_frame: model.is_frame,
            is_cache: model.is_cache,
            menu_type: model.menu_type,
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

    /// 构建查询条件
    fn build_condition(query: &MenuParam) -> Condition {
        let mut condition = Condition::all();

        if let Some(menu_name) = &query.menu_name {
            condition = condition.add(Column::MenuName.contains(menu_name));
        }

        if let Some(status) = query.status {
            condition = condition.add(Column::Status.eq(status));
        }

        condition
    }
}

#[rocket::async_trait]
impl MenuRepository for MenuRepositorySeaormImpl {
    /// 根据主键删除菜单
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;

        if result.rows_affected == 0 {
            return Err(Box::from("菜单删除失败"));
        }

        Ok(())
    }

    /// 插入菜单记录
    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 选择性插入菜单记录
    async fn insert_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 根据主键查询菜单
    async fn select_menu_by_id(&self, id: &str) -> Result<Option<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(Menu::from))
    }

    /// 查询菜单列表
    async fn select_menu_list(&self, menu_param: MenuParam) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let condition = Self::build_condition(&menu_param);
        let result = Entity::find()
            .filter(condition)
            .order_by(Column::Id, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(result.into_iter().map(Menu::from).collect())
    }

    /// 根据主键更新菜单
    async fn update_by_id(&self, row: &Menu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键选择性更新菜单
    async fn update_by_id_selective(&self, row: &Menu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut model: ActiveModel = row.into();
        // 将主键设置为未修改，因为我们使用它进行查找而不是更新
        model.id = sea_orm::ActiveValue::Unchanged(row.id.clone());
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键删除菜单
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }
}
