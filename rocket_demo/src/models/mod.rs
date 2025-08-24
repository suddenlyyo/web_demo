//! 系统所有实体模型模块

pub mod dept;
pub mod menu;
pub mod role;
pub mod role_menu;
pub mod user;
pub mod user_role; // 用户角色模块

pub use dept::Dept;
pub use menu::Menu;
pub use role::Role;
pub use role_menu::RoleMenu;
pub use user::User;
pub use user_role::UserRole;
