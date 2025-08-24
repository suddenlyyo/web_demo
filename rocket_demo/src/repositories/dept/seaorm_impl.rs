//! 部门数据访问层 SeaORM 实现

use sea_orm::sea_query::Order;
use sea_orm::{EntityTrait, QueryFilter, QueryOrder};

use crate::models::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;
use common_wrapper::PageInfo;

// 导入SeaORM实体
use crate::entities::sys_dept;
use crate::entities::sys_dept::{Column, Entity};

/// 部门数据访问 SeaORM 实现
#[derive(Debug)]
pub struct DeptRepositorySeaormImpl {
    connection: sea_orm::DatabaseConnection,
}

impl DeptRepositorySeaormImpl {
    /// 创建部门仓库 SeaORM 实例
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
impl DeptRepository for DeptRepositorySeaormImpl {
    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;

        if result.rows_affected == 0 {
            return Err(Box::from("部门删除失败"));
        }

        Ok(())
    }

    /// 插入部门记录
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_dept::ActiveModel = row.into();
        let result = model.insert(&self.connection).await?;

        Ok(())
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_dept::ActiveModel = row.into();
        let result = model.insert(&self.connection).await?;

        Ok(())
    }

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        let dept = Entity::find_by_id(id).one(&self.connection).await?;

        match dept {
            Some(dept) => Ok(Some(dept.into())),
            None => Ok(None),
        }
    }

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_dept::ActiveModel = row.into();
        let result = model.update(&self.connection).await?;

        Ok(())
    }

    /// 根据主键更新部门
    async fn update_by_primary_key(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: sys_dept::ActiveModel = row.into();
        let result = model.update(&self.connection).await?;

        Ok(())
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = Entity::find();

        if let Some(parent_id) = &row.parent_id {
            query = query.filter(Column::ParentId.eq(parent_id));
        }

        if let Some(name) = &row.name {
            query = query.filter(Column::Name.eq(name));
        }

        if let Some(status) = row.status {
            query = query.filter(Column::Status.eq(status));
        }

        let depts = query
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(depts.into_iter().map(|d| d.into()).collect())
    }

    /// 根据父部门ID查询子部门列表
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        let depts = Entity::find()
            .filter(Column::ParentId.eq(parent_id))
            .order_by(Column::SeqNo, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(depts.into_iter().map(|d| d.into()).collect())
    }
}
