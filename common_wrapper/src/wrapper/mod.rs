//! # 统一响应包装模块
//!
//! 提供统一的API响应格式，包括单数据、列表数据和分页数据的包装结构

pub mod list_wrapper;
pub mod page_info;
pub mod page_wrapper;
pub mod response_trait;
pub mod response_wrapper;
#[cfg(feature = "rocket_responder")]
pub mod rocket_responder;
pub mod single_wrapper;

pub use list_wrapper::ListWrapper;
pub use page_info::PageInfo;
pub use page_wrapper::PageWrapper;
pub use response_trait::ResponseTrait;
pub use response_wrapper::ResponseWrapper;
pub use single_wrapper::SingleWrapper;
