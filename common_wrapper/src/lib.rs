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
pub mod enums;

/// 包装器模块
pub mod wrapper;

// 重新导出常用的类型，方便外部使用
pub use wrapper::list_wrapper::ListWrapper;
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
