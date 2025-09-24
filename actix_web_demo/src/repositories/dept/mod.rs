//! 部门仓储模块
//!
//! 该模块定义了部门相关的数据访问接口和实现。
//! 提供了对部门信息的增删改查等基本操作。
//!
//! # 主要组件
//! - [dept_repository] - 部门数据访问接口定义，参见: [crate::services::dept]
//! - [diesel_impl] - 基于Diesel ORM的部门数据访问实现（需启用[diesel_impl]特性）
//! - [seaorm_impl] - 基于SeaORM的部门数据访问实现（需启用[seaorm_impl]特性）
//! - [sqlx_impl] - 基于SQLx的部门数据访问实现（需启用[sqlx_impl]特性）

pub mod dept_repository;
#[cfg(feature = "diesel_impl")]
pub mod diesel_impl;
#[cfg(feature = "seaorm_impl")]
pub mod seaorm_impl;
#[cfg(feature = "sqlx_impl")]
pub mod sqlx_impl;
