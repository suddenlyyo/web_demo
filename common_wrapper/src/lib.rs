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

mod enums;
mod wrapper;
//重新导出 enums、wrapper公共项,方便第三方使用
pub use enums::WrapperErrEnum;
pub use wrapper::{ListWrapper, PageInfo, PageWrapper, ResponseTrait, ResponseWrapper, SingleWrapper};

#[cfg(test)]
mod tests {
    use crate::enums::WrapperErrEnum;
    use crate::wrapper::response_trait::ResponseTrait;
    use crate::wrapper::{ListWrapper, PageWrapper, ResponseWrapper, SingleWrapper};

    #[test]
    fn response_wrapper_test() {
        let success_response = ResponseWrapper::success_default();
        assert_eq!(success_response.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(success_response.get_message(), "Success");

        let mut fail_response = ResponseWrapper::fail_default();
        fail_response.set_fail("Something went wrong");
        assert_eq!(fail_response.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(fail_response.get_message(), "Something went wrong");

        let mut unknown_response = ResponseWrapper::unknown_error_default();
        unknown_response.set_unknown_error("Unknown error occurred");
        assert_eq!(unknown_response.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(unknown_response.get_message(), "Unknown error occurred");
    }

    #[test]
    fn single_wrapper_test() {
        let mut single_wrapper = SingleWrapper::<String>::new();
        single_wrapper.set_success("Test Data".to_string());

        assert_eq!(single_wrapper.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(single_wrapper.get_message(), "Success");
        assert!(single_wrapper.is_success());
        assert_eq!(single_wrapper.get_data(), Some(&"Test Data".to_string()));

        single_wrapper.set_fail("Loading failed");
        assert_eq!(single_wrapper.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(single_wrapper.get_message(), "Loading failed");
        assert!(!single_wrapper.is_success());
        assert_eq!(single_wrapper.get_data(), None);

        single_wrapper.set_unknown_error("Unknown error occurred");
        assert_eq!(single_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(single_wrapper.get_message(), "Unknown error occurred");
        assert!(!single_wrapper.is_success());
        assert_eq!(single_wrapper.get_data(), None);
    }

    #[test]
    fn list_wrapper_test() {
        let mut list_wrapper = ListWrapper::<String>::new();
        list_wrapper.set_success(vec!["item1".to_string(), "item2".to_string()]);

        assert_eq!(list_wrapper.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(list_wrapper.get_message(), "Success");
        assert!(list_wrapper.is_success());
        assert_eq!(list_wrapper.get_data(), &vec!["item1".to_string(), "item2".to_string()]);

        list_wrapper.set_fail("List loading failed");
        assert_eq!(list_wrapper.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(list_wrapper.get_message(), "List loading failed");
        assert!(!list_wrapper.is_success());
        assert_eq!(list_wrapper.get_data(), &Vec::<String>::new());

        list_wrapper.set_unknown_error("Unknown error in list loading");
        assert_eq!(list_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(list_wrapper.get_message(), "Unknown error in list loading");
        assert!(!list_wrapper.is_success());
        assert_eq!(list_wrapper.get_data(), &Vec::<String>::new());
    }

    #[test]
    fn test_page_wrapper() {
        let mut page_wrapper = PageWrapper::new();
        page_wrapper.set_success(vec!["item1", "item2"], 100, 1, 10);

        assert_eq!(page_wrapper.get_code(), WrapperErrEnum::Success as i32);
        assert_eq!(page_wrapper.get_message(), "Success");
        assert!(page_wrapper.is_success());
        assert_eq!(page_wrapper.get_data(), &vec!["item1", "item2"]);
        assert_eq!(page_wrapper.get_total_count(), 100);
        assert_eq!(page_wrapper.get_total_page_count(), 10);
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 10);

        page_wrapper.set_fail("Page loading failed");
        assert_eq!(page_wrapper.get_code(), WrapperErrEnum::Fail as i32);
        assert_eq!(page_wrapper.get_message(), "Page loading failed");
        assert!(!page_wrapper.is_success());
        assert_eq!(page_wrapper.get_data(), &Vec::<&str>::new());
        assert_eq!(page_wrapper.get_total_count(), 0);
        assert_eq!(page_wrapper.get_total_page_count(), 0);
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 0);

        page_wrapper.set_unknown_error("Unknown error in page loading");
        assert_eq!(page_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
        assert_eq!(page_wrapper.get_message(), "Unknown error in page loading");
        assert!(!page_wrapper.is_success());
        assert_eq!(page_wrapper.get_data(), &Vec::<&str>::new());
        assert_eq!(page_wrapper.get_total_count(), 0);
        assert_eq!(page_wrapper.get_total_page_count(), 0);
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 0);
    }
}
