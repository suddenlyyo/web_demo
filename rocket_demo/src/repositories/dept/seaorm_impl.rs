//! 部门数据访问层 SeaORM 实现

use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::Order;
use sea_orm::{EntityTrait, QueryFilter, QueryOrder};

use crate::models::Dept;
use crate::models::constants::DEPT_FIELDS;
use crate::repositories::dept::dept_repository::DeptRepository;
use common_wrapper::PageInfo;

// 导入SeaORM实体
use crate::entities::sys_dept;
use crate::entities::sys_dept::{ActiveModel, Column, Entity, Model};

impl From<&Dept> for ActiveModel {
    fn from(dept: &Dept) -> Self {
        ActiveModel {
            id: Set(dept.id.clone()),
            parent_id: Set(dept.parent_id.clone()),
            name: Set(dept.name.clone()),
            email: Set(dept.email.clone()),
            telephone: Set(dept.telephone.clone()),
            address: Set(dept.address.clone()),
            logo: Set(dept.logo.clone()),
            dept_level: Set(dept.dept_level.clone()),
            seq_no: Set(dept.seq_no),
            status: Set(dept.status),
            create_by: Set(dept.create_by.clone()),
            create_time: Set(dept.create_time.map(|t| t.naive_utc())),
            update_by: Set(dept.update_by.clone()),
            update_time: Set(dept.update_time.map(|t| t.naive_utc())),
            remark: Set(dept.remark.clone()),
        }
    }
}

impl From<Model> for Dept {
    fn from(model: Model) -> Self {
        Dept {
            id: model.id,
            parent_id: model.parent_id,
            name: model.name,
            email: model.email,
            telephone: model.telephone,
            address: model.address,
            logo: model.logo,
            dept_level: model.dept_level,
            seq_no: model.seq_no,
            status: model.status,
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
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        model.insert(&self.connection).await?;
        Ok(())
    }

    /// 根据主键查询部门
    async fn select_dept_by_id(&self, id: &str) -> Result<Option<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::find_by_id(id).one(&self.connection).await?;
        Ok(result.map(Dept::from))
    }

    /// 查询部门列表
    async fn select_dept_list(&self, dept_param: crate::services::params::user_param::DeptParam) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        let mut query = Entity::find();

        if let Some(name) = dept_param.name {
            query = query.filter(Column::Name.contains(name));
        }

        if let Some(status) = dept_param.status {
            query = query.filter(Column::Status.eq(status));
        }

        let result = query
            .order_by(Column::Id, Order::Asc)
            .all(&self.connection)
            .await?;

        Ok(result.into_iter().map(Dept::from).collect())
    }

    /// 根据主键更新部门
    async fn update_by_id(&self, row: &Dept) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let model: ActiveModel = row.into();
        let result = model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键选择性更新部门
    async fn update_by_id_selective(&self, row: &Dept) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let mut model: ActiveModel = row.into();
        // 将主键设置为未修改，因为我们使用它进行查找而不是更新
        model.id = sea_orm::ActiveValue::Unchanged(row.id.clone());
        let result = model.update(&self.connection).await?;
        Ok(1) // SeaORM更新成功时返回1行受影响
    }

    /// 根据主键删除部门
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let result = Entity::delete_by_id(id).exec(&self.connection).await?;
        Ok(result.rows_affected)
    }
}
