//! 角色服务接口定义

use super::PageParam;
use crate::models::Role;
use common_wrapper::{ListWrapper, ResponseWrapper, SingleWrapper};

/// 角色服务接口
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

/// 角色参数
#[derive(Debug, Clone)]
pub struct RoleParam {
    /// 角色ID
    pub id: Option<String>,
    /// 角色名称
    pub name: Option<String>,
    /// 角色权限字符串
    pub role_key: Option<String>,
    /// 显示顺序
    pub seq_no: Option<i32>,
    /// 角色状态（0正常 1停用）
    pub status: Option<i32>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// 备注
    pub remark: Option<String>,
    /// 分页参数
    #[serde(flatten)]
    pub page_param: PageParam,
}
