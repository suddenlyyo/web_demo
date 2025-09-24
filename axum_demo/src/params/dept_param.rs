//! 部门参数
//!
//! 该模块定义了部门参数结构，用于接收和处理部门相关的HTTP请求参数。
//! 部门参数包含了部门的基本信息，如部门名称、联系方式、状态等。
//!
//! # 主要功能
//!
//! - 定义部门参数结构，参见: [DeptParam]
//! - 提供部门参数与实体对象之间的转换实现

use crate::params::page_param::PageParam;
use serde::{Deserialize, Serialize};

/// 部门参数
///
/// 用于接收和处理部门相关的HTTP请求参数
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeptParam {
    /// 部门id
    ///
    /// 类型: [Option]<[String]>
    pub id: Option<String>,

    /// 父部门id
    ///
    /// 类型: [Option]<[String]>
    pub parent_id: Option<String>,

    /// 部门名称
    ///
    /// 类型: [Option]<[String]>
    pub name: Option<String>,

    /// 邮箱
    ///
    /// 类型: [Option]<[String]>
    pub email: Option<String>,

    /// 联系电话
    ///
    /// 类型: [Option]<[String]>
    pub telephone: Option<String>,

    /// 地址
    ///
    /// 类型: [Option]<[String]>
    pub address: Option<String>,

    /// logo地址
    ///
    /// 类型: [Option]<[String]>
    pub logo: Option<String>,

    /// 部门层级
    ///
    /// 类型: [Option]<[String]>
    pub dept_level: Option<String>,

    /// 显示顺序
    ///
    /// 类型: [Option]<[i32]>
    pub seq_no: Option<i32>,

    /// 部门状态(0正常 1停用)
    ///
    /// 类型: [Option]<[i32]>
    pub status: Option<i32>,

    /// 创建者
    ///
    /// 类型: [Option]<[String]>
    pub create_by: Option<String>,

    /// 创建时间
    ///
    /// 类型: [Option]<[chrono::NaiveDateTime]>
    pub create_time: Option<chrono::NaiveDateTime>,

    /// 更新者
    ///
    /// 类型: [Option]<[String]>
    pub update_by: Option<String>,

    /// 更新时间
    ///
    /// 类型: [Option]<[chrono::NaiveDateTime]>
    pub update_time: Option<chrono::NaiveDateTime>,

    /// 备注
    ///
    /// 类型: [Option]<[String]>
    pub remark: Option<String>,

    /// 分页参数
    ///
    /// 类型: [PageParam]
    #[serde(flatten)]
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
