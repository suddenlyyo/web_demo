//! 部门实体模型
//!
//! 该模块定义了部门实体结构，用于映射数据库中的部门表。
//! 部门实体包含了部门的基本信息，如部门名称、联系方式、状态等。
//!
//! # 主要功能
//!
//! - 定义部门实体结构，参见: [Dept]
//! - 提供部门实体与参数对象之间的转换实现

use chrono::NaiveDateTime;
#[cfg(feature = "diesel_impl")]
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// 部门信息实体
///
/// 映射数据库部门表的实体结构，包含了部门的所有字段信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx_impl", derive(sqlx::FromRow))]
#[cfg_attr(feature = "diesel_impl", derive(Queryable, Selectable, Insertable, AsChangeset, QueryableByName))]
#[cfg_attr(feature = "diesel_impl", diesel(table_name = crate::schema::sys_dept))]
#[cfg_attr(feature = "diesel_impl", diesel(check_for_backend(diesel::mysql::Mysql)))]
pub struct Dept {
    /// 部门id
    ///
    /// 类型: [String]，数据库字段名: id
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "id"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = id))]
    pub id: String,

    /// 部门名称
    ///
    /// 类型: [Option]<[String]>，数据库字段名: name
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "name"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = name))]
    pub name: Option<String>,

    /// 邮箱
    ///
    /// 类型: [Option]<[String]>，数据库字段名: email
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "email"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = email))]
    pub email: Option<String>,

    /// 联系电话
    ///
    /// 类型: [Option]<[String]>，数据库字段名: telephone
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "telephone"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = telephone))]
    pub telephone: Option<String>,

    /// 地址
    ///
    /// 类型: [Option]<[String]>，数据库字段名: address
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "address"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = address))]
    pub address: Option<String>,

    /// logo地址
    ///
    /// 类型: [Option]<[String]>，数据库字段名: logo
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "logo"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = logo))]
    pub logo: Option<String>,

    /// 父部门id
    ///
    /// 类型: [Option]<[String]>，数据库字段名: parent_id
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "parent_id"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = parent_id))]
    pub parent_id: Option<String>,

    /// 显示顺序
    ///
    /// 类型: [Option]<[i32]>，数据库字段名: seq_no
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "seq_no"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = seq_no))]
    pub seq_no: Option<i32>,

    /// 部门状态(0正常 1停用)
    ///
    /// 类型: [Option]<[i32]>，数据库字段名: status
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "status"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = status))]
    pub status: Option<i32>,

    /// 创建者
    ///
    /// 类型: [Option]<[String]>，数据库字段名: create_by
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "create_by"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = create_by))]
    pub create_by: Option<String>,

    /// 创建时间
    ///
    /// 类型: [Option]<[NaiveDateTime]>，数据库字段名: create_time
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "create_time"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = create_time))]
    pub create_time: Option<NaiveDateTime>,

    /// 更新者
    ///
    /// 类型: [Option]<[String]>，数据库字段名: update_by
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "update_by"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = update_by))]
    pub update_by: Option<String>,

    /// 更新时间
    ///
    /// 类型: [Option]<[NaiveDateTime]>，数据库字段名: update_time
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "update_time"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = update_time))]
    pub update_time: Option<NaiveDateTime>,

    /// 备注
    ///
    /// 类型: [Option]<[String]>，数据库字段名: remark
    #[cfg_attr(feature = "sqlx_impl", sqlx(rename = "remark"))]
    #[cfg_attr(feature = "diesel_impl", diesel(column_name = remark))]
    pub remark: Option<String>,
}

impl From<crate::params::dept_param::DeptParam> for Dept {
    fn from(param: crate::params::dept_param::DeptParam) -> Self {
        // 优化使用Default的方式，避免后续字段重新赋值
        Dept {
            id: param.id.unwrap_or_default(),
            name: param.name,
            email: param.email,
            telephone: param.telephone,
            address: param.address,
            logo: param.logo,
            parent_id: param.parent_id,
            seq_no: param.seq_no,
            status: param.status,
            create_by: param.create_by,
            create_time: param.create_time,
            update_by: param.update_by,
            update_time: param.update_time,
            remark: param.remark,
        }
    }
}
