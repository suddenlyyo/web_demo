//! 集成测试文件，用于测试common_wrapper模块的所有功能
//!
//! 本测试文件验证了所有包装器（SingleWrapper、ListWrapper、PageWrapper）的功能，
//! 包括成功、失败和未知错误状态的处理。

use common_wrapper::{ListWrapper, PageWrapper, ResponseTrait, SingleWrapper, WrapperErrEnum};

/// 测试SingleWrapper的基本功能
#[test]
fn test_single_wrapper() {
    // 测试成功状态
    let mut single_wrapper = SingleWrapper::new();
    single_wrapper.set_success("Hello World");

    assert_eq!(single_wrapper.get_code(), WrapperErrEnum::Success as i32);
    assert_eq!(single_wrapper.get_message(), "Success");
    assert!(single_wrapper.is_success());
    assert_eq!(single_wrapper.get_data(), Some(&"Hello World"));

    // 测试失败状态
    single_wrapper.set_fail("Something went wrong");
    assert_eq!(single_wrapper.get_code(), WrapperErrEnum::Fail as i32);
    assert_eq!(single_wrapper.get_message(), "Something went wrong");
    assert!(!single_wrapper.is_success());
    assert_eq!(single_wrapper.get_data(), None);

    // 测试未知错误状态
    single_wrapper.set_unknown_error("Unknown error occurred");
    assert_eq!(single_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
    assert_eq!(single_wrapper.get_message(), "Unknown error occurred");
    assert!(!single_wrapper.is_success());
    assert_eq!(single_wrapper.get_data(), None);
}

/// 测试ListWrapper的基本功能
#[test]
fn test_list_wrapper() {
    // 测试成功状态
    let mut list_wrapper = ListWrapper::new();
    list_wrapper.set_success(vec!["item1", "item2", "item3"]);

    assert_eq!(list_wrapper.get_code(), WrapperErrEnum::Success as i32);
    assert_eq!(list_wrapper.get_message(), "Success");
    assert!(list_wrapper.is_success());
    assert_eq!(list_wrapper.get_data(), Some(&vec!["item1", "item2", "item3"]));

    // 测试失败状态
    list_wrapper.set_fail("List operation failed");
    assert_eq!(list_wrapper.get_code(), WrapperErrEnum::Fail as i32);
    assert_eq!(list_wrapper.get_message(), "List operation failed");
    assert!(!list_wrapper.is_success());
    assert_eq!(list_wrapper.get_data(), None);

    // 测试未知错误状态
    list_wrapper.set_unknown_error("Unknown error in list operation");
    assert_eq!(list_wrapper.get_code(), WrapperErrEnum::UnknownError as i32);
    assert_eq!(list_wrapper.get_message(), "Unknown error in list operation");
    assert!(!list_wrapper.is_success());
    assert_eq!(list_wrapper.get_data(), None);
}

/// 测试PageWrapper的基本功能
#[test]
fn test_page_wrapper() {
    // 为错误状态创建断言辅助函数
    fn assert_error_state(wrapper: &PageWrapper<&str>, err_code: WrapperErrEnum, err_message: &str) {
        assert_eq!(wrapper.get_code(), err_code as i32);
        assert_eq!(wrapper.get_message(), err_message);
        assert!(!wrapper.is_success());
        assert_eq!(wrapper.get_data(), None);
        assert_eq!(wrapper.get_total_count(), 0);
        assert_eq!(wrapper.get_total_page_count(), 0);
        assert_eq!(wrapper.get_current_page_num(), 0);
        assert_eq!(wrapper.get_page_size(), 0);
    }

    // 测试成功状态
    let mut page_wrapper = PageWrapper::new();
    page_wrapper.set_success(vec!["item1", "item2"], 100, 1, 10);

    assert_eq!(page_wrapper.get_code(), WrapperErrEnum::Success as i32);
    assert_eq!(page_wrapper.get_message(), "Success");
    assert!(page_wrapper.is_success());
    assert_eq!(page_wrapper.get_data(), Some(&vec!["item1", "item2"]));
    assert_eq!(page_wrapper.get_total_count(), 100);
    assert_eq!(page_wrapper.get_total_page_count(), 10);
    assert_eq!(page_wrapper.get_current_page_num(), 1);
    assert_eq!(page_wrapper.get_page_size(), 10);

    // 测试失败状态
    page_wrapper.set_fail("Page loading failed");
    assert_error_state(&page_wrapper, WrapperErrEnum::Fail, "Page loading failed");

    // 测试未知错误状态
    page_wrapper.set_unknown_error("Unknown error in page loading");
    assert_error_state(&page_wrapper, WrapperErrEnum::UnknownError, "Unknown error in page loading");
}

/// 测试PageInfo的功能
#[test]
fn test_page_info() {
    use common_wrapper::PageInfo;
    use common_wrapper::PageWrapper;

    // 测试正常分页信息
    let _page_info = PageWrapper::<String>::new(); // 使用PageWrapper中的PageInfo

    // 测试默认页面大小
    // 注意：这里我们通过PageWrapper的内部结构测试PageInfo功能
    let page_info1 = PageInfo::new(Some(1), Some(10));
    assert_eq!(page_info1.get_page_size(), 10);
    assert_eq!(page_info1.get_page_offset(), 0);

    // 测试默认页面大小（无设置或为0）
    let page_info2 = PageInfo::new(Some(1), Some(0));
    assert_eq!(page_info2.get_page_size(), 20); // 默认值

    let page_info3 = PageInfo::new(Some(1), None);
    assert_eq!(page_info3.get_page_size(), 20); // 默认值

    // 测试页面偏移量计算
    let page_info4 = PageInfo::new(Some(3), Some(10));
    assert_eq!(page_info4.get_page_offset(), 20); // (3-1)*10 = 20

    // 测试第0页偏移量
    let page_info5 = PageInfo::new(Some(0), Some(10));
    assert_eq!(page_info5.get_page_offset(), 0);
}

/// 测试ResponseWrapper的基本功能
#[test]
fn test_response_wrapper() {
    // 测试默认成功响应
    let success_response = common_wrapper::ResponseWrapper::success_default();
    assert_eq!(success_response.get_code(), WrapperErrEnum::Success as i32);
    assert_eq!(success_response.get_message(), "Success");
    assert!(success_response.is_success());

    // 测试默认失败响应
    let fail_response = common_wrapper::ResponseWrapper::fail_default();
    assert_eq!(fail_response.get_code(), WrapperErrEnum::Fail as i32);
    assert_eq!(fail_response.get_message(), "Fail");
    assert!(!fail_response.is_success());

    // 测试默认未知错误响应
    let unknown_response = common_wrapper::ResponseWrapper::unknown_error_default();
    assert_eq!(unknown_response.get_code(), WrapperErrEnum::UnknownError as i32);
    assert_eq!(unknown_response.get_message(), "Unknown Error");
    assert!(!unknown_response.is_success());

    // 测试设置自定义失败消息
    let mut response = common_wrapper::ResponseWrapper::success_default();
    response.set_fail("Custom fail message");
    assert_eq!(response.get_code(), WrapperErrEnum::Fail as i32);
    assert_eq!(response.get_message(), "Custom fail message");
    assert!(!response.is_success());

    // 测试设置自定义未知错误消息
    let mut response2 = common_wrapper::ResponseWrapper::success_default();
    response2.set_unknown_error("Custom unknown error message");
    assert_eq!(response2.get_code(), WrapperErrEnum::UnknownError as i32);
    assert_eq!(response2.get_message(), "Custom unknown error message");
    assert!(!response2.is_success());
}

/// 测试序列化和反序列化功能
#[test]
fn test_serialization() {
    use serde_json;

    // 测试SingleWrapper序列化
    let mut obj_wrapper = SingleWrapper::new();
    obj_wrapper.set_success("Test data");
    let serialized = serde_json::to_string(&obj_wrapper).unwrap();
    let deserialized: SingleWrapper<String> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(obj_wrapper.get_code(), deserialized.get_code());
    assert_eq!(obj_wrapper.get_message(), deserialized.get_message());
    assert_eq!(obj_wrapper.get_data().map(|s| s.as_ref()), deserialized.get_data().map(|s| s.as_str()));

    // 测试ListWrapper序列化
    let mut list_wrapper = ListWrapper::new();
    list_wrapper.set_success(vec![1, 2, 3]);
    let serialized = serde_json::to_string(&list_wrapper).unwrap();
    let deserialized: ListWrapper<i32> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(list_wrapper.get_code(), deserialized.get_code());
    assert_eq!(list_wrapper.get_message(), deserialized.get_message());
    assert_eq!(list_wrapper.get_data(), deserialized.get_data());

    // 测试PageWrapper序列化
    let mut page_wrapper = PageWrapper::new();
    page_wrapper.set_success(vec!["a".to_string(), "b".to_string()], 100, 1, 10);
    let serialized = serde_json::to_string(&page_wrapper).unwrap();
    let deserialized: PageWrapper<String> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(page_wrapper.get_code(), deserialized.get_code());
    assert_eq!(page_wrapper.get_message(), deserialized.get_message());
    assert_eq!(page_wrapper.get_data(), deserialized.get_data());
    assert_eq!(page_wrapper.get_total_count(), deserialized.get_total_count());
    assert_eq!(page_wrapper.get_total_page(), deserialized.get_total_page());
    assert_eq!(page_wrapper.get_current_page_num(), deserialized.get_current_page_num());
    assert_eq!(page_wrapper.get_page_size(), deserialized.get_page_size());
}
