//! 部门实体模型

use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

/// 部门信息实体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Dept {
    /// 部门id
    pub id: String,
    /// 部门名称
    pub name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 联系电话
    pub telephone: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// logo地址
    pub logo: Option<String>,
    /// 父部门id
    pub parent_id: Option<String>,
    /// 部门层级
    pub dept_level: Option<String>,
    /// 显示顺序
    pub seq_no: Option<i32>,
    /// 部门状态(0正常 1停用)
    pub status: Option<i32>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<NaiveDateTime>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<NaiveDateTime>,
    /// 备注
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
