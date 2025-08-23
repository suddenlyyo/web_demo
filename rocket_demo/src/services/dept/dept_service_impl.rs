//! 部门服务实现

use std::sync::Arc;

use crate::models::dept::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;
use crate::services::dept::dept_service::DeptService;

// 根据启用的feature导入对应的实现
#[cfg(feature = "sqlx_impl")]
use crate::repositories::dept::sqlx_impl::DeptRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::dept::diesel_impl::DeptRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::dept::seaorm_impl::DeptRepositorySeaormImpl;

use common_wrapper::{ListWrapper, PageWrapper, ResponseTrait, SingleWrapper};

/// 部门服务实现
pub struct DeptServiceImpl {
    repository: Arc<dyn DeptRepository>,
}

impl DeptServiceImpl {
    /// 创建部门服务实例
    ///
    /// # 参数
    ///
    /// - `database_url`: 数据库连接URL
    ///
    /// # 返回值
    ///
    /// 返回新的部门服务实例
    pub async fn new(database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let repository = DeptRepositorySqlxImpl::from_database_url(database_url)
            .await
            .unwrap();

        #[cfg(feature = "diesel_impl")]
        let repository = DeptRepositoryDieselImpl::new(); // Diesel不需要数据库URL

        #[cfg(feature = "seaorm_impl")]
        let repository = DeptRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现

        Self { repository: Arc::new(repository) }
    }
}

#[rocket::async_trait]
impl DeptService for DeptServiceImpl {
    /// 根据ID获取部门信息
    async fn get_dept_by_id(&self, id: &str) -> SingleWrapper<Dept> {
        match self.repository.get_dept_by_id(id).await {
            Ok(dept) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(dept);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("部门不存在");
                wrapper
            },
        }
    }

    /// 获取部门列表
    async fn list_depts(&self) -> ListWrapper<Dept> {
        match self.repository.list_depts().await {
            Ok(depts) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(depts);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取部门列表失败");
                wrapper
            },
        }
    }

    /// 分页查询部门列表
    async fn list_depts_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> PageWrapper<Dept> {
        match self
            .repository
            .list_depts_by_page(page_num, page_size)
            .await
        {
            Ok((depts, total, page_count)) => {
                let mut wrapper = PageWrapper::new();
                let current_page = page_num.unwrap_or(1);
                let page_size_value = page_size.unwrap_or(10);
                wrapper.set_success(depts, total, page_count, current_page, page_size_value);
                wrapper
            },
            Err(_) => {
                let mut wrapper = PageWrapper::new();
                wrapper.set_fail("获取部门列表失败");
                wrapper
            },
        }
    }

    /// 根据父部门ID获取子部门列表
    async fn list_children_by_parent_id(&self, parent_id: &str) -> ListWrapper<Dept> {
        match self.repository.list_children_by_parent_id(parent_id).await {
            Ok(depts) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(depts);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取子部门列表失败");
                wrapper
            },
        }
    }

    /// 获取部门树结构
    async fn list_dept_tree(&self) -> ListWrapper<Dept> {
        match self.repository.list_dept_tree().await {
            Ok(depts) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(depts);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取部门树失败");
                wrapper
            },
        }
    }

    /// 新增部门
    async fn add_dept(&self, dept: Dept) -> SingleWrapper<Dept> {
        match self.repository.add_dept(dept).await {
            Ok(dept) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(dept);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("新增部门失败");
                wrapper
            },
        }
    }

    /// 修改部门
    async fn update_dept(&self, dept: Dept) -> SingleWrapper<Dept> {
        match self.repository.update_dept(dept).await {
            Ok(dept) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(dept);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("修改部门失败");
                wrapper
            },
        }
    }

    /// 删除部门
    async fn delete_dept(&self, id: &str) -> SingleWrapper<Dept> {
        match self.repository.delete_dept(id).await {
            Ok(dept) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(dept);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("删除部门失败");
                wrapper
            },
        }
    }

    /// 修改部门状态
    async fn update_dept_status(&self, id: &str, status: i32) -> SingleWrapper<Dept> {
        match self.repository.update_dept_status(id, status).await {
            Ok(dept) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_success(dept);
                wrapper
            },
            Err(_) => {
                let mut wrapper = SingleWrapper::new();
                wrapper.set_fail("修改部门状态失败");
                wrapper
            },
        }
    }
}
