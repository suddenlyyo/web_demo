//! 用户信息实体

use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize, Serialize};

/// 用户信息实体
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    /// 用户ID
    pub id: String,
    /// 部ID
    pub dept_id: Option<String>,
    /// 用户账号
    pub name: Option<String>,
    /// 用户邮箱
    pub email: Option<String>,
    /// 手机号码
    pub phone_number: Option<String>,
    /// 用户性别(0未知 1男 2女)
    pub sex: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 头像
    pub avatar: Option<String>,
    /// 账号状态(0停用 1正常)
    pub status: Option<i32>,
    /// 最后登录IP
    pub login_ip: Option<String>,
    /// 最后登录时间
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
}
