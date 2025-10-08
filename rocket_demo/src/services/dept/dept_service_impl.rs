//! 部门服务实现

use std::collections::HashMap;
use std::sync::Arc;

use chrono::Utc;
use common_wrapper::enums::status_enum::StatusEnum;
use common_wrapper::{ListWrapper, ResponseTrait, ResponseWrapper};
use uuid::Uuid;

use crate::{models::Dept, params::dept_param::DeptParam, repositories::dept::dept_repository::DeptRepository, services::dept::dept_service::DeptService, views::dept_tree::DeptTree, views::dept_vo::DeptVO};

/// 部门服务实现
pub struct DeptServiceImpl {
    repository: Arc<dyn DeptRepository>,
}

impl DeptServiceImpl {
    /// 创建新的部门服务实例
    ///
    /// # 参数
    /// * `repository` - 部门仓库trait的实现
    ///
    /// # 返回值
    /// 返回新的部门服务实例
    pub fn new(repository: Arc<dyn DeptRepository>) -> Self {
        Self { repository }
    }

    /// 构建部门树
    fn build_dept_tree(&self, dept_list: Vec<Dept>) -> Vec<DeptTree> {
        // 如果dept_list为空，直接返回空树
        if dept_list.is_empty() {
            return Vec::new();
        }

        // 转换为树节点列表
        let tree_list: Vec<DeptTree> = dept_list
            .into_iter()
            .map(|dept| DeptTree {
                id: dept.id,
                parent_id: dept.parent_id,
                name: dept.name,
                children: Vec::new(),
            })
            .collect();

        // 创建一个映射，便于通过ID查找节点
        let mut tree_map: HashMap<String, DeptTree> = tree_list
            .into_iter()
            .map(|node| (node.id.clone(), node))
            .collect();

        // 收集所有父子关系
        let mut parent_child_map: HashMap<String, Vec<String>> = HashMap::new();

        for (id, node) in &tree_map {
            if let Some(ref parent_id) = node.parent_id
                && !parent_id.is_empty()
                && tree_map.contains_key(parent_id)
            {
                parent_child_map
                    .entry(parent_id.clone())
                    .or_default()
                    .push(id.clone());
            }
        }

        // 收集所有需要设置子节点的父节点ID
        let parent_ids: Vec<String> = parent_child_map.keys().cloned().collect();

        // 为每个节点设置子节点
        for parent_id in parent_ids {
            if let Some(child_ids) = parent_child_map.get(&parent_id) {
                let children: Vec<DeptTree> = child_ids
                    .iter()
                    .filter_map(|child_id| tree_map.get(child_id).cloned())
                    .collect();

                if let Some(parent_node) = tree_map.get_mut(&parent_id)
                    && !children.is_empty()
                {
                    parent_node.children = children;
                }
            }
        }

        // 只返回根节点（parent_id为None或空字符串的节点）
        tree_map
            .into_values()
            .filter(|node| match &node.parent_id {
                None => true,
                Some(parent_id) => parent_id.is_empty(),
            })
            .collect()
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
    async fn get_dept_tree(&self, dept_param: DeptParam) -> ListWrapper<DeptTree> {
        // 转换参数类型
        let dept = Dept::from(dept_param);
        match self.repository.select_dept_list(&dept).await {
            Ok(dept_list) => {
                let tree_list = self.build_dept_tree(dept_list);
                let mut wrapper = ListWrapper::new();
                wrapper.set_success(tree_list);
                wrapper
            },
            Err(e) => {
                let mut wrapper = ListWrapper::new();
                wrapper.set_fail(format!("查询部门树失败: {}", e));
                wrapper
            },
        }
    }

    async fn get_dept(&self, dept_param: DeptParam) -> HashMap<String, Dept> {
        // 转换参数类型
        let dept = Dept::from(dept_param);
        match self.repository.select_dept_list(&dept).await {
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

    async fn select_dept_list(&self, dept_param: DeptParam) -> ListWrapper<Dept> {
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
                wrapper.set_fail(format!("查询部门列表失败: {}", e));
                wrapper
            },
        }
    }

    async fn select_dept_vo_list(&self, dept_param: DeptParam) -> ListWrapper<DeptVO> {
        // 先获取部门列表
        let dept_result = self.select_dept_list(dept_param).await;

        // 如果没有数据或出现错误，直接返回转换后的结果
        let depts = match dept_result.get_data() {
            Some(data) => data,
            None => return dept_result.map(|_| vec![]), // 转换为VO类型的空列表
        };

        // 获取所有部门信息，用于匹配父部门名称
        let all_depts = self.get_dept(DeptParam::default()).await;

        // 转换为VO列表
        let dept_vos: Vec<DeptVO> = depts
            .iter()
            .map(|dept| {
                // 获取状态描述
                let status_desc = dept
                    .status
                    .and_then(StatusEnum::from_code)
                    .map(|status_enum| status_enum.desc().to_string());

                // 获取父部门名称
                let parent_name = dept.parent_id.as_ref().and_then(|parent_id| {
                    if !parent_id.is_empty() {
                        all_depts
                            .get(parent_id)
                            .and_then(|parent_dept| parent_dept.name.clone())
                    } else {
                        None
                    }
                });

                // 构造DeptVO
                DeptVO { base: dept.clone(), status_desc, parent_name }
            })
            .collect();

        // 创建成功响应
        let mut wrapper = ListWrapper::new();
        wrapper.set_success(dept_vos);
        wrapper
    }

    async fn add_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
        // 验证部门状态
        let status = match self.validate_dept_status(dept_param.status) {
            Ok(s) => s,
            Err(e) => return Self::create_error_response(&e),
        };

        // 如果有父部门ID，验证父级部门
        if let Some(ref parent_id) = dept_param.parent_id
            && let Err(e) = self.validate_parent_dept(parent_id).await
        {
            return Self::create_error_response(&e);
        }

        // 验证部门名称唯一性（新增部门）
        if let Some(ref name) = dept_param.name
            && let Err(e) = self
                .validate_dept_name_unique(dept_param.parent_id.as_ref(), name, true, None)
                .await
        {
            return Self::create_error_response(&e);
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

    async fn edit_dept(&self, dept_param: DeptParam) -> ResponseWrapper {
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
        if let Some(ref parent_id) = dept_param.parent_id
            && let Err(e) = self.validate_parent_dept(parent_id).await
        {
            return Self::create_error_response(&e);
        }

        // 验证部门状态
        let status = match self.validate_dept_status(dept_param.status) {
            Ok(s) => s,
            Err(e) => return Self::create_error_response(&e),
        };

        // 验证部门名称唯一性（编辑部门时需要排除自身）
        if let Some(ref name) = dept_param.name
            && let Err(e) = self
                .validate_dept_name_unique(dept_param.parent_id.as_ref(), name, false, Some(dept_id))
                .await
        {
            return Self::create_error_response(&e);
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

        let dept = Dept {
            id: trimmed_id.to_string(),
            status: Some(status),
            //TODO: 获取当前登录用户
            update_by: Some("system".to_string()),
            update_time: Some(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
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
            Ok(depts) if !depts.is_empty() => return Self::create_error_response("该部门下存在子部门，无法删除!"),
            Ok(_) => (), // 没有子部门，可以删除
            Err(e) => return Self::create_error_response(&format!("查询子部门时发生错误: {}", e)),
        }

        match self.repository.delete_by_primary_key(trimmed_dept_id).await {
            Ok(_) => ResponseWrapper::success_default(),
            Err(e) => Self::create_error_response(&format!("删除部门失败: {}", e)),
        }
    }
}
