//! 角色菜单数据访问层SQLx实现

use std::error::Error as StdError;

use sqlx::{MySqlPool, Row};

use crate::models::RoleMenu;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;

/// 角色菜单仓库SQLx实现
#[derive(Debug)]
pub struct RoleMenuRepositorySqlxImpl {
    pool: MySqlPool,
}

impl RoleMenuRepositorySqlxImpl {
    /// 创建新的角色菜单数据访问实例
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[rocket::async_trait]
impl RoleMenuRepository for RoleMenuRepositorySqlxImpl {
    /// 根据角色ID和菜单ID删除角色菜单
    async fn delete_by_primary_key(&self, role_id: &str, menu_id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_role_menu WHERE role_id = ? AND menu_id = ?";
        let result = sqlx::query(sql)
            .bind(role_id)
            .bind(menu_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("角色菜单删除失败"));
        }

        Ok(())
    }

    /// 插入角色菜单记录
    async fn insert(&self, row: &RoleMenu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "INSERT INTO sys_role_menu (role_id, menu_id) VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(&row.role_id)
            .bind(&row.menu_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("角色菜单插入失败"));
        }

        Ok(())
    }

    /// 选择性插入角色菜单记录
    async fn insert_selective(&self, row: &RoleMenu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        self.insert(row).await
    }

    /// 根据角色ID查询角色菜单列表
    async fn select_role_menu_by_role_id(&self, role_id: &str) -> Result<Vec<RoleMenu>, Box<dyn StdError + Send + Sync>> {
        let sql = "SELECT role_id, menu_id FROM sys_role_menu WHERE role_id = ?";
        let rows = sqlx::query(sql)
            .bind(role_id)
            .fetch_all(&self.pool)
            .await?;

        let role_menus: Result<Vec<RoleMenu>, _> = rows.iter().map(|row| {
            Ok(RoleMenu {
                role_id: row.try_get("role_id")?,
                menu_id: row.try_get("menu_id")?,
            })
        }).collect();

        Ok(role_menus?)
    }

    /// 批量插入角色菜单
    async fn batch_insert(&self, list: &[RoleMenu]) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        // 构建VALUES部分
        let values_placeholders: Vec<String> = (0..list.len()).map(|_| "(?, ?)".to_string()).collect();
        let sql = format!("INSERT INTO sys_role_menu (role_id, menu_id) VALUES {}", values_placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for role_menu in list {
            query = query.bind(&role_menu.role_id).bind(&role_menu.menu_id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    /// 根据角色ID和菜单ID列表批量删除角色菜单
    async fn batch_delete_by_role_id_and_menu_ids(&self, role_id: &str, list: &[String]) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = (0..list.len()).map(|_| "?".to_string()).collect();
        let sql = format!(
            "DELETE FROM sys_role_menu WHERE role_id = ? AND menu_id IN ({})",
            placeholders.join(", ")
        );

        let mut query = sqlx::query(&sql);
        query = query.bind(role_id);
        for menu_id in list {
            query = query.bind(menu_id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    /// 根据角色ID删除角色菜单
    async fn delete_by_role_id(&self, role_id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_role_menu WHERE role_id = ?";
        sqlx::query(sql)
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}