//! 用户角色数据访问层模块

pub mod user_role_repository;
#[cfg(feature = "diesel_impl")]
pub mod diesel_impl;
#[cfg(feature = "seaorm_impl")]
pub mod seaorm_impl;
#[cfg(feature = "sqlx_impl")]
pub mod sqlx_impl;