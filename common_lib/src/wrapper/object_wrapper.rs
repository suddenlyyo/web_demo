use crate::enums::WrapperErr;
use crate::wrapper::ResponseWrapper;

// 带数据的包装
#[derive(Debug, serde::Serialize, PartialEq, Eq, Hash)]
pub struct ObjectWrapper<T> {
    #[serde(flatten)] //扁平化，去掉json中的base把内部结构提出来
    pub base: ResponseWrapper,
    pub data: Option<T>,
}

impl<T> ObjectWrapper<T> {
    pub fn new() -> Self {
        ObjectWrapper {
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

    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}
