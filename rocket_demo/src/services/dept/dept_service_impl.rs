//! 部门服务实现

use std::collections::HashMap;
use std::sync::Arc;

use chrono;
use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper, SingleWrapper};
use uuid::Uuid;

use crate::{
    models::Dept,
    repositories::dept::dept_repository::DeptRepository,
    services::dept::dept_service::{DeptParam, DeptService, TreeVO},
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
}

#[rocket::async_trait]
impl DeptService for DeptServiceImpl {
    async fn get_dept_tree(&self, _dept_param: DeptParam) -> ListWrapper<TreeVO> {
        // 这里应该使用dept_param做参数过滤，为了简单示例使用默认值
        match self.repository.select_dept_list(&Dept::default()).await {
            Ok(dept_list) => {
                let mut wrapper = ListWrapper::new();
                let tree = self.build_tree(dept_list);
                wrapper.set_success(tree);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(&format!("查询部门列表失败: {}", e));
                wrapper
            },
        }
    }

    async fn add_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
        let mut dept = Dept::default();
        // 从参数构建部门对象
        dept.parent_id = dept_param.parent_id.unwrap_or_default();
        dept.name = dept_param.dept_name.unwrap_or_default();
        dept.email = dept_param.email.unwrap_or_default();
        dept.telephone = dept_param.telephone.unwrap_or_default();
        dept.address = dept_param.address.unwrap_or_default();
        dept.logo = dept_param.logo.unwrap_or_default();
        dept.dept_level = dept_param.dept_level.unwrap_or(0);
        dept.seq_no = dept_param.seq_no.unwrap_or(0);
        dept.status = dept_param.status.unwrap_or(0);
        dept.create_by = dept_param.create_by.unwrap_or_default();
        dept.remark = dept_param.remark.unwrap_or_default();

        // 生成唯一ID
        dept.id = Uuid::new_v4().to_string();
        // 设置创建时间
        dept.create_time = chrono::Utc::now().naive_utc();

        match self.repository.insert_selective(&dept).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("添加部门失败: {}", e));
                response
            },
        }
    }

    async fn update_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
        if let Some(dept_id) = dept_param.dept_id {
            let mut dept = Dept::default();
            dept.id = dept_id;
            dept.parent_id = dept_param.parent_id.unwrap_or_default();
            dept.name = dept_param.dept_name.unwrap_or_default();
            dept.email = dept_param.email.unwrap_or_default();
            dept.telephone = dept_param.telephone.unwrap_or_default();
            dept.address = dept_param.address.unwrap_or_default();
            dept.logo = dept_param.logo.unwrap_or_default();
            dept.dept_level = dept_param.dept_level.unwrap_or(0);
            dept.seq_no = dept_param.seq_no.unwrap_or(0);
            dept.status = dept_param.status.unwrap_or(0);
            dept.update_by = dept_param.update_by.unwrap_or_default();
            dept.remark = dept_param.remark.unwrap_or_default();

            match self.repository.update_by_id_selective(&dept).await {
                Ok(1) => ResponseWrapper::success_default(),
                Ok(_) => {
                    let mut response = ResponseWrapper::fail_default();
                    response.set_fail("未找到要更新的部门");
                    response
                },
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

    async fn remove_dept(&self, dept_id: &str) -> ResponseWrapper {
        match self.repository.delete_by_id(dept_id).await {
            Ok(1) => ResponseWrapper::success_default(),
            Ok(_) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail("未找到要删除的部门");
                response
            },
            Err(e) => {
                let mut response = ResponseWrapper::fail_default();
                response.set_fail(&format!("删除部门失败: {}", e));
                response
            },
        }
    }

    /// 构建部门树
    fn build_tree(&self, depts: Vec<Dept>) -> Vec<TreeVO> {
        // 创建一个HashMap来存储所有节点，key为id，value为对应的TreeVO
        let mut node_map: HashMap<String, TreeVO> = HashMap::new();
        // 存储根节点的id
        let mut root_ids: Vec<String> = Vec::new();

        // 第一次遍历，创建所有节点
        for dept in &depts {
            let tree_vo = TreeVO {
                id: dept.id.clone(),
                parent_id: dept.parent_id.clone(),
                name: dept.name.clone(),
                email: dept.email.clone(),
                telephone: dept.telephone.clone(),
                address: dept.address.clone(),
                logo: dept.logo.clone(),
                dept_level: dept.dept_level.clone(),
                seq_no: dept.seq_no,
                status: dept.status,
                create_by: dept.create_by.clone(),
                create_time: dept.create_time,
                update_by: dept.update_by.clone(),
                update_time: dept.update_time,
                remark: dept.remark.clone(),
                children: Vec::new(),
            };
            node_map.insert(dept.id.clone(), tree_vo);
        }

        // 第二次遍历，建立父子关系
        for dept in &depts {
            if dept.parent_id.is_empty() || dept.parent_id == "0" {
                // 根节点
                root_ids.push(dept.id.clone());
            } else {
                // 非根节点，将其添加到父节点的children中
                if let Some(parent) = node_map.get_mut(&dept.parent_id) {
                    if let Some(child) = node_map.get(&dept.id) {
                        parent.children.push(child.clone());
                    }
                }
            }
        }

        // 构建结果
        let mut result: Vec<TreeVO> = Vec::new();
        for id in root_ids {
            if let Some(node) = node_map.get(&id) {
                result.push(node.clone());
            }
        }

        result
    }
}
