use crate::wrapper::ResponseWrapper;
use crate::wrapper::response_trait::ResponseTrait;
use serde::{Deserialize, Serialize};

/// # 带单个数据对象的响应包装结构体
/// 用于统一 API 响应格式，包含基础响应信息和可选的单个数据对象
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ObjectWrapper<T> {
    /// 响应基础信息，包含响应码和消息
    #[serde(flatten)] // 扁平化，去掉json中的base把内部结构解构出来
    pub base: ResponseWrapper,
    /// 可选的数据对象
    pub data: Option<T>,
}

impl<T> ObjectWrapper<T> {
    /// 创建一个默认成功的 ObjectWrapper，数据为空
    pub fn new() -> Self {
        Self { base: ResponseWrapper::success_default(), data: None }
    }

    /// 创建一个默认失败的 ObjectWrapper，数据为空
    pub fn fail_default(&mut self) -> Self {
        Self { base: ResponseWrapper::fail_default(), data: None }
    }

    /// 创建一个默认未知错误的 ObjectWrapper，数据为空
    pub fn unknown_error_default(&mut self) -> Self {
        Self {
            base: ResponseWrapper::unknown_error_default(),
            data: None,
        }
    }

    /// 设置为成功状态并附带数据
    pub fn set_success(&mut self, data: T) {
        self.base = ResponseWrapper::success_default();
        self.data = Some(data);
    }

    /// 获取基础响应包装的引用
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }

    /// 获取数据对象的引用
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

/// 实现 ResponseTrait 以便统一处理响应包装
impl<T> ResponseTrait for ObjectWrapper<T> {
    /// 获取响应码
    fn get_code(&self) -> i32 {
        self.base.get_code()
    }

    /// 获取响应消息
    fn get_message(&self) -> &str {
        self.base.get_message()
    }

    /// 判断是否为成功响应
    fn is_success(&self) -> bool {
        self.base.is_success()
    }

    /// 设置为失败响应，并自定义消息，数据清空
    fn set_fail(&mut self, msg: impl Into<String>) {
        self.base.set_fail(msg);
        self.data = None;
    }

    /// 设置为未知错误响应，并自定义消息，数据清空
    fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base.set_unknown_error(msg);
        self.data = None;
    }
}
