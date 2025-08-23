//! 部门服务接口定义

use common_wrapper::{ListWrapper, PageWrapper, SingleWrapper};

use crate::models::Dept;

/// 部门服务trait
#[rocket::async_trait]
pub trait DeptService {
    /// 根据ID获取部门信息
    async fn get_dept_by_id(&self, id: &str) -> SingleWrapper<Dept>;

    /// 获取部门列表
    async fn list_depts(&self) -> ListWrapper<Dept>;

    /// 分页查询部门列表
    async fn list_depts_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> PageWrapper<Dept>;

    /// 根据父部门ID获取子部门列表
    async fn list_children_by_parent_id(&self, parent_id: &str) -> ListWrapper<Dept>;

    /// 获取部门树结构
    async fn list_dept_tree(&self) -> ListWrapper<Dept>;

    /// 新增部门
    async fn add_dept(&self, dept: Dept) -> SingleWrapper<Dept>;

    /// 修改部门
    async fn update_dept(&self, dept: Dept) -> SingleWrapper<Dept>;

    /// 删除部门
    async fn delete_dept(&self, id: &str) -> SingleWrapper<Dept>;

    /// 修改部门状态
    async fn update_dept_status(&self, id: &str, status: i32) -> SingleWrapper<Dept>;
}
