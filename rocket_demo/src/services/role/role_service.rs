//! 角色服务接口定义

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};

use crate::models::Role;

/// 角色服务trait
#[rocket::async_trait]
pub trait RoleService {
    /// 根据ID获取角色信息
    async fn get_role_by_id(&self, id: &str) -> SingleWrapper<Role>;

    /// 获取角色列表
    async fn list_roles(&self) -> ListWrapper<Role>;

    /// 分页查询角色列表
    async fn list_roles_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> PageWrapper<Role>;

    /// 新增角色
    async fn add_role(&self, role: Role) -> SingleWrapper<Role>;

    /// 修改角色
    async fn update_role(&self, role: Role) -> SingleWrapper<Role>;

    /// 删除角色
    async fn delete_role(&self, id: &str) -> SingleWrapper<Role>;

    /// 修改角色状态
    async fn update_role_status(&self, id: &str, status: i32) -> SingleWrapper<Role>;
}
