//! 菜单服务接口定义

use common_wrapper::{ListWrapper, ResponseWrapper};
use crate::models::Menu;
use std::collections::HashMap;
use super::PageParam;

/// 菜单服务trait
#[rocket::async_trait]
pub trait MenuService {
    /// 根据用户ID查询菜单树信息
    async fn select_menu_tree_by_user_id(&self, user_id: &str) -> Vec<RouterVO>;

    /// 查询菜单列表
    async fn select_menu_list(&self, menu_param: MenuParam) -> ListWrapper<Menu>;

    /// 新增菜单
    async fn add_menu(&self, menu_param: MenuParam) -> ResponseWrapper;

    /// 编辑菜单
    async fn edit_menu(&self, menu_param: MenuParam) -> ResponseWrapper;

    /// 修改菜单状态
    async fn edit_menu_status(&self, id: &str, status: i32) -> ResponseWrapper;

    /// 删除菜单
    async fn delete_menu(&self, menu_id: &str) -> ResponseWrapper;

    /// 获取菜单树
    async fn get_menu_tree(&self, menu_param: MenuParam) -> ListWrapper<TreeVO>;

    /// 获取菜单信息Map 用于菜单信息匹配
    async fn get_menu(&self) -> HashMap<String, Menu>;

    /// 查询菜单信息
    async fn select_sys_menu_infos(&self, user_id: Option<&str>, user_name: Option<&str>) -> ListWrapper<Menu>;
}

/// 菜单参数
#[derive(Debug, Clone)]
pub struct MenuParam {
    pub id: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub seq_no: Option<i32>,
    pub menu_type: Option<String>,
    pub url: Option<String>,
    pub perms: Option<String>,
    pub status: Option<i32>,
    pub hidden: Option<i32>,
    pub always_show: Option<i32>,
    pub redirect: Option<String>,
    pub component: Option<String>,
    pub href: Option<String>,
    pub icon: Option<String>,
    pub no_cache: Option<i32>,
    pub affix: Option<i32>,
    pub breadcrumb: Option<i32>,
    pub active_menu: Option<String>,
    pub create_by: Option<String>,
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub update_by: Option<String>,
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    pub remark: Option<String>,
}

/// 路由VO
#[derive(Debug, Clone)]
pub struct RouterVO {
    // 根据实际需要定义字段
}

/// 树形结构VO
#[derive(Debug, Clone)]
pub struct TreeVO {
    // 根据实际需要定义字段
}