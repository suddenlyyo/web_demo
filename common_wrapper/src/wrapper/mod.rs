//! # 统一响应包装模块
//!
//! 提供统一的API响应格式，包括单数据、列表数据和分页数据的包装结构
//!
//! ## 模块组成
//!
//! - [single_wrapper]: 单对象包装器，用于包装单个对象的响应
//! - [list_wrapper]: 列表包装器，用于包装列表数据的响应
//! - [page_wrapper]: 分页包装器，用于包装分页数据的响应
//! - [page_info]: 分页信息结构体，用于描述分页参数
//! - [response_wrapper]: 基础响应包装器，包含响应码和消息
//! - [response_trait]: 响应trait接口，定义响应包装的公共行为
//! - [rocket_responder]: Rocket框架的响应实现（可选）

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
