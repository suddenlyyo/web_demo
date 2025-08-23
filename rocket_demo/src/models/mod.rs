//! 模型模块，包含所有数据实体定义

pub mod dept;
pub mod menu;
pub mod role;
pub mod user;

pub use dept::Dept;
pub use menu::Menu;
pub use role::Role;
pub use user::User;
