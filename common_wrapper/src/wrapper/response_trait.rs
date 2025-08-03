//! # 响应trait
//!
//! 定义响应包装的公共行为接口

/// 定义响应包装的公共行为
///
/// 所有响应包装类型都应实现此trait以提供统一的接口
pub trait ResponseTrait {
    /// 获取响应码
    ///
    /// # 返回值
    ///
    /// 返回响应码，通常用于标识请求结果（如成功、失败等）
    fn get_code(&self) -> i32;

    /// 获取响应消息
    ///
    /// # 返回值
    ///
    /// 返回响应消息，描述请求结果的详细信息
    fn get_message(&self) -> &str;

    /// 判断响应是否成功
    ///
    /// # 返回值
    ///
    /// 如果响应成功返回true，否则返回false
    fn is_success(&self) -> bool;

    /// 设置失败状态和消息
    ///
    /// # 参数
    ///
    /// * `msg` - 失败消息
    fn set_fail(&mut self, msg: impl Into<String>);

    /// 设置未知错误状态和消息
    ///
    /// # 参数
    ///
    /// * `msg` - 未知错误消息
    fn set_unknown_error(&mut self, msg: impl Into<String>);
}
