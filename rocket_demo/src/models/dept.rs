//! 部门实体模型

use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

/// 部门信息实体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx_impl", derive(sqlx::FromRow))]
#[serde(crate = "rocket::serde")]
pub struct Dept {
    /// 部门id
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "id"))]
    pub id: String,
    /// 部门名称
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "name"))]
    pub name: Option<String>,
    /// 邮箱
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "email"))]
    pub email: Option<String>,
    /// 联系电话
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "telephone"))]
    pub telephone: Option<String>,
    /// 地址
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "address"))]
    pub address: Option<String>,
    /// logo地址
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "logo"))]
    pub logo: Option<String>,
    /// 父部门id
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "parent_id"))]
    pub parent_id: Option<String>,
    /// 部门层级
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "dept_level"))]
    pub dept_level: Option<String>,
    /// 显示顺序
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "seq_no"))]
    pub seq_no: Option<i32>,
    /// 部门状态(0正常 1停用)
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "status"))]
    pub status: Option<i32>,
    /// 创建者
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "create_by"))]
    pub create_by: Option<String>,
    /// 创建时间
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "create_time"))]
    pub create_time: Option<NaiveDateTime>,
    /// 更新者
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "update_by"))]
    pub update_by: Option<String>,
    /// 更新时间
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "update_time"))]
    pub update_time: Option<NaiveDateTime>,
    /// 备注
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "remark"))]
    pub remark: Option<String>,
}

impl From<crate::params::dept_param::DeptParam> for Dept {
    fn from(param: crate::params::dept_param::DeptParam) -> Self {
        let mut dept = Dept::default();

        // 手动赋值所有字段
        dept.id = param.id.unwrap_or_default();
        dept.name = param.name;
        dept.email = param.email;
        dept.telephone = param.telephone;
        dept.address = param.address;
        dept.logo = param.logo;
        dept.parent_id = param.parent_id;
        dept.dept_level = param.dept_level;
        dept.seq_no = param.seq_no;
        dept.status = param.status;
        dept.create_by = param.create_by;
        dept.create_time = param.create_time;
        dept.update_by = param.update_by;
        dept.update_time = param.update_time;
        dept.remark = param.remark;

        dept
    }
}
