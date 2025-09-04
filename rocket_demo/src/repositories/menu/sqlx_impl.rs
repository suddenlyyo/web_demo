//! 菜单数据访问层SQLx实现

use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::models::Menu;
use crate::models::constants::MENU_FIELDS;
use crate::repositories::menu::menu_repository::MenuRepository;

/// SQLx的菜单实体映射
#[derive(Debug, FromRow)]
struct MenuRow {
    id: String,
    name: String,
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
    create_time_raw: Option<NaiveDateTime>,
    update_by: Option<String>,
    #[sqlx(rename = "update_time")]
    update_time_raw: Option<NaiveDateTime>,
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
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            update_by: row.update_by,
            update_time: row
                .update_time_raw
                .map(|t| DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            remark: row.remark,
        }
    }
}

/// SQLx实现的菜单数据访问
#[derive(Debug)]
pub struct MenuRepositorySqlxImpl {
    pool: Arc<MySqlPool>,
}

impl MenuRepositorySqlxImpl {
    /// 创建新的菜单数据访问实例
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool: Arc::new(pool) }
    }
}

#[rocket::async_trait]
impl MenuRepository for MenuRepositorySqlxImpl {
    /// 根据主键查询菜单
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Menu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_menu WHERE id = ?", MENU_FIELDS);
        let result: Option<MenuRow> = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(result.map(Menu::from))
    }

    /// 根据主键删除菜单
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_menu WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("菜单删除失败"));
        }

        Ok(())
    }

    /// 插入菜单记录
    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "INSERT INTO sys_menu (id, name, parent_id, seq_no, menu_type, url, perms, status, hidden, always_show, redirect, component, href, icon, no_cache, affix, breadcrumb, active_menu, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.name)
            .bind(&row.parent_id)
            .bind(row.seq_no)
            .bind(&row.menu_type)
            .bind(&row.url)
            .bind(&row.perms)
            .bind(row.status)
            .bind(row.hidden)
            .bind(row.always_show)
            .bind(&row.redirect)
            .bind(&row.component)
            .bind(&row.href)
            .bind(&row.icon)
            .bind(row.no_cache)
            .bind(row.affix)
            .bind(row.breadcrumb)
            .bind(&row.active_menu)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("菜单插入失败"));
        }

        Ok(())
    }

    /// 选择性插入菜单记录
    async fn insert_selective(&self, row: &Menu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

        fields.push("id");
        placeholders.push("?");
        params.push(&row.id);

        if row.name.is_some() {
            fields.push("name");
            placeholders.push("?");
            params.push(&row.name);
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

        if row.menu_type.is_some() {
            fields.push("menu_type");
            placeholders.push("?");
            params.push(&row.menu_type);
        }

        if row.url.is_some() {
            fields.push("url");
            placeholders.push("?");
            params.push(&row.url);
        }

        if row.perms.is_some() {
            fields.push("perms");
            placeholders.push("?");
            params.push(&row.perms);
        }

        if row.status.is_some() {
            fields.push("status");
            placeholders.push("?");
            params.push(&row.status);
        }

        if row.hidden.is_some() {
            fields.push("hidden");
            placeholders.push("?");
            params.push(&row.hidden);
        }

        if row.always_show.is_some() {
            fields.push("always_show");
            placeholders.push("?");
            params.push(&row.always_show);
        }

        if row.redirect.is_some() {
            fields.push("redirect");
            placeholders.push("?");
            params.push(&row.redirect);
        }

        if row.component.is_some() {
            fields.push("component");
            placeholders.push("?");
            params.push(&row.component);
        }

        if row.href.is_some() {
            fields.push("href");
            placeholders.push("?");
            params.push(&row.href);
        }

        if row.icon.is_some() {
            fields.push("icon");
            placeholders.push("?");
            params.push(&row.icon);
        }

        if row.no_cache.is_some() {
            fields.push("no_cache");
            placeholders.push("?");
            params.push(&row.no_cache);
        }

        if row.affix.is_some() {
            fields.push("affix");
            placeholders.push("?");
            params.push(&row.affix);
        }

        if row.breadcrumb.is_some() {
            fields.push("breadcrumb");
            placeholders.push("?");
            params.push(&row.breadcrumb);
        }

        if row.active_menu.is_some() {
            fields.push("active_menu");
            placeholders.push("?");
            params.push(&row.active_menu);
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

    /// 根据主键查询菜单
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Menu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_menu WHERE id = ?", MENU_FIELDS);
        let result = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(row) => {
                let menu = DbMapper::map_to_menu(&row)?;
                Ok(Some(menu))
            },
            None => Ok(None),
        }
    }

    /// 根据主键选择性更新菜单
    async fn update_by_primary_key_selective(&self, row: &Menu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)> = vec![];

        if row.name.is_some() {
            updates.push("name = ?");
            params.push(&row.name);
        }

        if row.parent_id.is_some() {
            updates.push("parent_id = ?");
            params.push(&row.parent_id);
        }

        if row.seq_no.is_some() {
            updates.push("seq_no = ?");
            params.push(&row.seq_no);
        }

        if row.menu_type.is_some() {
            updates.push("menu_type = ?");
            params.push(&row.menu_type);
        }

        if row.url.is_some() {
            updates.push("url = ?");
            params.push(&row.url);
        }

        if row.perms.is_some() {
            updates.push("perms = ?");
            params.push(&row.perms);
        }

        if row.status.is_some() {
            updates.push("status = ?");
            params.push(&row.status);
        }

        if row.hidden.is_some() {
            updates.push("hidden = ?");
            params.push(&row.hidden);
        }

        if row.always_show.is_some() {
            updates.push("always_show = ?");
            params.push(&row.always_show);
        }

        if row.redirect.is_some() {
            updates.push("redirect = ?");
            params.push(&row.redirect);
        }

        if row.component.is_some() {
            updates.push("component = ?");
            params.push(&row.component);
        }

        if row.href.is_some() {
            updates.push("href = ?");
            params.push(&row.href);
        }

        if row.icon.is_some() {
            updates.push("icon = ?");
            params.push(&row.icon);
        }

        if row.no_cache.is_some() {
            updates.push("no_cache = ?");
            params.push(&row.no_cache);
        }

        if row.affix.is_some() {
            updates.push("affix = ?");
            params.push(&row.affix);
        }

        if row.breadcrumb.is_some() {
            updates.push("breadcrumb = ?");
            params.push(&row.breadcrumb);
        }

        if row.active_menu.is_some() {
            updates.push("active_menu = ?");
            params.push(&row.active_menu);
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
            return Ok(());
        }

        let sql = format!("UPDATE sys_menu SET {} WHERE id = ?", updates.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }
        query = query.bind(&row.id);

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("菜单更新失败"));
        }

        Ok(())
    }

    /// 根据主键更新菜单
    async fn update_by_primary_key(&self, row: &Menu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_menu SET name = ?, parent_id = ?, seq_no = ?, menu_type = ?, url = ?, perms = ?, status = ?, hidden = ?, always_show = ?, redirect = ?, component = ?, href = ?, icon = ?, no_cache = ?, affix = ?, breadcrumb = ?, active_menu = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";

        let result = sqlx::query(sql)
            .bind(&row.name)
            .bind(&row.parent_id)
            .bind(row.seq_no)
            .bind(&row.menu_type)
            .bind(&row.url)
            .bind(&row.perms)
            .bind(row.status)
            .bind(row.hidden)
            .bind(row.always_show)
            .bind(&row.redirect)
            .bind(&row.component)
            .bind(&row.href)
            .bind(&row.icon)
            .bind(row.no_cache)
            .bind(row.affix)
            .bind(row.breadcrumb)
            .bind(&row.active_menu)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .bind(&row.update_by)
            .bind(row.update_time.map(|t| t.naive_utc()))
            .bind(&row.remark)
            .bind(&row.id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("菜单更新失败"));
        }

        Ok(())
    }

    /// 根据用户ID查询菜单列表
    async fn select_sys_menu_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!(
            "SELECT DISTINCT {} FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id LEFT JOIN sys_user_role ur ON rm.role_id = ur.role_id WHERE ur.user_id = ? AND m.status = 1 ORDER BY m.seq_no",
            MENU_FIELDS
        );

        let rows = sqlx::query(&sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let menus: Result<Vec<Menu>, _> = rows.iter().map(|row| DbMapper::map_to_menu(row)).collect();

        Ok(menus?)
    }

    /// 查询所有菜单树
    async fn select_menu_tree_all(&self) -> Result<Vec<Menu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_menu WHERE status = 1 ORDER BY seq_no", MENU_FIELDS);
        let result: Vec<MenuRow> = sqlx::query_as(&sql).fetch_all(self.pool.as_ref()).await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    /// 根据用户ID查询菜单树
    async fn select_menu_tree_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!(
            "SELECT DISTINCT {} FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id LEFT JOIN sys_user_role ur ON rm.role_id = ur.role_id WHERE ur.user_id = ? AND m.status = 1 ORDER BY m.seq_no",
            MENU_FIELDS
        );
        let result: Vec<MenuRow> = sqlx::query_as(&sql)
            .bind(user_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    /// 查询菜单列表
    async fn select_sys_menu_list(&self, menu_param: &Menu) -> Result<Vec<Menu>, Box<dyn StdError + Send + Sync>> {
        let mut sql = format!("SELECT {} FROM sys_menu WHERE 1=1", MENU_FIELDS);
        let mut params: Vec<Box<(dyn sqlx::Encode<sqlx::MySql, sqlx::types::database::MySqlTypeInfo> + Send + Sync)>> = vec![];

        if let Some(name) = &menu_param.name {
            sql.push_str(" AND name LIKE ?");
            params.push(Box::new(format!("%{}%", name)));
        }

        if let Some(status) = menu_param.status {
            sql.push_str(" AND status = ?");
            params.push(Box::new(status));
        }

        sql.push_str(" ORDER BY seq_no");

        let mut query = sqlx::query_as::<_, MenuRow>(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(self.pool.as_ref()).await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    /// 根据父菜单ID查询子菜单列表
    async fn select_sys_menu_by_parent_id(&self, parent_id: &str) -> Result<Vec<Menu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_menu WHERE parent_id = ? ORDER BY seq_no", MENU_FIELDS);
        let result: Vec<MenuRow> = sqlx::query_as(&sql)
            .bind(parent_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }

    /// 根据角色ID查询菜单ID列表
    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> Result<Vec<Menu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id WHERE rm.role_id = ?", MENU_FIELDS);
        let result: Vec<MenuRow> = sqlx::query_as(&sql)
            .bind(role_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(result.into_iter().map(Menu::from).collect())
    }
}
