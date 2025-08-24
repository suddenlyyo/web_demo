//! 用户参数定义

use crate::services::PageParam;
use chrono::{DateTime, Utc};

/// 用户参数
#[derive(Debug, Clone)]
pub struct UserParam {
    /// 用户ID
    pub id: Option<String>,
    /// 用户名
    pub user_name: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号码
    pub phone_number: Option<String>,
    /// 性别
    pub sex: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 状态
    pub status: Option<i32>,
    /// 登录IP
    pub login_ip: Option<String>,
    /// 登录时间
    pub login_time: Option<DateTime<Utc>>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<DateTime<Utc>>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<DateTime<Utc>>,
    /// 备注
    pub remark: Option<String>,
    /// 部门ID
    pub dept_id: Option<String>,
    /// 用户姓名（用于查询）
    pub name: Option<String>,
    /// 开始时间（用于查询）
    pub start_date: Option<DateTime<Utc>>,
    /// 结束时间（用于查询）
    pub end_date: Option<DateTime<Utc>>,
    /// 分页参数
    #[serde(flatten)]
    pub page_param: PageParam,
}
