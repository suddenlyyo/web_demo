//! 参数模块
//!
//! 该模块包含了系统中所有请求参数的定义。
//! 参数用于接收和处理HTTP请求中的数据，如查询参数、表单数据、JSON数据等。
//!
//! # 模块组织
//!
//! - [dept_param] - 部门参数，参见: [crate::models::dept]
//! - [page_param] - 分页参数，参见: [crate::repositories]

/// 部门参数
///
/// 用于接收和处理部门相关的请求参数，参见: [crate::models::dept::Dept]
pub mod dept_param;

/// 分页参数
///
/// 用于接收和处理分页相关的请求参数
pub mod page_param;
