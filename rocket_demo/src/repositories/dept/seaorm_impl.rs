//! 部门数据访问层 SeaORM 实现

use sea_orm::sea_query::Order;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};

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
    /// 根据ID获取部门信息
    async fn get_dept_by_id(&self, id: &str) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询部门信息
        let dept = Entity::find_by_id(id).one(&self.connection).await?;

        match dept {
            Some(dept) => Ok(dept.into()),
            None => Err("Dept not found".into()),
        }
    }

    /// 获取部门列表
    async fn list_depts(&self) -> Result<Vec<Dept>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用SeaORM查询部门列表
        let depts = Entity::find()
            .order_by(Column::Sort, Order::Asc)
            .all(&self.connection)
            .await?
            .into_iter()
            .map(|d| d.into())
            .collect();

        Ok(depts)
    }

    /// 分页查询部门列表
    async fn list_depts_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Dept>, u64, u64), Box<dyn std::error::Error + Send + Sync>> {
        // 处理分页参数
        let current_page = page_num.unwrap_or(PageInfo::DEFAULT_CURRENT_PAGE);
        let page_size = page_size
            .unwrap_or(PageInfo::DEFAULT_PAGE_SIZE)
            .min(PageInfo::MAX_PAGE_SIZE);

        // 构建分页查询
        let paginator = Entity::find()
            .order_by(Column::Sort, Order::Asc)
            .paginate(&self.connection, page_size);

        // 获取分页数据
        let depts: Vec<Dept> = paginator
            .fetch_page(current_page - 1)
            .await?
            .into_iter()
            .map(|d| d.into())
            .collect();

        // 获取总数和总页数
        let total_count = paginator.num_items().await?;
        let total_pages = paginator.num_pages().await?;

        Ok((depts, total_count, total_pages))
    }

    /// 新增部门
    async fn add_dept(&self, dept: Dept) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 转换为实体
        let dept_model: sys_dept::ActiveModel = dept.into();

        // 使用SeaORM新增部门
        let inserted = dept_model.insert(&self.connection).await?;

        Ok(inserted.into())
    }

    /// 修改部门
    async fn update_dept(&self, dept: Dept) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 转换为实体
        let dept_model: sys_dept::ActiveModel = dept.into();

        // 使用SeaORM修改部门
        let updated = dept_model.update(&self.connection).await?;

        Ok(updated.into())
    }

    /// 删除部门
    async fn delete_dept(&self, id: &str) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 先查询部门信息
        let dept = Entity::find_by_id(id).one(&self.connection).await?;

        match dept {
            Some(dept) => {
                // 使用SeaORM删除部门
                let dept_model: sys_dept::ActiveModel = dept.into();
                dept_model.delete(&self.connection).await?;
                Ok(dept.into())
            },
            None => Err("Dept not found".into()),
        }
    }

    /// 修改部门状态
    async fn update_dept_status(&self, id: &str, status: i32) -> Result<Dept, Box<dyn std::error::Error + Send + Sync>> {
        // 先查询部门信息
        let dept = Entity::find_by_id(id).one(&self.connection).await?;

        match dept {
            Some(mut dept) => {
                // 更新状态
                dept.status = status;

                // 转换为实体并更新
                let dept_model: sys_dept::ActiveModel = dept.into();
                let updated = dept_model.update(&self.connection).await?;

                Ok(updated.into())
            },
            None => Err("Dept not found".into()),
        }
    }
}
