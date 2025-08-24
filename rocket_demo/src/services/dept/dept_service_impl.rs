//! 部门服务实现

use std::sync::Arc;
use std::collections::HashMap; // 添加HashMap导入

use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper}; // 添加ResponseWrapper
use uuid::Uuid;

use crate::{
    models::{Dept, DeptParam},
    repositories::dept::dept_repository::DeptRepository,
    services::dept::dept_service::{DeptService, TreeVO},
};

use super::DEPT_REPO;

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
    async fn get_dept_tree(&self, dept_param: DeptParam) -> ListWrapper<TreeVO> {
        match self.repository.select_dept_list(dept_param).await {
            Ok(dept_list) => {
                let mut wrapper = ListWrapper::new();
                let tree_list = self.build_tree(dept_list, "0");
                wrapper.set_success(tree_list);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取部门树失败");
                wrapper
            },
        }
    }

    async fn get_dept(&self) -> HashMap<String, Dept> {
        match self.repository.get_dept().await {
            Ok(dept_map) => dept_map,
            Err(_) => HashMap::new(),
        }
    }

    async fn select_dept_list(&self, dept_param: DeptParam) -> ListWrapper<Dept> {
        match self.repository.select_dept_list(dept_param).await {
            Ok(dept_list) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(dept_list);
                wrapper
            },
            Err(_) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail("获取部门列表失败");
                wrapper
            },
        }
    }

    async fn add_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
        let dept = Dept {
            id: Uuid::new_v4().to_string(),
            parent_id: dept_param.parent_id.clone(),
            dept_name: dept_param.dept_name.clone(),
            order_num: dept_param.order_num,
            status: dept_param.status,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        match self.repository.add_dept(dept).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("新增部门成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("新增部门失败");
                wrapper
            },
        }
    }

    async fn edit_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
        match self.repository.edit_dept(dept_param).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("编辑部门成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("编辑部门失败");
                wrapper
            },
        }
    }

    async fn edit_dept_status(&self, id: &str, status: i32) -> ResponseWrapper {
        match self.repository.edit_dept_status(id, status).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("编辑部门状态成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("编辑部门状态失败");
                wrapper
            },
        }
    }

    async fn delete_dept(&self, dept_id: &str) -> ResponseWrapper {
        match self.repository.delete_dept(dept_id).await {
            Ok(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_success("删除部门成功");
                wrapper
            },
            Err(_) => {
                let mut wrapper = ResponseWrapper::new();
                wrapper.set_fail("删除部门失败");
                wrapper
            },
        }
    }
}
