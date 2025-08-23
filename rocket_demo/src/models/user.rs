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

/// 用户查询参数
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserQuery {
    /// 用户ID
    pub id: Option<String>,
    /// 用户姓名
    pub name: Option<String>,
    /// 部门ID
    pub dept_id: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号码
    pub phone_number: Option<String>,
    /// 性别
    pub sex: Option<String>,
    /// 用户状态
    pub status: Option<i32>,
    /// 备注
    pub remark: Option<String>,
    /// 开始时间
    pub start_date: Option<DateTime<Utc>>,
    /// 结束时间
    pub end_date: Option<DateTime<Utc>>,
    /// 当前页码
    pub current_page_num: Option<u64>,
    /// 每页条数
    pub page_size: Option<u64>,
}

impl From<&User> for UserQuery {
    fn from(user: &User) -> Self {
        UserQuery {
            id: if user.id.is_empty() { None } else { Some(user.id.clone()) },
            name: user.name.clone(),
            dept_id: user.dept_id.clone(),
            email: user.email.clone(),
            phone_number: user.phone_number.clone(),
            sex: user.sex.clone(),
            status: user.status,
            remark: user.remark.clone(),
            // User模型中没有时间范围查询字段，所以设置为None
            start_date: None,
            end_date: None,
            // 分页字段在User模型中不存在，所以设置为None
            current_page_num: None,
            page_size: None,
        }
    }
}

impl From<User> for UserQuery {
    fn from(user: User) -> Self {
        UserQuery {
            id: if user.id.is_empty() { None } else { Some(user.id) },
            name: user.name,
            dept_id: user.dept_id,
            email: user.email,
            phone_number: user.phone_number,
            sex: user.sex,
            status: user.status,
            remark: user.remark,
            // User模型中没有时间范围查询字段，所以设置为None
            start_date: None,
            end_date: None,
            // 分页字段在User模型中不存在，所以设置为None
            current_page_num: None,
            page_size: None,
        }
    }
}
