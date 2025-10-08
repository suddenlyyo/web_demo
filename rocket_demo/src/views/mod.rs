//! 视图模块
//!
//! 该模块包含了系统中用于展示的视图模型。
//! 视图模型是对实体模型的封装或转换，用于满足特定的展示需求。
//!
//! # 模块组织
//!
//! - [dept_tree] - 部门树视图模型，参见: [crate::models::dept::Dept]
//! - [dept_vo] - 部门详情视图对象，参见: [crate::models::dept::Dept]

pub mod dept_tree;
pub mod dept_vo;
