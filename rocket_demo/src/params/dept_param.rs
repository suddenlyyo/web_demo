//! 部门参数

use crate::params::page_param::PageParam;
use serde::{Deserialize, Serialize};

/// 部门参数
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptParam {
    /// 部门id
    pub id: Option<String>,
    /// 父部门id
    pub parent_id: Option<String>,
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
    /// 部门层级
    pub dept_level: Option<String>,
    /// 显示顺序
    pub seq_no: Option<i32>,
    /// 部门状态(0正常 1停用)
    pub status: Option<i32>,
    /// 创建者
    pub create_by: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新者
    pub update_by: Option<String>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
    /// 备注
    pub remark: Option<String>,
    /// 分页参数
    pub page_param: PageParam,
}

impl From<crate::models::dept::Dept> for DeptParam {
    fn from(dept: crate::models::dept::Dept) -> Self {
        DeptParam {
            id: Some(dept.id),
            parent_id: dept.parent_id,
            name: dept.name,
            email: dept.email,
            telephone: dept.telephone,
            address: dept.address,
            logo: dept.logo,
            dept_level: dept.dept_level,
            seq_no: dept.seq_no,
            status: dept.status,
            create_by: dept.create_by,
            create_time: dept.create_time,
            update_by: dept.update_by,
            update_time: dept.update_time,
            remark: dept.remark,
            page_param: PageParam::default(),
        }
    }
}
