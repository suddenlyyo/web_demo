use crate::enums::WrapperErrEnum;
use crate::wrapper::ResponseWrapper;
use serde::{Deserialize, Serialize};
// 带列表数据的包装
#[derive(Debug, Serialize,Deserialize, PartialEq, Eq, Hash)]
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
    pub fn set_fail(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErrEnum::Fail.code(), msg.into());
        self.data = None;
    }

    pub fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErrEnum::UnknownError.code(), msg.into());
        self.data = None;
    }

    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    pub fn data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }
}
