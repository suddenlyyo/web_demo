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
//! page_wrapper.set_success(vec!["item1", "item2"], 100, 10, 1, 10);
//! ```

mod enums;
mod wrapper;
//重新导出 enums、wrapper公共项,方便第三方使用
pub use enums::WrapperErrEnum;
pub use wrapper::{ListWrapper, PageInfo, PageWrapper, ResponseTrait, ResponseWrapper, SingleWrapper};

#[cfg(test)]
mod tests {
    use crate::{
        enums::WrapperErrEnum,
        wrapper::{ListWrapper, PageWrapper, ResponseTrait, ResponseWrapper, SingleWrapper},
    };

    #[test]
    fn test_single_wrapper() {
        let mut single_wrapper = SingleWrapper::new();
        single_wrapper.set_success("Hello World");
        assert_eq!(single_wrapper.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(single_wrapper.get_message(), "Success");
        assert!(single_wrapper.is_success());
        assert_eq!(single_wrapper.get_data(), Some(&"Hello World"));

        single_wrapper.set_fail("Something went wrong");
        assert_eq!(single_wrapper.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(single_wrapper.get_message(), "Something went wrong");
        assert!(!single_wrapper.is_success());
        assert_eq!(single_wrapper.get_data(), None);

        single_wrapper.set_unknown_error("Unknown error occurred");
        assert_eq!(single_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(single_wrapper.get_message(), "Unknown error occurred");
        assert!(!single_wrapper.is_success());
        assert_eq!(single_wrapper.get_data(), None);
    }

    #[test]
    fn test_list_wrapper() {
        let mut list_wrapper = ListWrapper::new();
        list_wrapper.set_success(vec!["item1", "item2"]);
        assert_eq!(list_wrapper.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(list_wrapper.get_message(), "Success");
        assert!(list_wrapper.is_success());
        assert_eq!(list_wrapper.get_data(), Some(&vec!["item1", "item2"]));

        list_wrapper.set_fail("List loading failed");
        assert_eq!(list_wrapper.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(list_wrapper.get_message(), "List loading failed");
        assert!(!list_wrapper.is_success());
        assert_eq!(list_wrapper.get_data(), None);

        list_wrapper.set_unknown_error("Unknown error in list loading");
        assert_eq!(list_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(list_wrapper.get_message(), "Unknown error in list loading");
        assert!(!list_wrapper.is_success());
        assert_eq!(list_wrapper.get_data(), None);
    }

    #[test]
    fn test_page_wrapper() {
        let mut page_wrapper = PageWrapper::new();
        page_wrapper.set_success(
            vec!["item1", "item2"],
            100, // total_count
            10,  // total_page
            1,   // current_page_num
            10,  // page_size
        );

        assert_eq!(page_wrapper.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(page_wrapper.get_message(), "Success");
        assert!(page_wrapper.is_success());
        assert_eq!(page_wrapper.get_data(), Some(&vec!["item1", "item2"]));
        assert_eq!(page_wrapper.get_total_count(), 100);
        assert_eq!(page_wrapper.get_total_page(), 10);
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 10);

        page_wrapper.set_fail("Page loading failed");
        assert_eq!(page_wrapper.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(page_wrapper.get_message(), "Page loading failed");
        assert!(!page_wrapper.is_success());
        assert_eq!(page_wrapper.get_data(), None);
        assert_eq!(page_wrapper.get_total_count(), 0);
        assert_eq!(page_wrapper.get_total_page(), 0);
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 0);

        page_wrapper.set_unknown_error("Unknown error in page loading");
        assert_eq!(page_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(page_wrapper.get_message(), "Unknown error in page loading");
        assert!(!page_wrapper.is_success());
        assert_eq!(page_wrapper.get_data(), None);
        assert_eq!(page_wrapper.get_total_count(), 0);
        assert_eq!(page_wrapper.get_total_page(), 0);
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 0);
    }

    #[test]
    fn test_response_wrapper() {
        let success_response = ResponseWrapper::success_default();
        assert_eq!(success_response.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(success_response.get_message(), "Success");

        let mut fail_response = ResponseWrapper::fail_default();
        fail_response.set_fail("Something went wrong");
        assert_eq!(fail_response.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(fail_response.get_message(), "Something went wrong");

        let mut unknown_response = ResponseWrapper::unknown_error_default();
        unknown_response.set_unknown_error("Unknown error");
        assert_eq!(unknown_response.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(unknown_response.get_message(), "Unknown error");
    }
}
