//! 部门服务实现

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use chrono::Utc;
use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper, SingleWrapper, StatusEnum};
use uuid::Uuid;

use crate::{
    models::Dept,
    params::dept_param::DeptParam,
    repositories::dept::dept_repository::DeptRepository,
    repositories::dept::dept_repository::DeptRepositoryImpl,
    services::dept::dept_service::{DeptService, DeptTreeVO},
    views::dept_tree::DeptTree,
};

/// 部门服务实现
pub struct DeptServiceImpl {
    repository: Arc<dyn DeptRepository>,
}

impl DeptServiceImpl {
    /// 创建新的部门服务实例
    pub async fn new() -> Self {
        let repository = DeptRepositoryImpl::new(); // 使用默认实现

        Self { repository: Arc::new(repository) }
    }

    /// 构建部门树
    fn build_dept_tree(&self, dept_list: Vec<Dept>) -> Vec<DeptTree> {
        // 如果dept_list为空，直接返回空树
        if dept_list.is_empty() {
            return Vec::new();
        }

        // 先将所有部门转换为树节点并构建HashMap以便快速查找
        let mut dept_map: HashMap<String, DeptTree> = HashMap::with_capacity(dept_list.len());
        for dept in dept_list {
            let tree_node = DeptTree {
                id: dept.id.clone(),
                parent_id: dept.parent_id.clone(),
                name: dept.name.clone(),
                children: Vec::new(), // 默认为空的子节点列表
            };
            dept_map.insert(dept.id, tree_node);
        }

        // 记录所有子节点的ID，这些节点不应该作为根节点出现
        let mut child_ids = HashSet::with_capacity(dept_map.len());

        // 构建父子关系
        for (id, tree_node) in &mut dept_map {
            if let Some(ref parent_id) = tree_node.parent_id {
                // 记录子节点ID
                child_ids.insert(id.clone());

                // 如果父节点存在，则将当前节点添加到父节点的子节点列表中
                if let Some(parent_node) = dept_map.get_mut(parent_id) {
                    parent_node.children.push(tree_node.clone());
                }
            }
        }

        // 收集根节点（parent_id为None且未作为子节点出现的节点）
        let mut result: Vec<DeptTree> = Vec::new();
        for (id, tree_node) in dept_map.into_iter() {
            // 如果节点没有父节点且未作为子节点出现，则为根节点
            if tree_node.parent_id.is_none() && !child_ids.contains(&id) {
                result.push(tree_node);
            }
            // 如果节点有父节点但父节点不存在，也将其作为根节点处理
            else if tree_node.parent_id.is_some() && !child_ids.contains(&id) {
                result.push(tree_node);
            }
        }

        result
    }

    /// 验证父级部门是否存在且状态正常
    async fn validate_parent_dept(&self, parent_id: &str) -> Result<Option<Dept>, String> {
        // 对ID进行去空格处理
        let trimmed_parent_id = parent_id.trim();
        if trimmed_parent_id.is_empty() {
            return Err("父级部门ID不能为空!".to_string());
        }

        match self
            .repository
            .select_by_primary_key(trimmed_parent_id)
            .await
        {
            Ok(Some(parent_dept)) => {
                // 检查父级部门状态，使用StatusEnum枚举进行验证
                if let Some(status) = parent_dept.status {
                    let status_enum = StatusEnum::from_code(status);
                    if let Some(StatusEnum::Disable) = status_enum {
                        return Err("传入的父级部门已停用!".to_string());
                    }
                }
                Ok(Some(parent_dept))
            },
            Ok(None) => Err("传入的父级部门信息不存在!".to_string()),
            Err(e) => Err(format!("查询父级部门时发生错误: {}", e)),
        }
    }

