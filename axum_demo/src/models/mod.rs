//! 系统所有实体模型模块
//!
//! 该模块包含了系统中所有实体模型的定义。
//! 实体模型是对数据库表结构的映射，用于在应用程序中表示数据。
//!
//! # 模块组织
//!
//! - [dept] - 部门实体模型，参见: [Dept]

pub mod dept;

pub use dept::Dept;

/// 数据库表字段常量定义
///
/// 该模块包含了数据库表字段的常量定义，用于避免在代码中硬编码字段名
pub mod constants {
    /// 部门表字段
    ///
    /// 部门表的所有字段名，以逗号分隔的字符串形式表示
    pub const DEPT_FIELDS: &str = "id, parent_id, name, email, telephone, address, logo, seq_no, status, create_by, create_time, update_by, update_time, remark";
}
