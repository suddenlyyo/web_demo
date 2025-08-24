//! 菜单数据访问层 SeaORM 实现

use sea_orm::sea_query::Order;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use common_wrapper::PageInfo;

// 导入SeaORM实体
use crate::entities::sys_menu;
use crate::entities::sys_menu::{Column, Entity};

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
    /// 根据主键删除菜单
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id)
            .exec(&self.connection)
            .await?;

        if result.rows_affected == 0 {
            return Err(Box::from("菜单删除失败"));
        }

        Ok(())
    }

    /// 插入菜单记录
    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_menu::ActiveModel = row.into();
        let result = model.insert(&self.connection).await?;

        Ok(())
    }

    /// 选择性插入菜单记录
    async fn insert_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_menu::ActiveModel = row.into();
        let result = model.insert(&self.connection).await?;

        Ok(())
    }

    /// 根据主键查询菜单
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let menu = Entity::find_by_id(id)
            .one(&self.connection)
            .await?;

        match menu {
            Some(menu) => Ok(Some(menu.into())),
            None => Ok(None),
        }
    }

    /// 根据主键选择性更新菜单
    async fn update_by_primary_key_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_menu::ActiveModel = row.into();
        let result = model.update(&self.connection).await?;

        Ok(())
    }

    /// 根据主键更新菜单
    async fn update_by_primary_key(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_menu::ActiveModel = row.into();
        let result = model.update(&self.connection).await?;

        Ok(())
    }

    /// 根据用户ID查询菜单列表
    async fn select_sys_menu_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        // 这里简化实现，实际应该关联查询用户角色和角色菜单表
        let menus = Entity::find()
            .filter(Column::Status.eq(1))
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(menus.into_iter().map(|m| m.into()).collect())
    }

    /// 查询所有菜单树
    async fn select_menu_tree_all(&self) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let menus = Entity::find()
            .filter(Column::Status.eq(1))
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(menus.into_iter().map(|m| m.into()).collect())
    }

    /// 根据用户ID查询菜单树
    async fn select_menu_tree_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        // 这里简化实现，实际应该关联查询用户角色和角色菜单表
        let menus = Entity::find()
            .filter(Column::Status.eq(1))
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(menus.into_iter().map(|m| m.into()).collect())
    }

    /// 查询菜单列表
    async fn select_sys_menu_list(&self, menu_param: &Menu) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = Entity::find();

        if let Some(name) = &menu_param.name {
            query = query.filter(Column::Name.contains(name));
        }

        if let Some(status) = menu_param.status {
            query = query.filter(Column::Status.eq(status));
        }

        let menus = query
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(menus.into_iter().map(|m| m.into()).collect())
    }

    /// 根据父菜单ID查询子菜单列表
    async fn select_sys_menu_by_parent_id(&self, parent_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let menus = Entity::find()
            .filter(Column::ParentId.eq(parent_id))
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(menus.into_iter().map(|m| m.into()).collect())
    }

    /// 根据角色ID查询菜单ID列表
    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        // 这里简化实现，实际应该关联查询角色菜单表
        let menus = Entity::find()
            .all(&self.connection)
            .await?;

        Ok(menus.into_iter().map(|m| m.into()).collect())
    }
}