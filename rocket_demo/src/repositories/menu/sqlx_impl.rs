//! 菜单数据访问层SQLx实现

use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::OnceLock;

use crate::models::Menu;
use crate::params::menu_param::MenuParam;
use crate::repositories::menu::menu_repository::MenuRepository;

// 数据库连接池
static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

#[derive(Debug)]
pub struct MenuRepositorySqlxImpl {
    pool: MySqlPool,
}

impl MenuRepositorySqlxImpl {
    pub fn new() -> Self {
        let pool = DB_POOL.get().expect("数据库连接池未初始化").clone();
        Self { pool }
    }

    pub fn init_pool(pool: MySqlPool) {
        DB_POOL.set(pool).ok(); // 如果已经设置过，则忽略
    }
}

// ==================== 表结构体映射 ====================
#[derive(Debug, FromRow)]
struct MenuRow {
    id: String,
    name: Option<String>,
    parent_id: Option<String>,
    seq_no: Option<i32>,
    menu_type: Option<String>,
    url: Option<String>,
    perms: Option<String>,
    status: Option<i32>,
    hidden: Option<i32>,
    always_show: Option<i32>,
    redirect: Option<String>,
    component: Option<String>,
    href: Option<String>,
    icon: Option<String>,
    no_cache: Option<i32>,
    affix: Option<i32>,
    breadcrumb: Option<i32>,
    active_menu: Option<String>,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<chrono::NaiveDateTime>,
    update_by: Option<String>,
    #[sqlx(rename = "update_time")]
    update_time_raw: Option<chrono::NaiveDateTime>,
    remark: Option<String>,
}

impl From<MenuRow> for Menu {
    fn from(row: MenuRow) -> Self {
        Menu {
            id: row.id,
            name: row.name,
            parent_id: row.parent_id,
            seq_no: row.seq_no,
            menu_type: row.menu_type,
            url: row.url,
            perms: row.perms,
            status: row.status,
            hidden: row.hidden,
            always_show: row.always_show,
            redirect: row.redirect,
            component: row.component,
            href: row.href,
            icon: row.icon,
            no_cache: row.no_cache,
            affix: row.affix,
            breadcrumb: row.breadcrumb,
            active_menu: row.active_menu,
            create_by: row.create_by,
            create_time: row
                .create_time_raw
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time_raw
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
            remark: row.remark,
        }
    }
}

