//! 用户服务接口定义

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};

use crate::models::User;

/// 用户服务接口
#[rocket::async_trait]
pub trait UserService {
    /// 根据ID获取用户信息
    ///
    /// # 参数
    ///
    /// - `id`: 用户ID
    ///
    /// # 返回值
    ///
    /// 返回包装后的用户信息
    async fn get_user_by_id(&self, id: &str) -> SingleWrapper<User>;

    /// 获取用户列表
    ///
    /// # 返回值
    ///
    /// 返回包装后的用户列表
    async fn list_users(&self) -> ListWrapper<User>;

    /// 分页查询用户列表
    ///
    /// # 参数
    ///
    /// - `page_num`: 页码
    /// - `page_size`: 每页条数
    ///
    /// # 返回值
    ///
    /// 返回包装后的分页用户列表
    async fn list_users_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> PageWrapper<User>;

    /// 新增用户
    ///
    /// # 参数
    ///
    /// - `user`: 用户信息
    ///
    /// # 返回值
    ///
    /// 返回包装后的新增用户信息
    async fn add_user(&self, user: User) -> SingleWrapper<User>;

    /// 修改用户
    ///
    /// # 参数
    ///
    /// - `user`: 用户信息
    ///
    /// # 返回值
    ///
    /// 返回包装后的修改用户信息
    async fn update_user(&self, user: User) -> SingleWrapper<User>;

    /// 删除用户
    ///
    /// # 参数
    ///
    /// - `id`: 用户ID
    ///
    /// # 返回值
    ///
    /// 返回包装后的删除结果
    async fn delete_user(&self, id: &str) -> SingleWrapper<User>;

    /// 修改用户状态
    ///
    /// # 参数
    ///
    /// - `id`: 用户ID
    /// - `status`: 用户状态
    ///
    /// # 返回值
    ///
    /// 返回包装后的修改结果
    async fn update_user_status(&self, id: &str, status: i32) -> SingleWrapper<User>;
}
