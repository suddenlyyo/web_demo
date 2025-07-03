use crate::enums::WrapperErrEnum;
use crate::wrapper::ResponseWrapper;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};
// 带数据的包装
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ObjectWrapper<T> {
    #[serde(flatten)] //扁平化，去掉json中的base把内部结构解构出来
    pub base: ResponseWrapper,
    pub data: Option<T>,
}

impl<T> ObjectWrapper<T> {
    pub fn new() -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: None,
        }
    }

    pub fn success(data: T) -> Self {
        Self {
            base: ResponseWrapper::success_default(),
            data: Some(data),
        }
    }
    // 默认失败响应
    pub fn fail_default() -> Self {
        self.base.fail_default();
        self.data = None;
    }

    // 默认未知错误响应
    pub fn unknown_error_default() -> Self {
        self.base.unknown_error_default();
        self.data = None;
    }

    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

impl<T> ResponseTrait for ObjectWrapper<T> {
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