// ==================== SQL trait 实现 ====================
#[rocket::async_trait]
impl MenuRepository for MenuRepositorySqlxImpl {
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "DELETE FROM sys_menu WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "INSERT INTO sys_menu (id, name, parent_id, seq_no, menu_type, url, perms, status, hidden, always_show, redirect, component, href, icon, no_cache, affix, breadcrumb, active_menu, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.name)
            .bind(&row.parent_id)
            .bind(row.seq_no)
            .bind(&row.menu_type)
            .bind(&row.url)
            .bind(&row.perms)
            .bind(row.status)
            .bind(&row.hidden)
            .bind(&row.always_show)
            .bind(&row.redirect)
            .bind(&row.component)
            .bind(&row.href)
            .bind(&row.icon)
            .bind(&row.no_cache)
            .bind(&row.affix)
            .bind(&row.breadcrumb)
            .bind(&row.active_menu)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err("菜单插入失败".into());
        }

        Ok(())
    }

    async fn insert_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

        fields.push("id");
        placeholders.push("?");
        params.push(&row.id);

        if row.menu_name.is_some() {
            fields.push("menu_name");
            placeholders.push("?");
            params.push(&row.menu_name);
        }

        if row.menu_level.is_some() {
            fields.push("menu_level");
            placeholders.push("?");
            params.push(&row.menu_level);
        }

        if row.menu_type.is_some() {
            fields.push("menu_type");
            placeholders.push("?");
            params.push(&row.menu_type);
        }

        if row.parent_id.is_some() {
            fields.push("parent_id");
            placeholders.push("?");
            params.push(&row.parent_id);
        }

        if row.seq_no.is_some() {
            fields.push("seq_no");
            placeholders.push("?");
            params.push(&row.seq_no);
        }

        if row.icon.is_some() {
            fields.push("icon");
            placeholders.push("?");
            params.push(&row.icon);
        }

        if row.route_path.is_some() {
            fields.push("route_path");
            placeholders.push("?");
            params.push(&row.route_path);
        }

        if row.route_params.is_some() {
            fields.push("route_params");
            placeholders.push("?");
            params.push(&row.route_params);
        }

        if row.status.is_some() {
            fields.push("status");
            placeholders.push("?");
            params.push(&row.status);
        }

        if row.create_by.is_some() {
            fields.push("create_by");
            placeholders.push("?");
            params.push(&row.create_by);
        }

        if row.create_time.is_some() {
            fields.push("create_time");
            placeholders.push("?");
            params.push(&row.create_time.map(|t| t.naive_utc()));
        }

        if row.update_by.is_some() {
            fields.push("update_by");
            placeholders.push("?");
            params.push(&row.update_by);
        }

        if row.update_time.is_some() {
            fields.push("update_time");
            placeholders.push("?");
            params.push(&row.update_time.map(|t| t.naive_utc()));
        }

        if row.remark.is_some() {
            fields.push("remark");
            placeholders.push("?");
            params.push(&row.remark);
        }

        let sql = format!("INSERT INTO sys_menu ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("菜单插入失败"));
        }

        Ok(())
    }

    async fn select_menu_by_id(&self, id: &str) -> Result<Option<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, menu_name, menu_level, menu_type, parent_id, seq_no, icon, route_path, route_params, status, create_by, create_time, update_by, update_time, remark FROM sys_menu WHERE id = ?";
        let result: Option<MenuRow> = sqlx::query_as(sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.map(Menu::from))
    }

    async fn update_by_id_selective(&self, row: &Menu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

        if row.menu_name.is_some() {
            updates.push("menu_name = ?");
            params.push(&row.menu_name);
        }

        if row.menu_level.is_some() {
            updates.push("menu_level = ?");
            params.push(&row.menu_level);
        }

        if row.menu_type.is_some() {
            updates.push("menu_type = ?");
            params.push(&row.menu_type);
        }

        if row.parent_id.is_some() {
            updates.push("parent_id = ?");
            params.push(&row.parent_id);
        }

        if row.seq_no.is_some() {
            updates.push("seq_no = ?");
            params.push(&row.seq_no);
        }

        if row.icon.is_some() {
            updates.push("icon = ?");
            params.push(&row.icon);
        }

        if row.route_path.is_some() {
            updates.push("route_path = ?");
            params.push(&row.route_path);
        }

        if row.route_params.is_some() {
            updates.push("route_params = ?");
            params.push(&row.route_params);
        }

        if row.status.is_some() {
            updates.push("status = ?");
            params.push(&row.status);
        }

        if row.create_by.is_some() {
            updates.push("create_by = ?");
            params.push(&row.create_by);
        }

        if row.create_time.is_some() {
            updates.push("create_time = ?");
            params.push(&row.create_time.map(|t| t.naive_utc()));
        }

        if row.update_by.is_some() {
            updates.push("update_by = ?");
            params.push(&row.update_by);
        }

        if row.update_time.is_some() {
            updates.push("update_time = ?");
            params.push(&row.update_time.map(|t| t.naive_utc()));
        }

        if row.remark.is_some() {
            updates.push("remark = ?");
            params.push(&row.remark);
        }

        if updates.is_empty() {
            return Ok(0);
        }

        let sql = format!("UPDATE sys_menu SET {} WHERE id = ?", updates.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }
        query = query.bind(&row.id);

        let result = query.execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    async fn update_by_id(&self, row: &Menu) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "UPDATE sys_menu SET menu_name = ?, menu_level = ?, menu_type = ?, parent_id = ?, seq_no = ?, icon = ?, route_path = ?, route_params = ?, status = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";

        let result = sqlx::query(sql)
            .bind(&row.menu_name)
            .bind(&row.menu_level)
            .bind(&row.menu_type)
            .bind(&row.parent_id)
            .bind(row.seq_no)
            .bind(&row.icon)
            .bind(&row.route_path)
            .bind(&row.route_params)
            .bind(row.status)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .bind(&row.id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    async fn select_sys_menu_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT DISTINCT m.id, m.menu_name, m.menu_level, m.menu_type, m.parent_id, m.seq_no, m.icon, m.route_path, m.route_params, m.status, m.create_by, m.create_time, m.update_by, m.update_time, m.remark FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id LEFT JOIN sys_user_role ur ON rm.role_id = ur.role_id WHERE ur.user_id = ? ORDER BY m.id";
        let result: Vec<MenuRow> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    async fn select_menu_tree_all(&self) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, menu_name, menu_level, menu_type, parent_id, seq_no, icon, route_path, route_params, status, create_by, create_time, update_by, update_time, remark FROM sys_menu ORDER BY seq_no";
        let result: Vec<MenuRow> = sqlx::query_as(sql).fetch_all(&self.pool).await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    async fn select_menu_tree_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT DISTINCT m.id, m.menu_name, m.menu_level, m.menu_type, m.parent_id, m.seq_no, m.icon, m.route_path, m.route_params, m.status, m.create_by, m.create_time, m.update_by, m.update_time, m.remark FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id LEFT JOIN sys_user_role ur ON rm.role_id = ur.role_id WHERE ur.user_id = ? ORDER BY m.seq_no";
        let result: Vec<MenuRow> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    async fn select_menu_list(&self, menu_param: MenuParam) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let mut sql = "SELECT id, menu_name, menu_level, menu_type, parent_id, seq_no, icon, route_path, route_params, status, create_by, create_time, update_by, update_time, remark FROM sys_menu WHERE 1=1".to_string();
        let mut params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        if let Some(menu_name) = menu_param.menu_name {
            sql.push_str(" AND menu_name LIKE ?");
            params.push(Box::new(format!("%{}%", menu_name)));
        }

        if let Some(status) = menu_param.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        // 构建查询
        let mut query = sqlx::query_as::<_, MenuRow>(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(&self.pool).await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    async fn select_sys_menu_by_parent_id(&self, parent_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, menu_name, menu_level, menu_type, parent_id, seq_no, icon, route_path, route_params, status, create_by, create_time, update_by, update_time, remark FROM sys_menu WHERE parent_id = ? ORDER BY seq_no";
        let result: Vec<MenuRow> = sqlx::query_as(sql)
            .bind(parent_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT m.id, m.menu_name, m.menu_level, m.menu_type, m.parent_id, m.seq_no, m.icon, m.route_path, m.route_params, m.status, m.create_by, m.create_time, m.update_by, m.update_time, m.remark FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id WHERE rm.role_id = ? ORDER BY m.seq_no";
        let result: Vec<MenuRow> = sqlx::query_as(sql)
            .bind(role_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }
}