    /// 验证部门名称在同级部门中是否唯一
    ///
    /// # 参数
    /// * `parent_id` - 父部门ID，None表示顶级部门
    /// * `name` - 要验证的部门名称
    /// * `is_add` - 是否为新增操作，true表示新增，false表示编辑
    /// * `dept_id` - 当前部门ID，仅在编辑操作时使用
    ///
    /// # 说明
    /// 在编辑部门时，如果用户只是修改部门的其他属性（如状态、电话等）而不修改部门名称，
    /// 那么在验证部门名称唯一性时，就会发现数据库中已经存在一个同名的部门——就是正在编辑的这个部门本身。
    /// 如果不排除自身，就会错误地报告"已存在相同部门名称!"的错误。
    /// 因此在编辑操作时，需要排除自身。
    ///
    /// # 唯一性规则
    /// 部门名称的唯一性是基于同一父级部门的，即：
    /// 1. 同一父部门下不能有重名的子部门
    /// 2. 不同父部门下的子部门可以有相同的名称
    /// 3. 例如：A分公司和B分公司都可以有名为"财务部"的部门
    ///
    /// # 示例
    /// ```
    /// // 新增部门时验证名称唯一性
    /// validate_dept_name_unique(Some("parent_id"), "财务部", true, None);
    ///
    /// // 编辑部门时验证名称唯一性
    /// validate_dept_name_unique(Some("parent_id"), "财务部", false, Some("current_dept_id"));
    /// ```
    async fn validate_dept_name_unique(&self, parent_id: Option<&String>, name: &str, is_add: bool, dept_id: Option<&String>) -> Result<(), String> {
        // 根据父部门ID查询同级部门列表
        let sibling_depts = match self
            .repository
            .select_dept_list(&Dept { parent_id: parent_id.cloned(), ..Default::default() })
            .await
        {
            Ok(depts) => depts,
            Err(e) => return Err(format!("查询同级部门时发生错误: {}", e)),
        };

        // 检查是否有相同名称的部门（新增操作不需要排除自身，编辑操作需要排除自身）
        for dept in sibling_depts {
            if let Some(dept_name) = &dept.name {
                // is_add为true表示是新增操作，不需要排除自身
                // is_add为false表示是编辑操作，需要排除自身
                if dept_name == name && (is_add || dept.id != *dept_id.unwrap_or(&String::new())) {
                    return Err("已存在相同部门名称!".to_string());
                }
            }
        }

        Ok(())
    }

    /// 验证部门状态是否有效
    fn validate_dept_status(&self, status: Option<i32>) -> Result<i32, String> {
        match status {
            Some(s) => {
                match StatusEnum::from_code(s) {
                    Some(_) => Ok(s), // 状态码有效
                    None => Err("传入的部门状态错误!".to_string()),
                }
            },
            None => Err("部门状态不能为空!".to_string()),
        }
    }

    /// 验证部门是否存在
    async fn validate_dept_exists(&self, dept_id: &str) -> Result<Dept, String> {
        // 对ID进行去空格处理
        let trimmed_dept_id = dept_id.trim();
        if trimmed_dept_id.is_empty() {
            return Err("部门ID不能为空!".to_string());
        }

        match self.repository.select_by_primary_key(trimmed_dept_id).await {
            Ok(Some(dept)) => Ok(dept),
            Ok(None) => Err("传入的部门信息不存在!".to_string()),
            Err(e) => Err(format!("查询部门时发生错误: {}", e)),
        }
    }

    /// 创建失败响应的辅助方法
    fn create_error_response(message: &str) -> ResponseWrapper {
        let mut response = ResponseWrapper::fail_default();
        response.set_fail(message);
        response
    }
}

