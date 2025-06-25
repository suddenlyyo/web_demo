use crate::enums::WrapperErr;
use crate::wrapper::ResponseWrapper;

// 带数据的包装
#[derive(Debug, serde::Serialize)]
pub struct ObjectWrapper<T> {
    pub base: ResponseWrapper,
    pub data: T,
}

impl<T> ObjectWrapper<T> {
    /// 创建新的带数据包装
    pub fn new(base: ResponseWrapper, data: T) -> Self {
        ObjectWrapper { base, data }
    }
    /// 创建默认成功的带数据包装
    pub fn success(data: T) -> Self {
        ObjectWrapper {
            base: ResponseWrapper::success_default(),
            data,
        }
    }
    /// 创建默认失败的带数据包装
    pub fn fail(data: T) -> Self {
        ObjectWrapper {
            base: ResponseWrapper::fail_default(),
            data,
        }
    }
    /// 设置失败状态（保留现有数据）
    pub fn set_fail(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErr::Fail.code(), msg);
    }
    /// 设置未知错误状态（保留现有数据）
    pub fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErr::UnknownError.code(), msg);
    }
    /// 创建默认未知错误的带数据包装
    pub fn unknown_error(data: T) -> Self {
        ObjectWrapper {
            base: ResponseWrapper::unknown_error_default(),
            data,
        }
    }
    /// 获取基础响应包装的引用
    pub fn get_base(&self) -> &ResponseWrapper {
        &self.base
    }
    /// 获取基础响应包装的可变引用
    pub fn get_base_mut(&mut self) -> &mut ResponseWrapper {
        &mut self.base
    }
    /// 获取数据的引用
    pub fn get_data(&self) -> &T {
        &self.data
    }
    /// 获取数据的可变引用
    pub fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }
    /// 从错误枚举创建带数据包装
    pub fn from(err: WrapperErr, data: T) -> Self {
        ObjectWrapper {
            base: ResponseWrapper::from(err),
            data,
        }
    }
}
