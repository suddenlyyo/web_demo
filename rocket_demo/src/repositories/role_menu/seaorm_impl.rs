//! 角色菜单关联数据访问层 SeaORM 实现

// ==================== 数据库连接 ====================
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Order};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::models::SysRoleMenu;
use crate::models::constants::ROLE_MENU_FIELDS;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;
use crate::services::params::user_param::RoleMenuParam;

// 导入SeaORM实体
use crate::entities::sys_role_menu;
use crate::entities::sys_role_menu::{ActiveModel, Column, Entity, Model};

impl From<&SysRoleMenu> for ActiveModel {
    fn from(role_menu: &SysRoleMenu) -> Self {
        ActiveModel {
            id: Set(role_menu.id.clone()),
            role_id: Set(role_menu.role_id.clone()),
            menu_id: Set(role_menu.menu_id.clone()),
        }
    }
}

impl From<Model> for SysRoleMenu {
    fn from(model: Model) -> Self {
        SysRoleMenu {
            id: model.id,
            role_id: model.role_id,
            menu_id: model.menu_id,
        }
    }
}

/// 角色菜单关联数据访问 SeaORM 实现
#[derive(Debug)]
pub struct RoleMenuRepositorySeaormImpl {
    pool: MySqlPool,
}

impl RoleMenuRepositorySeaormImpl {
    /// 创建角色菜单关联仓库 SeaORM 实例
    pub fn new() -> Self {
        let pool = DB_POOL.get().expect("数据库连接池未初始化").clone();
        Self { pool }
    }

    /// 初始化数据库连接池
    pub fn init_pool(pool: MySqlPool) {
        DB_POOL.set(pool).ok(); // 如果已经设置过，则忽略
    }
}

// ==================== 查询条件构建 ====================
impl RoleMenuRepositorySeaormImpl {
    /// 构建查询条件
    fn build_condition(query: &RoleMenuParam) -> Condition {
        let mut condition = Condition::all();

        if let Some(role_id) = &query.role_id {
            condition = condition.add(Column::RoleId.eq(role_id));
        }

        if let Some(menu_id) = &query.menu_id {
            condition = condition.add(Column::MenuId.eq(menu_id));
        }

        condition
    }
}

#[rocket::async_trait]
impl RoleMenuRepository for RoleMenuRepositorySeaormImpl {
    /// 根据主键删除角色菜单关联
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;

        if result.rows_affected == 0 {
            return Err(Box::from("角色菜单关联删除失败"));
        }

        Ok(())
    }

    /// 插入角色菜单关联记录
    async fn insert(&self, row: &SysRoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 选择性插入角色菜单关联记录
    async fn insert_selective(&self, row: &SysRoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 根据主键查询角色菜单关联
    async fn select_by_id(&self, id: &str) -> Result<Option<SysRoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(SysRoleMenu::from))
    }

    /// 查询角色菜单关联列表
    async fn select_list(&self, role_menu_param: RoleMenuParam) -> Result<Vec<SysRoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        let condition = Self::build_condition(&role_menu_param);
        let result = Entity::find()
            .filter(condition)
            .order_by(Column::Id, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(result.into_iter().map(SysRoleMenu::from).collect())
    }

    /// 根据主键更新角色菜单关联
    async fn update_by_id(&self, row: &SysRoleMenu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        let result = Entity::update(model)
            .filter(Column::Id.eq(&row.id))
            .exec(&self.connection)
            .await?;

        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    async fn update_by_id_selective(&self, row: &SysRoleMenu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        let result = Entity::update(model)
            .filter(Column::Id.eq(&row.id))
            .exec(&self.connection)
            .await?;

        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键删除角色菜单关联
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }
}
