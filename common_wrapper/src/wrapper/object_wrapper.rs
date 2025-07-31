use crate::wrapper::ResponseWrapper;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};
// 带数据的包装
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ObjectWrapper<T> {
    /// 响应基础信息
    #[serde(flatten)] //扁平化，去掉json中的base把内部结构解构出来
    pub base: ResponseWrapper,
    /// 数据
    pub data: Option<T>,
}

impl<T> ObjectWrapper<T> {
    pub fn new() -> Self {
        Self { base: ResponseWrapper::success_default(), data: None }
    }

    // 默认失败响应
    pub fn fail_default(&mut self) -> Self {
        Self { base: ResponseWrapper::fail_default(), data: None }
    }

    // 默认未知错误响应
    pub fn unknown_error_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: None,
        }
    }

    // 设置成功状态和数据
    pub fn set_success(&mut self, data: T) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
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
