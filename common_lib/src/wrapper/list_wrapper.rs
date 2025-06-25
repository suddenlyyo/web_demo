use crate::enums::WrapperErr;
use crate::wrapper::ResponseWrapper;

// 带列表数据的包装
#[derive(Debug,serde::Serialize, PartialEq, Eq, Hash, Default)]
pub struct ListWrapper<T> {
    pub base: ResponseWrapper,
    pub data: T,
}
impl<T> ListWrapper<T> {
    /// 创建新的列表包装
    pub fn new(base: ResponseWrapper, data: Vec<T>) -> Self {
        ListWrapper { base, data }
    }

    /// 创建成功响应并携带数据列表
    pub fn success(data: Vec<T>) -> Self {
        ListWrapper {
            base: ResponseWrapper::success_default(),
            data,
        }
    }

    /// 创建空列表的成功响应
    pub fn success_empty() -> Self {
        ListWrapper {
            base: ResponseWrapper::success_default(),
            data: Vec::new(),
        }
    }

    /// 创建失败响应并携带数据列表
    pub fn fail(data: Vec<T>) -> Self {
        ListWrapper {
            base: ResponseWrapper::fail_default(),
            data,
        }
    }

    /// 从错误枚举创建列表包装
    pub fn from_err(err: WrapperErr, data: Vec<T>) -> Self {
        ListWrapper {
            base: ResponseWrapper::from(err),
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
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    /// 获取数据的可变引用
    pub fn get_data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// 设置失败状态（保留现有数据）
    pub fn set_fail(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErr::Fail.code(), msg);
    }

    /// 设置未知错误状态（保留现有数据）
    pub fn set_unknown_error(&mut self, msg: impl Into<String>) {
        self.base = ResponseWrapper::new(WrapperErr::UnknownError.code(), msg);
    }

    /// 转换数据为新的列表包装
    pub fn map<U, F>(self, f: F) -> ListWrapper<U>
    where
        F: FnMut(T) -> U,
    {
        ListWrapper {
            base: self.base,
            data: self.data.into_iter().map(f).collect(),
        }
    }

    /// 添加元素到列表
    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }
}

// 实现 From 转换以便于从单个元素创建列表
impl<T> From<T> for ListWrapper<T> {
    fn from(item: T) -> Self {
        ListWrapper::success(vec![item])
    }
}

// 实现 From 转换以便于从 Vec 创建列表
impl<T> From<Vec<T>> for ListWrapper<T> {
    fn from(data: Vec<T>) -> Self {
        ListWrapper::success(data)
    }
}

// 实现 Default trait
impl<T> Default for ListWrapper<T> {
    fn default() -> Self {
        ListWrapper::success_empty()
    }
}