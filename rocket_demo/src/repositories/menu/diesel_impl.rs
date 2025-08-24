//! 菜单数据访问层 Diesel 实现

use diesel::prelude::*;
use crate::models::Menu;
use crate::repositories::menu::menu_repository::MenuRepository;

/// 菜单表的所有字段，用于SQL查询
const MENU_FIELDS: &str = "id, name, menu_type, url, perms, icon, seq_no, status, create_by, create_time, update_by, update_time, remark, parent_id, hidden, always_show, redirect, component, href, no_cache, affix, breadcrumb, active_menu";

/// 用于获取COUNT查询结果的结构体
#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: u64,
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
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("DELETE FROM sys_menu WHERE id = ?")
            .bind::<Text, _>(id)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("菜单删除失败"));
        }

        Ok(())
    }

    /// 插入菜单记录
    async fn insert(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("INSERT INTO sys_menu (id, name, parent_id, seq_no, menu_type, url, perms, status, hidden, always_show, redirect, component, href, icon, no_cache, affix, breadcrumb, active_menu, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind::<Text, _>(&row.id)
            .bind::<Text, _>(&row.name.clone().unwrap_or_default())
            .bind::<Text, _>(&row.parent_id.clone().unwrap_or_default())
            .bind::<Integer, _>(row.seq_no.unwrap_or_default())
            .bind::<Text, _>(&row.menu_type.clone().unwrap_or_default())
            .bind::<Text, _>(&row.url.clone().unwrap_or_default())
            .bind::<Text, _>(&row.perms.clone().unwrap_or_default())
            .bind::<Integer, _>(row.status.unwrap_or_default())
            .bind::<Integer, _>(row.hidden.unwrap_or_default())
            .bind::<Integer, _>(row.always_show.unwrap_or_default())
            .bind::<Text, _>(&row.redirect.clone().unwrap_or_default())
            .bind::<Text, _>(&row.component.clone().unwrap_or_default())
            .bind::<Text, _>(&row.href.clone().unwrap_or_default())
            .bind::<Text, _>(&row.icon.clone().unwrap_or_default())
            .bind::<Integer, _>(row.no_cache.unwrap_or_default())
            .bind::<Integer, _>(row.affix.unwrap_or_default())
            .bind::<Integer, _>(row.breadcrumb.unwrap_or_default())
            .bind::<Text, _>(&row.active_menu.clone().unwrap_or_default())
            .bind::<Text, _>(&row.create_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.create_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.update_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.remark.clone().unwrap_or_default())
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("菜单插入失败"));
        }

        Ok(())
    }

    /// 选择性插入菜单记录
    async fn insert_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec!["id".to_string()];
        let mut placeholders = vec!["?".to_string()];
        let mut bindings: Vec<Box<dyn std::any::Any>> = vec![];
        
        bindings.push(Box::new(row.id.clone()) as Box<dyn std::any::Any>);

        if row.name.is_some() {
            fields.push("name".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.name.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.parent_id.is_some() {
            fields.push("parent_id".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.parent_id.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.seq_no.is_some() {
            fields.push("seq_no".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.seq_no.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.menu_type.is_some() {
            fields.push("menu_type".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.menu_type.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.url.is_some() {
            fields.push("url".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.url.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.perms.is_some() {
            fields.push("perms".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.perms.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.status.is_some() {
            fields.push("status".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.status.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.hidden.is_some() {
            fields.push("hidden".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.hidden.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.always_show.is_some() {
            fields.push("always_show".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.always_show.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.redirect.is_some() {
            fields.push("redirect".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.redirect.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.component.is_some() {
            fields.push("component".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.component.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.href.is_some() {
            fields.push("href".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.href.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.icon.is_some() {
            fields.push("icon".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.icon.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.no_cache.is_some() {
            fields.push("no_cache".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.no_cache.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.affix.is_some() {
            fields.push("affix".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.affix.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.breadcrumb.is_some() {
            fields.push("breadcrumb".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.breadcrumb.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.active_menu.is_some() {
            fields.push("active_menu".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.active_menu.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_by.is_some() {
            fields.push("create_by".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.create_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_time.is_some() {
            fields.push("create_time".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.create_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.update_by.is_some() {
            fields.push("update_by".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.update_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.update_time.is_some() {
            fields.push("update_time".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.update_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.remark.is_some() {
            fields.push("remark".to_string());
            placeholders.push("?".to_string());
            bindings.push(Box::new(row.remark.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        let sql = format!(
            "INSERT INTO sys_menu ({}) VALUES ({})",
            fields.join(", "),
            placeholders.join(", ")
        );

        let result = sql_query(&sql)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("菜单插入失败"));
        }

        Ok(())
    }

    /// 根据主键查询菜单
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        match sql_query(&format!("SELECT {} FROM sys_menu WHERE id = ?", MENU_FIELDS))
            .bind::<Text, _>(id)
            .get_result::<Menu>(&mut self.connection) {
                Ok(menu) => Ok(Some(menu)),
                Err(_) => Ok(None),
            }
    }

    /// 根据主键选择性更新菜单
    async fn update_by_primary_key_selective(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut updates = vec![];
        let mut bindings: Vec<Box<dyn std::any::Any>> = vec![];

        if row.name.is_some() {
            updates.push("name = ?".to_string());
            bindings.push(Box::new(row.name.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.parent_id.is_some() {
            updates.push("parent_id = ?".to_string());
            bindings.push(Box::new(row.parent_id.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.seq_no.is_some() {
            updates.push("seq_no = ?".to_string());
            bindings.push(Box::new(row.seq_no.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.menu_type.is_some() {
            updates.push("menu_type = ?".to_string());
            bindings.push(Box::new(row.menu_type.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.url.is_some() {
            updates.push("url = ?".to_string());
            bindings.push(Box::new(row.url.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.perms.is_some() {
            updates.push("perms = ?".to_string());
            bindings.push(Box::new(row.perms.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.status.is_some() {
            updates.push("status = ?".to_string());
            bindings.push(Box::new(row.status.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.hidden.is_some() {
            updates.push("hidden = ?".to_string());
            bindings.push(Box::new(row.hidden.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.always_show.is_some() {
            updates.push("always_show = ?".to_string());
            bindings.push(Box::new(row.always_show.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.redirect.is_some() {
            updates.push("redirect = ?".to_string());
            bindings.push(Box::new(row.redirect.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.component.is_some() {
            updates.push("component = ?".to_string());
            bindings.push(Box::new(row.component.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.href.is_some() {
            updates.push("href = ?".to_string());
            bindings.push(Box::new(row.href.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.icon.is_some() {
            updates.push("icon = ?".to_string());
            bindings.push(Box::new(row.icon.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.no_cache.is_some() {
            updates.push("no_cache = ?".to_string());
            bindings.push(Box::new(row.no_cache.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.affix.is_some() {
            updates.push("affix = ?".to_string());
            bindings.push(Box::new(row.affix.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.breadcrumb.is_some() {
            updates.push("breadcrumb = ?".to_string());
            bindings.push(Box::new(row.breadcrumb.unwrap()) as Box<dyn std::any::Any>);
        }

        if row.active_menu.is_some() {
            updates.push("active_menu = ?".to_string());
            bindings.push(Box::new(row.active_menu.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_by.is_some() {
            updates.push("create_by = ?".to_string());
            bindings.push(Box::new(row.create_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.create_time.is_some() {
            updates.push("create_time = ?".to_string());
            bindings.push(Box::new(row.create_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.update_by.is_some() {
            updates.push("update_by = ?".to_string());
            bindings.push(Box::new(row.update_by.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if row.update_time.is_some() {
            updates.push("update_time = ?".to_string());
            bindings.push(Box::new(row.update_time.unwrap().naive_utc()) as Box<dyn std::any::Any>);
        }

        if row.remark.is_some() {
            updates.push("remark = ?".to_string());
            bindings.push(Box::new(row.remark.clone().unwrap()) as Box<dyn std::any::Any>);
        }

        if updates.is_empty() {
            return Ok(());
        }

        let sql = format!(
            "UPDATE sys_menu SET {} WHERE id = ?",
            updates.join(", ")
        );

        let mut query = sql_query(&sql);
        query = query.bind::<Text, _>(&row.id);

        let result = query.execute(&mut self.connection)?;
        if result == 0 {
            return Err(Box::from("菜单更新失败"));
        }

        Ok(())
    }

    /// 根据主键更新菜单
    async fn update_by_primary_key(&self, row: &Menu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("UPDATE sys_menu SET name = ?, parent_id = ?, seq_no = ?, menu_type = ?, url = ?, perms = ?, status = ?, hidden = ?, always_show = ?, redirect = ?, component = ?, href = ?, icon = ?, no_cache = ?, affix = ?, breadcrumb = ?, active_menu = ?, create_by = ?, create_time = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?")
            .bind::<Text, _>(&row.name.clone().unwrap_or_default())
            .bind::<Text, _>(&row.parent_id.clone().unwrap_or_default())
            .bind::<Integer, _>(row.seq_no.unwrap_or_default())
            .bind::<Text, _>(&row.menu_type.clone().unwrap_or_default())
            .bind::<Text, _>(&row.url.clone().unwrap_or_default())
            .bind::<Text, _>(&row.perms.clone().unwrap_or_default())
            .bind::<Integer, _>(row.status.unwrap_or_default())
            .bind::<Integer, _>(row.hidden.unwrap_or_default())
            .bind::<Integer, _>(row.always_show.unwrap_or_default())
            .bind::<Text, _>(&row.redirect.clone().unwrap_or_default())
            .bind::<Text, _>(&row.component.clone().unwrap_or_default())
            .bind::<Text, _>(&row.href.clone().unwrap_or_default())
            .bind::<Text, _>(&row.icon.clone().unwrap_or_default())
            .bind::<Integer, _>(row.no_cache.unwrap_or_default())
            .bind::<Integer, _>(row.affix.unwrap_or_default())
            .bind::<Integer, _>(row.breadcrumb.unwrap_or_default())
            .bind::<Text, _>(&row.active_menu.clone().unwrap_or_default())
            .bind::<Text, _>(&row.create_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.create_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.update_by.clone().unwrap_or_default())
            .bind::<Timestamp, _>(row.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(&row.remark.clone().unwrap_or_default())
            .bind::<Text, _>(&row.id)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("菜单更新失败"));
        }

        Ok(())
    }

    /// 根据用户ID查询菜单列表
    async fn select_sys_menu_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query(&format!("SELECT DISTINCT {} FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id LEFT JOIN sys_user_role ur ON rm.role_id = ur.role_id WHERE ur.user_id = ? AND m.status = 1 ORDER BY m.seq_no", MENU_FIELDS))
            .bind::<Text, _>(user_id)
            .load::<Menu>(&mut self.connection)?;

        Ok(result)
    }

    /// 查询所有菜单树
    async fn select_menu_tree_all(&self) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query(&format!("SELECT {} FROM sys_menu WHERE status = 1 ORDER BY seq_no", MENU_FIELDS))
            .load::<Menu>(&mut self.connection)?;

        Ok(result)
    }

    /// 根据用户ID查询菜单树
    async fn select_menu_tree_by_user_id(&self, user_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query(&format!("SELECT DISTINCT {} FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id LEFT JOIN sys_user_role ur ON rm.role_id = ur.role_id WHERE ur.user_id = ? AND m.status = 1 ORDER BY m.seq_no", MENU_FIELDS))
            .bind::<Text, _>(user_id)
            .load::<Menu>(&mut self.connection)?;

        Ok(result)
    }

    /// 查询菜单列表
    async fn select_sys_menu_list(&self, menu_param: &Menu) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut conditions = vec![];
        let mut bindings: Vec<Box<dyn std::any::Any>> = vec![];

        if let Some(name) = &menu_param.name {
            conditions.push("name LIKE ?".to_string());
            bindings.push(Box::new(format!("%{}%", name)) as Box<dyn std::any::Any>);
        }

        if let Some(status) = menu_param.status {
            conditions.push("status = ?".to_string());
            bindings.push(Box::new(status) as Box<dyn std::any::Any>);
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!("SELECT {} FROM sys_menu {} ORDER BY seq_no", MENU_FIELDS, where_clause);

        let result = sql_query(&sql)
            .load::<Menu>(&mut self.connection)?;

        Ok(result)
    }

    /// 根据父菜单ID查询子菜单列表
    async fn select_sys_menu_by_parent_id(&self, parent_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query(&format!("SELECT {} FROM sys_menu WHERE parent_id = ? ORDER BY seq_no", MENU_FIELDS))
            .bind::<Text, _>(parent_id)
            .load::<Menu>(&mut self.connection)?;

        Ok(result)
    }

    /// 根据角色ID查询菜单ID列表
    async fn select_menu_ids_by_role_id(&self, role_id: &str) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query(&format!("SELECT {} FROM sys_menu m LEFT JOIN sys_role_menu rm ON m.id = rm.menu_id WHERE rm.role_id = ?", MENU_FIELDS))
            .bind::<Text, _>(role_id)
            .load::<Menu>(&mut self.connection)?;

        Ok(result)
    }
}