#[rocket::async_trait]
impl DeptService for DeptServiceImpl {
    async fn get_dept_tree(&self, dept_param: crate::services::dept::dept_service::DeptParam) -> ListWrapper<DeptTree> {
        // 转换参数类型
        let param = DeptParam::from(dept_param);
        match self.repository.select_dept_list(&param).await {
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

    async fn get_dept(&self) -> HashMap<String, Dept> {
        // 创建一个特殊的Dept对象用于查询所有部门
        // 将id设置为空字符串，数据库查询层需要特殊处理这种情况
        match self.repository.select_dept_list(&Dept::default()).await {
            Ok(dept_list) => {
                let mut dept_map = HashMap::with_capacity(dept_list.len());
                for dept in dept_list {
                    dept_map.insert(dept.id.clone(), dept);
                }
                dept_map
            },
            Err(_) => HashMap::new(),
        }
    }

    async fn select_dept_list(&self, dept_param: crate::services::dept::dept_service::DeptParam) -> ListWrapper<Dept> {
        match self
            .repository
            .select_dept_list(&Dept::from(dept_param))
            .await
        {
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

    async fn add_dept(&self, dept_param: crate::services::dept::dept_service::DeptParam) -> ResponseWrapper {
        // 验证部门状态
        let status = match self.validate_dept_status(dept_param.status) {
            Ok(s) => s,
            Err(e) => return Self::create_error_response(&e),
        };

        // 如果有父部门ID，验证父级部门
        if let Some(ref parent_id) = dept_param.parent_id {
            if let Err(e) = self.validate_parent_dept(parent_id).await {
                return Self::create_error_response(&e);
            }
        }

        // 验证部门名称唯一性（新增部门）
        if let Some(ref name) = dept_param.name {
            if let Err(e) = self
                .validate_dept_name_unique(dept_param.parent_id.as_ref(), name, true, None)
                .await
            {
                return Self::create_error_response(&e);
            }
        }

        // 创建部门实体
        let mut dept = Dept::from(dept_param);
        // 设置ID
        dept.id = Uuid::new_v4().to_string();
        // 设置创建者（TODO: 获取当前登录用户）
        dept.create_by = Some("system".to_string());
        dept.create_time = Some(Utc::now().naive_utc());
        // 确保状态正确设置
        dept.status = Some(status);

        match self.repository.insert(&dept).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => Self::create_error_response(&format!("添加部门失败: {}", e)),
        }
    }

    async fn edit_dept(&self, dept_param: crate::services::dept::dept_service::DeptParam) -> ResponseWrapper {
        // 验证部门ID
        let dept_id = match &dept_param.id {
            Some(id) => id,
            None => return Self::create_error_response("部门ID不能为空"),
        };

        // 验证部门是否存在
        let _existing_dept = match self.validate_dept_exists(dept_id).await {
            Ok(dept) => dept,
            Err(e) => return Self::create_error_response(&e),
        };

        // 如果有父部门ID，验证父级部门
        if let Some(ref parent_id) = dept_param.parent_id {
            if let Err(e) = self.validate_parent_dept(parent_id).await {
                return Self::create_error_response(&e);
            }
        }

        // 验证部门状态
        let status = match self.validate_dept_status(dept_param.status) {
            Ok(s) => s,
            Err(e) => return Self::create_error_response(&e),
        };

        // 验证部门名称唯一性（编辑部门时需要排除自身）
        if let Some(ref name) = dept_param.name {
            if let Err(e) = self
                .validate_dept_name_unique(dept_param.parent_id.as_ref(), name, false, Some(dept_id))
                .await
            {
                return Self::create_error_response(&e);
            }
        }

        // 创建部门实体
        let mut dept = Dept::from(dept_param);
        // 设置更新者（TODO: 获取当前登录用户）
        dept.update_by = Some("system".to_string());
        dept.update_time = Some(Utc::now().naive_utc());
        // 确保状态正确设置
        dept.status = Some(status);

        match self.repository.update_by_primary_key_selective(&dept).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => Self::create_error_response(&format!("更新部门失败: {}", e)),
        }
    }

    async fn edit_dept_status(&self, id: &str, status: i32) -> ResponseWrapper {
        // 对ID进行去空格处理
        let trimmed_id = id.trim();
        if trimmed_id.is_empty() {
            return Self::create_error_response("部门ID不能为空");
        }

        // 验证部门状态
        if let Err(e) = self.validate_dept_status(Some(status)) {
            return Self::create_error_response(&e);
        }

        // 验证部门是否存在
        if let Err(e) = self.validate_dept_exists(trimmed_id).await {
            return Self::create_error_response(&e);
        }

        let mut dept = Dept::default();
        dept.id = trimmed_id.to_string();
        dept.status = Some(status);
        //TODO: 获取当前登录用户
        dept.update_by = Some("system".to_string());
        dept.update_time = Some(chrono::Utc::now().naive_utc());
        match self.repository.update_by_primary_key_selective(&dept).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => Self::create_error_response(&format!("更新部门状态失败: {}", e)),
        }
    }

    async fn delete_dept(&self, dept_id: &str) -> ResponseWrapper {
        // 对ID进行去空格处理
        let trimmed_dept_id = dept_id.trim();
        if trimmed_dept_id.is_empty() {
            return Self::create_error_response("部门ID不能为空");
        }

        // 验证部门是否存在
        if let Err(e) = self.validate_dept_exists(trimmed_dept_id).await {
            return Self::create_error_response(&e);
        }

        // 检查是否存在子部门
        match self
            .repository
            .select_dept_by_parent_id(trimmed_dept_id)
            .await
        {
            Ok(Some(_)) => Self::create_error_response("该部门下存在子部门，无法删除!"),
            Ok(None) => (), // 没有子部门，可以删除
            Err(e) => return Self::create_error_response(&format!("查询子部门时发生错误: {}", e)),
        }

        match self.repository.delete_by_primary_key(trimmed_dept_id).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => Self::create_error_response(&format!("删除部门失败: {}", e)),
        }
    }
}
