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

/// 数据库表字段常量定义
pub mod constants {
    /// 部门表字段
    pub const DEPT_FIELDS: &str = "id, parent_id, name, email, telephone, address, logo, dept_level, seq_no, status, create_by, create_time, update_by, update_time, remark";

    /// 用户表字段
    pub const USER_FIELDS: &str = "id, dept_id, name, email, phone_number, sex, password, avatar, status, login_ip, login_time, create_by, create_time, update_by, update_time, remark";

    /// 角色表字段
    pub const ROLE_FIELDS: &str = "id, name, role_key, status, seq_no, create_by, create_time, update_by, update_time, remark";

    /// 菜单表字段
    pub const MENU_FIELDS: &str = "id, menu_name, menu_level, parent_id, seq_no, path, component, query, is_frame, is_cache, menu_type, visible, status, perms, icon, create_by, create_time, update_by, update_time, remark";

    /// 用户角色关联表字段
    pub const USER_ROLE_FIELDS: &str = "id, user_id, role_id";

    /// 角色菜单关联表字段
    pub const ROLE_MENU_FIELDS: &str = "id, role_id, menu_id";
}
