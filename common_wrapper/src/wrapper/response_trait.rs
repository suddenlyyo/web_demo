//! # 响应trait
//!
//! 定义响应包装的公共行为接口
//!
//! 所有响应包装类型都应实现此trait以提供统一的接口，
//! 使得上层代码可以以统一的方式处理不同类型的响应。

/// 定义响应包装的公共行为
///
/// 所有响应包装类型都应实现此trait以提供统一的接口
pub trait ResponseTrait {
    /// 获取响应码
    ///
    /// 获取响应的状态码，用于标识请求结果（如成功、失败等）
    ///
    /// # 返回值
    ///
    /// [i32] - 响应码
    fn get_code(&self) -> i32;

    /// 获取响应消息
    ///
    /// 获取响应的消息，描述请求结果的详细信息
    ///
    /// # 返回值
    ///
    /// &[str] - 响应消息的引用
    fn get_message(&self) -> &str;

    /// 判断响应是否成功
    ///
    /// 根据响应码判断响应是否表示操作成功
    ///
    /// # 返回值
    ///
    /// [bool] - 如果响应成功返回true，否则返回false
    fn is_success(&self) -> bool;

    /// 设置失败状态和消息
    ///
    /// 将响应设置为失败状态，并指定失败消息
    ///
    /// # 参数
    ///
    /// * `msg` - 失败消息，类型: impl [Into]<[String]>，可以接受 &str 或 String
    fn set_fail(&mut self, msg: impl Into<String>);

    /// 设置未知错误状态和消息
    ///
    /// 将响应设置为未知错误状态，并指定错误消息
    ///
    /// # 参数
    ///
    /// * `msg` - 未知错误消息，类型: impl [Into]<[String]>，可以接受 &str 或 String
    fn set_unknown_error(&mut self, msg: impl Into<String>);
}
