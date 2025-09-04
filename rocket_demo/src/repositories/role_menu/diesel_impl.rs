//! 角色菜单关联数据访问层 Diesel 实现

use diesel::prelude::*;

use crate::models::SysRoleMenu;
use crate::models::constants::ROLE_MENU_FIELDS;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;

table! {
    sys_role_menu (id) {
        id -> Text,
        role_id -> Nullable<Text>,
        menu_id -> Nullable<Text>,
    }
}

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_role_menu)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct RoleMenuRow {
    id: String,
    role_id: Option<String>,
    menu_id: Option<String>,
}

impl From<RoleMenuRow> for SysRoleMenu {
    fn from(row: RoleMenuRow) -> Self {
        SysRoleMenu { id: row.id, role_id: row.role_id, menu_id: row.menu_id }
    }
}

impl From<&SysRoleMenu> for RoleMenuRow {
    fn from(role_menu: &SysRoleMenu) -> Self {
        RoleMenuRow {
            id: role_menu.id.clone(),
            role_id: role_menu.role_id.clone(),
            menu_id: role_menu.menu_id.clone(),
        }
    }
}

/// 角色菜单关联数据访问 Diesel 实现
#[derive(Debug)]
pub struct RoleMenuRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl RoleMenuRepositoryDieselImpl {
    /// 创建角色菜单关联仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl RoleMenuRepository for RoleMenuRepositoryDieselImpl {
    /// 根据主键删除角色菜单关联
    async fn delete_by_primary_key(&self, role_menu_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        diesel::delete(sys_role_menu.filter(id.eq(role_menu_id))).execute(&mut self.connection)?;
        Ok(())
    }

    /// 插入角色菜单关联记录
    async fn insert(&self, row: &SysRoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let role_menu_row: RoleMenuRow = row.into();
        diesel::insert_into(sys_role_menu)
            .values(&role_menu_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 选择性插入角色菜单关联记录
    async fn insert_selective(&self, row: &SysRoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let role_menu_row: RoleMenuRow = row.into();
        diesel::insert_into(sys_role_menu)
            .values(&role_menu_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据主键查询角色菜单关联
    async fn select_by_id(&self, role_menu_id: &str) -> Result<Option<SysRoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let result = sys_role_menu
            .filter(id.eq(role_menu_id))
            .first::<RoleMenuRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(SysRoleMenu::from))
    }

    /// 查询角色菜单关联列表
    async fn select_list(&self, role_menu_param: crate::services::params::user_param::RoleMenuParam) -> Result<Vec<SysRoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let mut query = sys_role_menu.into_boxed();

        if let Some(role_id_filter) = &role_menu_param.role_id {
            query = query.filter(role_id.eq(role_id_filter));
        }

        if let Some(menu_id_filter) = &role_menu_param.menu_id {
            query = query.filter(menu_id.eq(menu_id_filter));
        }

        let result = query
            .order(id.asc())
            .load::<RoleMenuRow>(&mut self.connection)?;
        Ok(result.into_iter().map(SysRoleMenu::from).collect())
    }

    /// 根据主键更新角色菜单关联
    async fn update_by_id(&self, row: &SysRoleMenu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let role_menu_row: RoleMenuRow = row.into();
        let result = diesel::update(sys_role_menu.filter(id.eq(&row.id)))
            .set(&role_menu_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键选择性更新角色菜单关联
    async fn update_by_id_selective(&self, row: &SysRoleMenu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let role_menu_row: RoleMenuRow = row.into();
        let result = diesel::update(sys_role_menu.filter(id.eq(&row.id)))
            .set(&role_menu_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键删除角色菜单关联
    async fn delete_by_id(&self, role_menu_id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let result = diesel::delete(sys_role_menu.filter(id.eq(role_menu_id))).execute(&mut self.connection)?;
        Ok(result as u64)
    }
}
