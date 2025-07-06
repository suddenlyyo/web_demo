/// 定义响应包装的公共行为
pub trait ResponseTrait {
    /// 获取响应码
    fn get_code(&self) -> i32;
    /// 获取响应消息
    fn get_message(&self) -> &str;
    /// 判断响应是否成功
    fn is_success(&self) -> bool;
    /// 设置失败状态和消息
    fn set_fail(&mut self, msg: impl Into<String>);
    /// 设置未知错误状态和消息
    fn set_unknown_error(&mut self, msg: impl Into<String>);
}
