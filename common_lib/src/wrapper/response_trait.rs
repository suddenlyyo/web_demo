/// 定义响应包装的公共行为
pub trait ResponseTrait {
    fn get_code(&self) -> i32;
    fn get_message(&self) -> &str;
    fn is_success(&self) -> bool;
    
    fn set_fail(&mut self, msg: impl Into<String>);
    fn set_unknown_error(&mut self, msg: impl Into<String>);
}