//! 角色菜单关联数据访问层 Diesel 实现

use diesel::prelude::*;

use crate::models::RoleMenu;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;

// ==================== 表结构体映射 ====================
table! {
    sys_role_menu (id) {
        id -> Text,
        role_id -> Nullable<Text>,
        menu_id -> Nullable<Text>,
        create_by -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
    }
}

#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = sys_role_menu)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct RoleMenuRow {
    id: String,
    role_id: Option<String>,
    menu_id: Option<String>,
    create_by: Option<String>,
    create_time: Option<chrono::NaiveDateTime>,
}

impl From<RoleMenuRow> for RoleMenu {
    fn from(row: RoleMenuRow) -> Self {
        RoleMenu {
            id: row.id,
            role_id: row.role_id,
            menu_id: row.menu_id,
            create_by: row.create_by,
            create_time: row
                .create_time
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
        }
    }
}

impl From<&RoleMenu> for RoleMenuRow {
    fn from(role_menu: &RoleMenu) -> Self {
        RoleMenuRow {
            id: role_menu.id.clone(),
            role_id: role_menu.role_id.clone(),
            menu_id: role_menu.menu_id.clone(),
            create_by: role_menu.create_by.clone(),
            create_time: role_menu.create_time.map(|t| t.naive_utc()),
        }
    }
}

// ==================== SQL trait 实现 ====================
#[derive(Debug)]
pub struct RoleMenuRepositoryDieselImpl {
    connection: diesel::mysql::MysqlConnection,
}

impl RoleMenuRepositoryDieselImpl {
    pub fn new() -> Self {
        let database_url = if let Ok(config) = crate::config::Config::from_default_file() {
            config.database.url
        } else {
            panic!("无法从配置文件获取数据库连接信息");
        };

        let connection = diesel::mysql::MysqlConnection::establish(&database_url).expect("连接MySQL数据库时出错");

        Self { connection }
    }
}

#[rocket::async_trait]
impl RoleMenuRepository for RoleMenuRepositoryDieselImpl {
    async fn select_role_menu_by_role_id(&self, role_id: &str) -> Result<Vec<RoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let result = sys_role_menu
            .filter(role_id.eq(role_id))
            .load::<RoleMenuRow>(&mut self.connection)?;

        Ok(result.into_iter().map(RoleMenu::from).collect())
    }

    async fn insert(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let role_menu_row: RoleMenuRow = row.into();
        diesel::insert_into(sys_role_menu)
            .values(&role_menu_row)
            .execute(&mut self.connection)?;
        Ok(())
    }

    async fn batch_insert(&self, rows: &[RoleMenu]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let role_menu_rows: Vec<RoleMenuRow> = rows.iter().map(|r| r.into()).collect();
        diesel::insert_into(sys_role_menu)
            .values(&role_menu_rows)
            .execute(&mut self.connection)?;
        Ok(())
    }

    async fn delete_by_role_id(&self, role_id_filter: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        use crate::repositories::role_menu::diesel_impl::sys_role_menu::dsl::*;

        let result = diesel::delete(sys_role_menu.filter(role_id.eq(role_id_filter))).execute(&mut self.connection)?;
        Ok(result as u64)
    }
}
