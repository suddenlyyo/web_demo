//! 部门服务实现

use std::collections::HashMap;
use std::sync::Arc;

use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper, SingleWrapper};
use uuid::Uuid;

use crate::{
    models::Dept,
    params::dept_param::DeptParam,
    repositories::dept::dept_repository::DeptRepository,
    services::dept::dept_service::{DeptService, DeptTreeVO},
};

#[cfg(any(feature = "sqlx_impl", feature = "seaorm_impl"))]
use crate::repositories::dept::sqlx_impl::DeptRepositorySqlxImpl;

#[cfg(feature = "diesel_impl")]
use crate::repositories::dept::diesel_impl::DeptRepositoryDieselImpl;

#[cfg(feature = "seaorm_impl")]
use crate::repositories::dept::seaorm_impl::DeptRepositorySeaormImpl;

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
    pub async fn new(_database_url: &str) -> Self {
        #[cfg(feature = "sqlx_impl")]
        let repository = DeptRepositorySqlxImpl::new();

        #[cfg(feature = "diesel_impl")]
        let repository = DeptRepositoryDieselImpl::new(); // Diesel不需要数据库URL

        #[cfg(feature = "seaorm_impl")]
        let repository = DeptRepositorySeaormImpl::new().await.unwrap(); // SeaORM实现

        Self { repository: Arc::new(repository) }
    }

    /// 构建部门树
    fn build_dept_tree(&self, dept_list: Vec<Dept>) -> Vec<DeptTreeVO> {
        // TODO: 实现构建部门树的逻辑
        Vec::new()
    }
}

#[rocket::async_trait]
impl DeptService for DeptServiceImpl {
    async fn select_dept_list(&self, dept_param: DeptParam) -> ListWrapper<Dept> {
        match self.repository.select_dept_list(dept_param).await {
            Ok(dept_list) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(dept_list);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询部门列表失败: {}", e));
                wrapper
            },
        }
    }

    async fn get_dept_tree(&self, dept_param: DeptParam) -> ListWrapper<DeptTreeVO> {
        match self.repository.select_dept_list(dept_param).await {
            Ok(dept_list) => {
                let tree_list = self.build_dept_tree(dept_list);
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(tree_list);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询部门树失败: {}", e));
                wrapper
            },
        }
    }

    async fn add_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
        let dept = Dept {
            id: Uuid::new_v4().to_string(),
            parent_id: dept_param.parent_id,
            name: dept_param.name,
            email: dept_param.email,
            telephone: dept_param.telephone,
            address: dept_param.address,
            logo: dept_param.logo,
            dept_level: dept_param.dept_level,
            seq_no: dept_param.seq_no,
            status: dept_param.status,
            create_by: dept_param.create_by.unwrap_or_default(),
            create_time: Some(chrono::Utc::now()),
            update_by: dept_param.update_by.unwrap_or_default(),
            update_time: Some(chrono::Utc::now()),
            remark: dept_param.remark.unwrap_or_default(),
        };

        match self.repository.insert_selective(&dept).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("添加部门失败: {}", e));
                response
            },
        }
    }

    async fn edit_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
        if let Some(dept_id) = &dept_param.id {
            let dept = Dept {
                id: dept_id.clone(),
                parent_id: dept_param.parent_id,
                name: dept_param.name,
                email: dept_param.email,
                telephone: dept_param.telephone,
                address: dept_param.address,
                logo: dept_param.logo,
                dept_level: dept_param.dept_level,
                seq_no: dept_param.seq_no,
                status: dept_param.status,
                create_by: dept_param.create_by.unwrap_or_default(),
                create_time: None, // 不更新创建时间
                update_by: dept_param.update_by.unwrap_or_default(),
                update_time: Some(chrono::Utc::now()),
                remark: dept_param.remark.unwrap_or_default(),
            };

            match self.repository.update_by_id_selective(&dept).await {
                Ok(_) => ResponseWrapper::success_default(),
                Err(e) => {
                    let mut response = ResponseWrapper::fail_default();
                    response.set_fail(&format!("更新部门失败: {}", e));
                    response
                },
            }
        } else {
            let mut response = ResponseWrapper::fail_default();
            response.set_fail("部门ID不能为空");
            response
        }
    }

    async fn delete_dept(&self, dept_id: &str) -> ResponseWrapper {
        match self.repository.delete_by_id(dept_id).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("删除部门失败: {}", e));
                response
            },
        }
    }
}
