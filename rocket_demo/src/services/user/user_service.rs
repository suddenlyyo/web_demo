//! 用户服务接口定义

use crate::models::User;
use crate::services::params::page_param::PageParam;
use crate::services::params::user_param::UserParam;
use common_wrapper::{ListWrapper, PageWrapper, ResponseWrapper, SingleWrapper};

/// 用户服务接口
#[rocket::async_trait]
pub trait UserService {
    /// 根据用户名查询用户信息
    ///
    /// # 参数
    /// * `user_name` - 用户名，类型: [&str]
    ///
    /// # 返回值
    /// 返回用户信息，类型: [Option<User>]
    async fn select_user_by_user_name(&self, user_name: &str) -> Option<User>;

    /// 分页查询用户列表
    ///
    /// # 参数
    /// * `user_param` - 用户参数，类型: [UserParam]
    ///
    /// # 返回值
    /// 返回分页用户列表，类型: [PageWrapper<User>]
    async fn get_user_list_by_page(&self, user_param: UserParam) -> PageWrapper<User>;

    /// 新增用户
    ///
    /// # 参数
    /// * `user_param` - 用户参数，类型: [UserParam]
    ///
    /// # 返回值
    /// 返回响应结果，类型: [ResponseWrapper]
    async fn add_user(&self, user_param: UserParam) -> ResponseWrapper;

    /// 编辑用户
    ///
    /// # 参数
    /// * `user_param` - 用户参数，类型: [UserParam]
    ///
    /// # 返回值
    /// 返回响应结果，类型: [ResponseWrapper]
    async fn edit_user(&self, user_param: UserParam) -> ResponseWrapper;

    /// 编辑用户状态
    ///
    /// # 参数
    /// * `id` - 用户ID，类型: [&str]
    /// * `status` - 用户状态，类型: [i32]
    ///
    /// # 返回值
    /// 返回响应结果，类型: [ResponseWrapper]
    async fn edit_user_status(&self, id: &str, status: i32) -> ResponseWrapper;

    /// 删除用户
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    ///
    /// # 返回值
    /// 返回响应结果，类型: [ResponseWrapper]
    async fn delete_user(&self, user_id: &str) -> ResponseWrapper;

    /// 重置密码
    ///
    /// # 参数
    /// * `user_param` - 用户参数，类型: [UserParam]
    ///
    /// # 返回值
    /// 返回响应结果，类型: [ResponseWrapper]
    async fn reset_user_pwd(&self, user_param: UserParam) -> ResponseWrapper;

    /// 分配角色
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    /// * `role_ids` - 角色ID列表，类型: [&[String]]
    ///
    /// # 返回값
    /// 返回响应结果，类型: [ResponseWrapper]
    async fn set_user_role(&self, user_id: &str, role_ids: &[String]) -> ResponseWrapper;

    /// 查询用户的角色id列表
    ///
    /// # 参数
    /// * `user_id` - 用户ID，类型: [&str]
    ///
    /// # 返回값
    /// 返回角色ID集合，类型: [SingleWrapper<std::collections::HashSet<String>>]
    async fn select_role_ids_by_user_id(&self, user_id: &str) -> SingleWrapper<std::collections::HashSet<String>>;
}
