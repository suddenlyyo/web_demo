//! 菜单参数定义

use serde::{Deserialize, Serialize};

use crate::params::page_param::PageParam;

/// 菜单参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuParam {
    /// 菜单ID
    pub id: Option<String>,
    /// 菜单名称
    pub name: Option<String>,
    /// 父菜单ID
    pub parent_id: Option<String>,
    /// 显示顺序
    pub seq_no: Option<i32>,
    /// 菜单类型（D目录 M菜单 B按钮）
    pub menu_type: Option<String>,
    /// 请求地址
    pub url: Option<String>,
    /// 权限标识
    pub perms: Option<String>,
    /// 菜单状态(0停用 1正常)
    pub status: Option<i32>,
    /// 是否在侧边栏隐藏(0显示 1隐藏)
    pub hidden: Option<i32>,
    /// 是否始终显示根菜单(0隐藏 1显示)
    pub always_show: Option<i32>,
    /// 重定向地址，当设置 noRedirect 的时候该路由在面包屑导航中不可被点击
    pub redirect: Option<String>,
    /// 当前路由外层包裹的组件信息(嵌套路由)
    pub component: Option<String>,
    /// 外部链接地址
    pub href: Option<String>,
    /// 侧边栏中显示的图标
    pub icon: Option<String>,
    /// 不缓存页面(0缓存 1不缓存)
    pub no_cache: Option<i32>,
    /// 页面附加在标签视图中(0不附加 1附加)
    pub affix: Option<i32>,
    /// 该项目将隐藏在breadcrumb中(0隐藏 1显示)
    pub breadcrumb: Option<i32>,
    /// 如果设置路径，侧边栏会突出显示您设置的路径(例: /example/list)
    pub active_menu: Option<String>,
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
