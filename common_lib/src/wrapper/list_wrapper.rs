use crate::enums::WrapperErr;
use crate::wrapper::ResponseWrapper;

// 带列表数据的包装
#[derive(Debug, serde::Serialize, PartialEq, Eq, Hash)]
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

    pub fn success(data: T) -> Self {
        ObjectWrapper {
            base: ResponseWrapper::success_default(),
            data: Some(data),
        }
    }
    pub fn set_fail(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErr::Fail.code(), msg.into());
        self.data = None;
    }

    pub fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErr::UnknownError.code(), msg.into());
        self.data = None;
    }

    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    pub fn data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }
}
