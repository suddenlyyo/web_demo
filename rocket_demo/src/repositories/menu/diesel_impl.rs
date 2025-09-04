//! 菜单数据访问层 Diesel 实现

use diesel::prelude::*;

use crate::models::Menu;
use crate::models::constants::MENU_FIELDS;
use crate::repositories::menu::menu_repository::MenuRepository;

table! {
    sys_menu (id) {
        id -> Text,
        menu_name -> Nullable<Text>,
        menu_level -> Nullable<Text>,
        parent_id -> Nullable<Text>,
        seq_no -> Nullable<Integer>,
        path -> Nullable<Text>,
        component -> Nullable<Text>,
        query -> Nullable<Text>,
        is_frame -> Nullable<Integer>,
        is_cache -> Nullable<Integer>,
        menu_type -> Nullable<Text>,
        visible -> Nullable<Text>,
        status -> Nullable<Integer>,
        perms -> Nullable<Text>,
        icon -> Nullable<Text>,
        create_by -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
        update_by -> Nullable<Text>,
        update_time -> Nullable<Timestamp>,
        remark -> Nullable<Text>,
    }
}

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_menu)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct MenuRow {
    id: String,
    menu_name: Option<String>,
    menu_level: Option<String>,
    parent_id: Option<String>,
    seq_no: Option<i32>,
    path: Option<String>,
    component: Option<String>,
    query: Option<String>,
    is_frame: Option<i32>,
    is_cache: Option<i32>,
    menu_type: Option<String>,
    visible: Option<String>,
    status: Option<i32>,
    perms: Option<String>,
    icon: Option<String>,
    create_by: Option<String>,
    create_time: Option<chrono::NaiveDateTime>,
    update_by: Option<String>,
    update_time: Option<chrono::NaiveDateTime>,
    remark: Option<String>,
}

impl From<MenuRow> for Menu {
    fn from(row: MenuRow) -> Self {
        Menu {
            id: row.id,
            menu_name: row.menu_name,
            menu_level: row.menu_level,
            parent_id: row.parent_id,
            seq_no: row.seq_no,
            path: row.path,
            component: row.component,
            query: row.query,
            is_frame: row.is_frame,
            is_cache: row.is_cache,
            menu_type: row.menu_type,
            visible: row.visible,
            status: row.status,
            perms: row.perms,
            icon: row.icon,
            create_by: row.create_by,
            create_time: row
                .create_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: row.remark,
        }
    }
}

impl From<&Menu> for MenuRow {
    fn from(menu: &Menu) -> Self {
        MenuRow {
            id: menu.id.clone(),
            menu_name: menu.menu_name.clone(),
            menu_level: menu.menu_level.clone(),
            parent_id: menu.parent_id.clone(),
            seq_no: menu.seq_no,
            path: menu.path.clone(),
            component: menu.component.clone(),
            query: menu.query.clone(),
            is_frame: menu.is_frame,
            is_cache: menu.is_cache,
            menu_type: menu.menu_type.clone(),
            visible: menu.visible.clone(),
            status: menu.status,
            perms: menu.perms.clone(),
            icon: menu.icon.clone(),
            create_by: menu.create_by.clone(),
            create_time: menu.create_time.map(|t| t.naive_utc()),
            update_by: menu.update_by.clone(),
            update_time: menu.update_time.map(|t| t.naive_utc()),
            remark: menu.remark.clone(),
        }
    }
}

/// 菜单数据访问 Diesel 实现
#[derive(Debug)]
pub struct MenuRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl MenuRepositoryDieselImpl {
    /// 创建菜单仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl MenuRepository for MenuRepositoryDieselImpl {
    /// 根据主键删除菜单
    async fn delete_by_primary_key(&self, menu_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        diesel::delete(sys_menu.filter(id.eq(menu_id))).execute(&mut self.connection)?;
        Ok(())
    }

    /// 插入菜单记录
    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        let menu_row: MenuRow = row.into();
        diesel::insert_into(sys_menu)
            .values(&menu_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 选择性插入菜单记录
    async fn insert_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        let menu_row: MenuRow = row.into();
        diesel::insert_into(sys_menu)
            .values(&menu_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据主键查询菜单
    async fn select_menu_by_id(&self, menu_id: &str) -> Result<Option<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        let result = sys_menu
            .filter(id.eq(menu_id))
            .first::<MenuRow>(&mut self.connection)
            .optional()?;
        Ok(result.map(Menu::from))
    }

    /// 查询菜单列表
    async fn select_menu_list(&self, menu_param: crate::services::params::user_param::MenuParam) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        let mut query = sys_menu.into_boxed();

        if let Some(menu_name_filter) = &menu_param.menu_name {
            query = query.filter(menu_name.like(format!("%{}%", menu_name_filter)));
        }

        if let Some(status_filter) = menu_param.status {
            query = query.filter(status.eq(status_filter));
        }

        let result = query
            .order(id.asc())
            .load::<MenuRow>(&mut self.connection)?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    /// 根据主键更新菜单
    async fn update_by_id(&self, row: &Menu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        let menu_row: MenuRow = row.into();
        let result = diesel::update(sys_menu.filter(id.eq(&row.id)))
            .set(&menu_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键选择性更新菜单
    async fn update_by_id_selective(&self, row: &Menu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        let menu_row: MenuRow = row.into();
        let result = diesel::update(sys_menu.filter(id.eq(&row.id)))
            .set(&menu_row)
            .execute(&mut self.connection)?;
        Ok(result as u64)
    }

    /// 根据主键删除菜单
    async fn delete_by_id(&self, menu_id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::menu::diesel_impl::sys_menu::dsl::*;

        let result = diesel::delete(sys_menu.filter(id.eq(menu_id))).execute(&mut self.connection)?;
        Ok(result as u64)
    }
}
