//! 菜单实体模型

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

/// 菜单信息实体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Menu {
    /// 菜单ID
    pub id: String,
    /// 菜单名称
    #[serde()]
    pub name: Option<String>,
    /// 父菜单ID
    #[serde()]
    pub parent_id: Option<String>,
    /// 显示顺序
    #[serde()]
    pub seq_no: Option<i32>,
    /// 菜单类型（D目录 M菜单 B按钮）
    #[serde()]
    pub menu_type: Option<String>,
    /// 请求地址
    #[serde()]
    pub url: Option<String>,
    /// 权限标识
    #[serde()]
    pub perms: Option<String>,
    /// 菜单状态(0停用 1正常)
    #[serde()]
    pub status: Option<i32>,
    /// 是否在侧边栏隐藏(0显示 1隐藏)
    #[serde()]
    pub hidden: Option<i32>,
    /// 是否始终显示根菜单(0隐藏 1显示)
    #[serde()]
    pub always_show: Option<i32>,
    /// 重定向地址，当设置 noRedirect 的时候该路由在面包屑导航中不可被点击
    #[serde()]
    pub redirect: Option<String>,
    /// 当前路由外层包裹的组件信息(嵌套路由)
    #[serde()]
    pub component: Option<String>,
    /// 外部链接地址
    #[serde()]
    pub href: Option<String>,
    /// 侧边栏中显示的图标
    #[serde()]
    pub icon: Option<String>,
    /// 不缓存页面(0缓存 1不缓存)
    #[serde()]
    pub no_cache: Option<i32>,
    /// 页面附加在标签视图中(0不附加 1附加)
    #[serde()]
    pub affix: Option<i32>,
    /// 该项目将隐藏在breadcrumb中(0隐藏 1显示)
    #[serde()]
    pub breadcrumb: Option<i32>,
    /// 如果设置路径，侧边栏会突出显示您设置的路径(例: /example/list)
    #[serde()]
    pub active_menu: Option<String>,
    /// 创建者
    #[serde()]
    pub create_by: Option<String>,
    /// 创建时间
    #[serde()]
    pub create_time: Option<DateTime<Utc>>,
    /// 更新者
    #[serde()]
    pub update_by: Option<String>,
    /// 更新时间
    #[serde()]
    pub update_time: Option<DateTime<Utc>>,
    /// 备注
    #[serde()]
    pub remark: Option<String>,
}
