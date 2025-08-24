//! 角色菜单数据访问层模块

#[cfg(feature = "diesel_impl")]
pub mod diesel_impl;
pub mod role_menu_repository;
#[cfg(feature = "seaorm_impl")]
pub mod seaorm_impl;
#[cfg(feature = "sqlx_impl")]
pub mod sqlx_impl;
