mod enums;
mod wrapper;

#[cfg(test)]
mod tests {
    use crate::{
        enums::WrapperErrEnum,
        wrapper::{ListWrapper, ObjectWrapper, PageWrapper, ResponseWrapper},
    };

    #[test]
    fn wrapper_err_enum_test() {
        let success = WrapperErrEnum::Success;
        assert_eq!(success.code(), 1);
        assert_eq!(success.message(), "Success");

        let fail = WrapperErrEnum::Fail;
        assert_eq!(fail.code(), -1);
        assert_eq!(fail.message(), "Fail");

        let unknown_error = WrapperErrEnum::UnknownError;
        assert_eq!(unknown_error.code(), -2);
        assert_eq!(unknown_error.message(), "Unknown Error");
    }

    #[test]
    fn response_wrapper_test() {
        let response = ResponseWrapper::success_default();
        assert_eq!(response.get_code(), WrapperErrEnum::Success.code());
        assert_eq!(response.get_message(), "Success");
        let mut fail = ResponseWrapper::fail_default();
        assert_eq!(fail.get_code(), WrapperErrEnum::Fail.code());
        assert_eq!(fail.get_message(), "Fail");
        fail.fail("New Fail Message");
        assert_eq!(fail.get_code(), WrapperErrEnum::Fail.code());
        assert_eq!(fail.get_message(), "New Fail Message");
        let mut unknown_error = ResponseWrapper::unknown_error_default();
        assert_eq!(
            unknown_error.get_code(),
            WrapperErrEnum::UnknownError.code()
        );
        assert_eq!(unknown_error.get_message(), "Unknown Error");
        unknown_error.unknown_error("New Unknown Error Message");
        assert_eq!(
            unknown_error.get_code(),
            WrapperErrEnum::UnknownError.code()
        );
        assert_eq!(unknown_error.get_message(), "New Unknown Error Message");
    }

    #[test]
    fn object_wrapper_test() {
        let data = ObjectWrapper::success("Test Data");
        assert_eq!(data.get_base().get_code(), WrapperErrEnum::Success.code());
        assert_eq!(data.get_data(), Some(&"Test Data"));

        let mut fail_data = ObjectWrapper::<String>::new();
        fail_data.set_fail("Failed");
        assert_eq!(fail_data.get_base().get_code(), WrapperErrEnum::Fail.code());
        assert_eq!(fail_data.get_data(), None);
        let mut unknown_error_data = ObjectWrapper::<String>::new();
        unknown_error_data.set_unknown_error("Unknown Error");
        assert_eq!(
            unknown_error_data.get_base().get_code(),
            WrapperErrEnum::UnknownError.code()
        );
        assert_eq!(unknown_error_data.get_data(), None);
    }

    #[test]
    fn list_wrapper_test() {
        let data = ListWrapper::success(vec!["Test Data 1", "Test Data 2"]);
        assert_eq!(data.get_base().get_code(), WrapperErrEnum::Success.code());
        assert_eq!(data.data(), Some(&vec!["Test Data 1", "Test Data 2"]));
        let mut fail_data = ListWrapper::<String>::new();
        fail_data.set_fail("Failed");
        assert_eq!(fail_data.get_base().get_code(), WrapperErrEnum::Fail.code());
        assert_eq!(fail_data.data(), None);
        let mut unknown_error_data = ListWrapper::<String>::new();
        unknown_error_data.set_unknown_error("Unknown Error");
        assert_eq!(
            unknown_error_data.get_base().get_code(),
            WrapperErrEnum::UnknownError.code()
        );
        assert_eq!(unknown_error_data.data(), None);
    }

    #[test]
    fn page_wrapper_test() {
        let mut page_wrapper = PageWrapper::<String>::new();
        page_wrapper = PageWrapper::success(vec!["1".to_string(), "2".to_string()], 2, 1, 1, 1);
        assert_eq!(page_wrapper.get_total(), 2);
        assert_eq!(page_wrapper.get_total_page(), 1);
        assert_eq!(page_wrapper.get_current_page_num(), 1);
        assert_eq!(page_wrapper.get_page_size(), 1);
        assert_eq!(
            page_wrapper.get_data(),
            Some(&vec!["1".to_string(), "2".to_string()])
        );
        page_wrapper.set_fail("Fail");
        assert_eq!(
            page_wrapper.get_base().get_code(),
            WrapperErrEnum::Fail.code()
        );
        assert_eq!(page_wrapper.get_data(), None);
        page_wrapper.set_unknown_error("Unknown Error");
        assert_eq!(
            page_wrapper.get_base().get_code(),
            WrapperErrEnum::UnknownError.code()
        );
        assert_eq!(page_wrapper.get_data(), None);
    }
}
