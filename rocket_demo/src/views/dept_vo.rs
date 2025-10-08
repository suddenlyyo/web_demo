//! 部门详情视图对象定义
//!
//! 该模块定义了用于API响应的部门视图对象结构。

use crate::models::dept::Dept;
use serde::{Deserialize, Serialize};
/// 部门详情视图对象
///
/// 用于部门列表或详情接口的响应数据结构，包含部门基本信息及关联信息
#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeptVO {
    /// 部门基础信息
    ///
    /// 通过flatten展开Dept模型的所有字段
    #[serde(flatten)]
    pub base: Dept,

    /// 状态描述
    ///
    /// 对部门状态的文本描述，便于前端显示
    pub status_desc: Option<String>,

    /// 上级部门名称
    ///
    /// 关联的上级部门名称，避免前端二次查询
    pub parent_name: Option<String>,
}
