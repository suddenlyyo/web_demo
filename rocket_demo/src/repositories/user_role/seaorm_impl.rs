//! 用户角色关联数据访问层 SeaORM 实现

// ==================== 数据库连接 ====================
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Order};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

// 导入SeaORM实体
use crate::entities::sys_user_role;
use crate::entities::sys_user_role::{ActiveModel, Column, Entity, Model};

// ==================== 业务模型导入 ====================
use crate::models::SysUserRole;
use crate::models::constants::USER_ROLE_FIELDS;
use crate::repositories::user_role::user_role_repository::UserRoleRepository;
use crate::services::params::user_param::UserRoleParam;

// ==================== 表结构体映射 ====================
impl From<&SysUserRole> for ActiveModel {
    fn from(user_role: &SysUserRole) -> Self {
        ActiveModel {
            id: Set(user_role.id.clone()),
            user_id: Set(user_role.user_id.clone()),
            role_id: Set(user_role.role_id.clone()),
        }
    }
}

impl From<Model> for SysUserRole {
    fn from(model: Model) -> Self {
        SysUserRole {
            id: model.id,
            user_id: model.user_id,
            role_id: model.role_id,
        }
    }
}

/// 用户角色关联数据访问 SeaORM 实现
// ==================== SQL trait 实现 ====================
#[derive(Debug)]
pub struct UserRoleRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

impl UserRoleRepositorySeaormImpl {
    /// 创建用户角色关联仓库 SeaORM 实例
    pub async fn new() -> Self {
        // 初始化数据库连接
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() {
            config.database.url
        } else {
            // 示例数据库连接URL
            "mysql://root:密码@localhost:3306/数据库名".to_string()
        };

        let connection = sea_orm::Database::connect(&database_url)
            .await
            .expect("无法连接到MySQL数据库");

        Self { connection }
    }

    /// 构建查询条件
    fn build_query_condition(user_role_param: &UserRoleParam) -> Condition {
        let mut condition = Condition::all();

        if let Some(user_id) = &user_role_param.user_id {
            condition = condition.add(Column::UserId.eq(user_id));
        }

        if let Some(role_id) = &user_role_param.role_id {
            condition = condition.add(Column::RoleId.eq(role_id));
        }

        condition
    }
}

#[rocket::async_trait]
impl UserRoleRepository for UserRoleRepositorySeaormImpl {
    /// 根据主键删除用户角色关联
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;

        if result.rows_affected == 0 {
            return Err(Box::from("用户角色关联删除失败"));
        }

        Ok(())
    }

    /// 插入用户角色关联记录
    async fn insert(&self, row: &SysUserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 选择性插入用户角色关联记录
    async fn insert_selective(&self, row: &SysUserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 根据ID查询用户角色关联
    async fn select_by_id(&self, id: &str) -> Result<Option<SysUserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(SysUserRole::from))
    }

    /// 查询用户角色关联列表
    async fn select_list(&self, user_role_param: UserRoleParam) -> Result<Vec<SysUserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let condition = Self::build_condition(&user_role_param);
        let result = Entity::find()
            .filter(condition)
            .order_by(Column::Id, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(result.into_iter().map(SysUserRole::from).collect())
    }

    /// 根据ID更新用户角色关联
    async fn update_by_id(&self, row: &SysUserRole) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功返回1行受影响
    }

    /// 选择性根据ID更新用户角色关联
    async fn update_by_id_selective(&self, row: &SysUserRole) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut model: ActiveModel = row.into();
        // 设置主键为未更改，因为我们使用它进行查找而不是更新
        model.id = sea_orm::ActiveValue::Unchanged(row.id.clone());
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功返回1行受影响
    }

    /// 根据ID删除用户角色关联
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }
}
