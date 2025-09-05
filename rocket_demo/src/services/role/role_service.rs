//! 角色服务接口定义

use crate::models::Role;
use crate::params::role_param::RoleParam;
use common_wrapper::{ListWrapper, ResponseWrapper, SingleWrapper};
use std::collections::HashSet;

/// 角色服务trait
#[rocket::async_trait]
pub trait RoleService {
    /// 角色设置权限
    ///
    /// # 参数
    /// * `role_id` - 角色ID，类型: [&str]
    /// * `menu_ids` - 菜单ID列表，类型: [&[String]]
    ///
    /// # 返回值
    /// 返回响应结果，类型: [ResponseWrapper]
    async fn role_set_menu(&self, role_id: &str, menu_ids: &[String]) -> ResponseWrapper;

    /// 查询角色的菜单id列表
    ///
    /// # 参数
    /// * `role_id` - 角色ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回菜单ID集合，类型: [SingleWrapper<std::collections::HashSet<String>>]
    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> SingleWrapper<std::collections::HashSet<String>>;

    /// 查询角色信息
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [Option<&str>]
    /// * `user_name` - 用户名，类型: [Option<&str>]
    ///
    /// # 返回值
    /// 返回角色列表，类型: [ListWrapper<Role>]
    async fn select_role_infos(&self, user_id: Option<&str>, user_name: Option<&str>) -> ListWrapper<Role>;
}
