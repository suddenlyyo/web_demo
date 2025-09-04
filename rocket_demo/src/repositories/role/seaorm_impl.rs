//! 角色数据访问层 SeaORM 实现

use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::{Condition, Order};
use sea_orm::{EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

use crate::models::Role;
use crate::repositories::role::role_repository::RoleRepository;
use crate::services::params::user_param::RoleParam;

// 导入SeaORM实体
use crate::entities::sys_role;
use crate::entities::sys_role::{ActiveModel, Column, Entity, Model};

impl From<&Role> for ActiveModel {
    fn from(role: &Role) -> Self {
        ActiveModel {
            id: Set(role.id.clone()),
            name: Set(role.name.clone()),
            role_key: Set(role.role_key.clone()),
            status: Set(role.status),
            seq_no: Set(role.seq_no),
            create_by: Set(role.create_by.clone()),
            create_time: Set(role.create_time.map(|t| t.naive_utc())),
            update_by: Set(role.update_by.clone()),
            update_time: Set(role.update_time.map(|t| t.naive_utc())),
            remark: Set(role.remark.clone()),
        }
    }
}

impl From<Model> for Role {
    fn from(model: Model) -> Self {
        Role {
            id: model.id,
            name: model.name,
            role_key: model.role_key,
            status: model.status,
            seq_no: model.seq_no,
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

/// 角色数据访问 SeaORM 实现
use crate::models::constants::ROLE_FIELDS;
#[derive(Debug)]
pub struct RoleRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

impl RoleRepositorySeaormImpl {
    /// 创建角色仓库 SeaORM 实例
    pub async fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("sqlite://data.db".to_string());
        let connection = sea_orm::Database::connect(&database_url)
            .await
            .expect("Error connecting to SQLite database");

        Self { connection }
    }

    /// 构建查询条件
    fn build_condition(query: &RoleParam) -> Condition {
        let mut condition = Condition::all();

        if let Some(name) = &query.name {
            condition = condition.add(Column::Name.contains(name));
        }

        if let Some(role_key) = &query.role_key {
            condition = condition.add(Column::RoleKey.contains(role_key));
        }

        if let Some(status) = query.status {
            condition = condition.add(Column::Status.eq(status));
        }

        condition
    }
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositorySeaormImpl {
    /// 根据主键删除角色
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;

        if result.rows_affected == 0 {
            return Err(Box::from("角色删除失败"));
        }

        Ok(())
    }

    /// 插入角色记录
    async fn insert(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 选择性插入角色记录
    async fn insert_selective(&self, row: &Role) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 根据主键查询角色
    async fn select_role_by_id(&self, id: &str) -> Result<Option<Role>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(Role::from))
    }

    /// 查询角色列表
    async fn select_role_list(&self, role_param: RoleParam) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>> {
        let condition = Self::build_condition(&role_param);
        let result = Entity::find()
            .filter(condition)
            .order_by(Column::Id, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(result.into_iter().map(Role::from).collect())
    }

    /// 根据主键更新角色
    async fn update_by_id(&self, row: &Role) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键选择性更新角色
    async fn update_by_id_selective(&self, row: &Role) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut model: ActiveModel = row.into();
        // 将主键设置为未修改，因为我们使用它进行查找而不是更新
        model.id = sea_orm::ActiveValue::Unchanged(row.id.clone());
        model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键删除角色
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }
}
