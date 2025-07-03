use crate::wrapper::ResponseWrapper;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};
// 带列表数据的包装
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListWrapper<T> {
    #[serde(flatten)]
    base: ResponseWrapper,
    data: Option<Vec<T>>,
}

impl<T> ListWrapper<T> {
    pub fn new() -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: None,
        }
    }

    pub fn success(data: Vec<T>) -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: Some(data),
        }
    }

    // 默认失败响应
    pub fn fail_default(&mut self) -> Self {
        self.base.fail_default();
        self.data = None;
    }

    // 默认未知错误响应
    pub fn unknown_error_default(&mut self) -> Self {
        self.base.unknown_error_default();
        self.data = None;
    }

    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    pub fn data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }
}

impl<T> ResponseTrait for ListWrapper<T> {
    fn get_code(&self) -> i32 {
        self.base.get_code()
    }

    fn get_message(&self) -> &str {
        self.base.get_message()
    }

    fn is_success(&self) -> bool {
        self.base.is_success()
    }

    fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
    }

    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
    }
}
