//! 角色菜单数据访问层 Diesel 实现

use diesel::{RunQueryDsl, sql_query};

use crate::models::RoleMenu;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;

/// 角色菜单数据访问 Diesel 实现
#[derive(Debug)]
pub struct RoleMenuRepositoryDieselImpl {
    connection: diesel::sqlite::SqliteConnection,
}

impl RoleMenuRepositoryDieselImpl {
    /// 创建角色菜单仓库 Diesel 实例
    pub fn new() -> Self {
        // 初始化数据库连接
        let database_url = std::env::var("DATABASE_URL").unwrap_or("data.db".to_string());
        let connection = diesel::sqlite::SqliteConnection::establish(&database_url).expect("Error connecting to SQLite database");

        Self { connection }
    }
}

#[rocket::async_trait]
impl RoleMenuRepository for RoleMenuRepositoryDieselImpl {
    /// 根据角色ID和菜单ID删除角色菜单
    async fn delete_by_primary_key(&self, role_id: &str, menu_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("DELETE FROM sys_role_menu WHERE role_id = ? AND menu_id = ?")
            .bind::<diesel::sql_types::Text, _>(role_id)
            .bind::<diesel::sql_types::Text, _>(menu_id)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("角色菜单删除失败"));
        }

        Ok(())
    }

    /// 插入角色菜单记录
    async fn insert(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("INSERT INTO sys_role_menu (role_id, menu_id) VALUES (?, ?)")
            .bind::<diesel::sql_types::Text, _>(&row.role_id)
            .bind::<diesel::sql_types::Text, _>(&row.menu_id)
            .execute(&mut self.connection)?;

        if result == 0 {
            return Err(Box::from("角色菜单插入失败"));
        }

        Ok(())
    }

    /// 选择性插入角色菜单记录
    async fn insert_selective(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.insert(row).await
    }

    /// 根据角色ID查询角色菜单列表
    async fn select_role_menu_by_role_id(&self, role_id: &str) -> Result<Vec<RoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sql_query("SELECT role_id, menu_id FROM sys_role_menu WHERE role_id = ?")
            .bind::<diesel::sql_types::Text, _>(role_id)
            .load::<RoleMenu>(&mut self.connection)?;

        Ok(result)
    }

    /// 批量插入角色菜单
    async fn batch_insert(&self, list: &[RoleMenu]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        // 构建VALUES部分
        let values_placeholders: Vec<String> = (0..list.len()).map(|_| "(?, ?)".to_string()).collect();
        let sql = format!("INSERT INTO sys_role_menu (role_id, menu_id) VALUES {}", values_placeholders.join(", "));

        let mut query = sql_query(&sql);
        for role_menu in list {
            query = query
                .bind::<diesel::sql_types::Text, _>(&role_menu.role_id)
                .bind::<diesel::sql_types::Text, _>(&role_menu.menu_id);
        }

        query.execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据角色ID和菜单ID列表批量删除角色菜单
    async fn batch_delete_by_role_id_and_menu_ids(&self, role_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = (0..list.len()).map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM sys_role_menu WHERE role_id = ? AND menu_id IN ({})", placeholders.join(", "));

        let mut query = sql_query(&sql);
        query = query.bind::<diesel::sql_types::Text, _>(role_id);
        for menu_id in list {
            query = query.bind::<diesel::sql_types::Text, _>(menu_id);
        }

        query.execute(&mut self.connection)?;
        Ok(())
    }

    /// 根据角色ID删除角色菜单
    async fn delete_by_role_id(&self, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sql_query("DELETE FROM sys_role_menu WHERE role_id = ?")
            .bind::<diesel::sql_types::Text, _>(role_id)
            .execute(&mut self.connection)?;

        Ok(())
    }
}
