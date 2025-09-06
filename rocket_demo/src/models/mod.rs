//! 系统所有实体模型模块

pub mod dept;

pub use dept::Dept;

/// 数据库表字段常量定义
pub mod constants {
    /// 部门表字段
    pub const DEPT_FIELDS: &str = "id, parent_id, name, email, telephone, address, logo, dept_level, seq_no, status, create_by, create_time, update_by, update_time, remark";
}
