//! 角色数据访问层 SeaORM 实现

use sea_orm::sea_query::Order;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

use crate::models::Role;
use crate::repositories::role::role_repository::RoleRepository;
use common_wrapper::PageInfo;

// 导入SeaORM实体
use crate::entities::sys_role;
use crate::entities::sys_role::{Column, Entity};

/// 角色表的所有字段，用于SQL查询
const ROLE_FIELDS: &str = "id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark";

/// 角色数据访问 SeaORM 实现
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
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositorySeaormImpl {
    /// 根据ID获取角色信息
    async fn get_role_by_id(&self, id: &str) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询角色信息
        let role = Entity::find_by_id(id).one(&self.connection).await?;

        match role {
            Some(role) => Ok(role.into()),
            None => Err("Role not found".into()),
        }
    }

    /// 获取角色列表
    async fn list_roles(&self) -> Result<Vec<Role>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询角色列表
        let roles = Entity::find()
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect();

        Ok(roles)
    }

    /// 分页查询角色列表
    async fn list_roles_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Role>, u64, u64), Box<dyn std::error::Error + Send + Sync>> {
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
        let roles: Vec<Role> = paginator
            .fetch_page(current_page - 1)
            .await?
            .into_iter()
            .map(|r| r.into())
            .collect();

        // 获取总数和总页数
        let total_count = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        Ok((roles, total_count, total_pages))
    }

    /// 新增角色
    async fn add_role(&self, role: Role) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 转换为实体
        let role_model: sys_role::ActiveModel = role.into();

        // 使用SeaORM新增角色
        let inserted = role_model.insert(&self.connection).await?;

        Ok(inserted.into())
    }

    /// 修改角色
    async fn update_role(&self, role: Role) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 转换为实体
        let role_model: sys_role::ActiveModel = role.into();

        // 使用SeaORM修改角色
        let updated = role_model.update(&self.connection).await?;

        Ok(updated.into())
    }

    /// 删除角色
    async fn delete_role(&self, id: &str) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 先查询角色信息
        let role = Entity::find_by_id(id).one(&self.connection).await?;

        match role {
            Some(role) => {
                // 使用SeaORM删除角色
                let role_model: sys_role::ActiveModel = role.into();
                role_model.delete(&self.connection).await?;
                Ok(role.into())
            },
            None => Err("Role not found".into()),
        }
    }

    /// 修改角色状态
    async fn update_role_status(&self, id: &str, status: i32) -> Result<Role, Box<dyn std::error::Error + Send + Sync>> {
        // 先查询角色信息
        let role = Entity::find_by_id(id).one(&self.connection).await?;

        match role {
            Some(mut role) => {
                // 更新状态
                role.status = status;

                // 转换为实体并更新
                let role_model: sys_role::ActiveModel = role.into();
                let updated = role_model.update(&self.connection).await?;

                Ok(updated.into())
            },
            None => Err("Role not found".into()),
        }
    }
}
