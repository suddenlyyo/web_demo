//! 角色菜单数据访问层 SeaORM 实现

use sea_orm::EntityTrait;

use crate::models::RoleMenu;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;

// 导入SeaORM实体
use crate::entities::sys_role_menu;
use crate::entities::sys_role_menu::Entity;

/// 角色菜单数据访问 SeaORM 实现
#[derive(Debug)]
pub struct RoleMenuRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

impl RoleMenuRepositorySeaormImpl {
    /// 创建角色菜单仓库 SeaORM 实例
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
impl RoleMenuRepository for RoleMenuRepositorySeaormImpl {
    /// 根据角色ID和菜单ID删除角色菜单
    async fn delete_by_primary_key(&self, role_id: &str, menu_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id((role_id, menu_id))
            .exec(&self.connection)
            .await?;

        if result.rows_affected == 0 {
            return Err(Box::from("角色菜单删除失败"));
        }

        Ok(())
    }

    /// 插入角色菜单记录
    async fn insert(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_role_menu::ActiveModel = row.into();
        let result = model.insert(&self.connection).await?;

        Ok(())
    }

    /// 选择性插入角色菜单记录
    async fn insert_selective(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.insert(row).await
    }

    /// 根据角色ID查询角色菜单列表
    async fn select_role_menu_by_role_id(&self, role_id: &str) -> Result<Vec<RoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        let role_menus = Entity::find()
            .filter(sys_role_menu::Column::RoleId.eq(role_id))
            .all(&self.connection)
            .await?;

        Ok(role_menus.into_iter().map(|rm| rm.into()).collect())
    }

    /// 批量插入角色菜单
    async fn batch_insert(&self, list: &[RoleMenu]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let models: Vec<sys_role_menu::ActiveModel> = list.iter().map(|rm| rm.into()).collect();
        sys_role_menu::Entity::insert_many(models)
            .exec(&self.connection)
            .await?;

        Ok(())
    }

    /// 根据角色ID和菜单ID列表批量删除角色菜单
    async fn batch_delete_by_role_id_and_menu_ids(&self, role_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        sys_role_menu::Entity::delete_many()
            .filter(sys_role_menu::Column::RoleId.eq(role_id))
            .filter(sys_role_menu::Column::MenuId.is_in(list))
            .exec(&self.connection)
            .await?;

        Ok(())
    }

    /// 根据角色ID删除角色菜单
    async fn delete_by_role_id(&self, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sys_role_menu::Entity::delete_many()
            .filter(sys_role_menu::Column::RoleId.eq(role_id))
            .exec(&self.connection)
            .await?;

        Ok(())
    }
}