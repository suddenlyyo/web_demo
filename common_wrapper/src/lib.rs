//! # Common Wrapper 通用包装库
//!
//! 提供统一的API响应格式包装，包括单数据、列表、分页等不同类型的响应。
//!
//! ## 功能特性
//!
//! - 统一的响应格式
//! - 支持多种数据类型（单数据、列表、分页）
//! - 可自定义响应码和消息
//! - 与serde集成，支持序列化和反序列化
//!
//! ## 模块组织
//!
//! 该库分为两个主要模块：
//! - [enums][]: 包含各种枚举类型，如错误码枚举等
//! - [wrapper][]: 包含各种响应包装器实现
//!
//! ## 使用示例
//!
//! ```rust
//! use common_wrapper::{SingleWrapper, ListWrapper, PageWrapper};
//!
//! // 单数据响应
//! let mut single_wrapper = SingleWrapper::new();
//! single_wrapper.set_success("Hello World");
//!
//! // 列表响应
//! let mut list_wrapper = ListWrapper::new();
//! list_wrapper.set_success(vec!["item1", "item2"]);
//!
//! // 分页响应
//! let mut page_wrapper = PageWrapper::new();
//! page_wrapper.set_success(vec!["item1", "item2"], 100, 1, 10);
//! ```

/// 枚举类型模块
///
/// 包含项目中使用的各种枚举类型，如：
/// - [enums::wrapper_err::WrapperErrEnum]: 响应包装器错误枚举
/// - [enums::gender_enum::GenderEnum]: 性别枚举
/// - [enums::status_enum::StatusEnum]: 状态枚举
pub mod enums;

/// 包装器模块
///
/// 包含各种响应包装器实现，如：
/// - [wrapper::single_wrapper::SingleWrapper]: 单对象包装器
/// - [wrapper::list_wrapper::ListWrapper]: 列表包装器
/// - [wrapper::page_wrapper::PageWrapper]: 分页包装器
/// - [wrapper::page_info::PageInfo]: 分页信息结构体
/// - [wrapper::response_wrapper::ResponseWrapper]: 基础响应包装器
/// - [wrapper::response_trait::ResponseTrait]: 响应trait接口
pub mod wrapper;

// 重新导出常用的类型，方便外部使用
pub use wrapper::list_wrapper::ListWrapper;
pub use wrapper::page_info::PageInfo;
pub use wrapper::page_wrapper::PageWrapper;
pub use wrapper::response_trait::ResponseTrait;
pub use wrapper::response_wrapper::ResponseWrapper;
pub use wrapper::single_wrapper::SingleWrapper;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
