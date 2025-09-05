use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::models::RoleMenu;
use crate::models::constants::ROLE_MENU_FIELDS;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;

/// 角色菜单关联仓库SQLx实现
#[derive(Debug)]
pub struct RoleMenuRepositorySqlxImpl {
    pool: Arc<MySqlPool>,
}

/// SQLx的角色菜单实体映射
#[derive(Debug, FromRow)]
struct RoleMenuRow {
    id: String,
    role_id: Option<String>,
    menu_id: Option<String>,
}

impl From<RoleMenuRow> for RoleMenu {
    fn from(row: RoleMenuRow) -> Self {
        RoleMenu { id: row.id, role_id: row.role_id, menu_id: row.menu_id }
    }
}

impl RoleMenuRepositorySqlxImpl {
    /// 创建角色菜单关联仓库SQLx实现
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool: Arc::new(pool) }
    }
}

#[rocket::async_trait]
impl RoleMenuRepository for RoleMenuRepositorySqlxImpl {
    /// 根据主键删除角色菜单关联
    async fn delete_by_primary_key(&self, role_id: &str, menu_id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_role_menu WHERE role_id = ? AND menu_id = ?";
        sqlx::query(sql)
            .bind(role_id)
            .bind(menu_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    /// 插入角色菜单关联记录
    async fn insert(&self, row: &RoleMenu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "INSERT INTO sys_role_menu (id, role_id, menu_id) VALUES (?, ?, ?)";
        sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.role_id)
            .bind(&row.menu_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    /// 选择性插入角色菜单关联记录
    async fn insert_selective(&self, row: &RoleMenu) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<sqlx::MySql, sqlx::MySqlTypeInfo> + Send + Sync)> = vec![];

        fields.push("id");
        placeholders.push("?");
        params.push(&row.id);

        if row.role_id.is_some() {
            fields.push("role_id");
            placeholders.push("?");
            params.push(&row.role_id);
        }

        if row.menu_id.is_some() {
            fields.push("menu_id");
            placeholders.push("?");
            params.push(&row.menu_id);
        }

        let sql = format!("INSERT INTO sys_role_menu ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let query = sqlx::query(&sql).bind_all(params);
        query.execute(self.pool.as_ref()).await?;
        Ok(())
    }

    /// 根据主键查询角色菜单关联
    /*async fn select_by_id(&self, id: &str) -> Result<Option<RoleMenu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_role_menu WHERE id = ?", ROLE_MENU_FIELDS);
        let result: Option<RoleMenuRow> = sqlx::query_as(&sql)
            .bind(id)
            .fetch_optional(self.pool.as_ref())
            .await?;
        Ok(result.map(RoleMenu::from))
    }

    /// 查询角色菜单关联列表
    async fn select_list(&self, role_menu_param: crate::services::params::user_param::RoleMenuParam) -> Result<Vec<RoleMenu>, Box<dyn StdError + Send + Sync>> {
        let mut sql = format!("SELECT {} FROM sys_role_menu WHERE 1=1", ROLE_MENU_FIELDS);
        let mut params: Vec<Box<(dyn sqlx::Encode<sqlx::MySql, sqlx::MySqlTypeInfo> + Send + Sync)>> = vec![];

        if let Some(role_id) = &role_menu_param.role_id {
            sql.push_str(" AND role_id = ?");
            params.push(Box::new(role_id.clone()));
        }

        if let Some(menu_id) = &role_menu_param.menu_id {
            sql.push_str(" AND menu_id = ?");
            params.push(Box::new(menu_id.clone()));
        }

        sql.push_str(" ORDER BY id");

        let mut query = sqlx::query_as::<_, RoleMenuRow>(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }

        let result = query.fetch_all(self.pool.as_ref()).await?;
        Ok(result.into_iter().map(RoleMenu::from).collect())
    }

    /// 根据主键更新角色菜单关联
    async fn update_by_id(&self, row: &RoleMenu) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "UPDATE sys_role_menu SET role_id = ?, menu_id = ? WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(&row.role_id)
            .bind(&row.menu_id)
            .bind(&row.id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }

    /// 根据主键选择性更新角色菜单关联
    async fn update_by_id_selective(&self, row: &RoleMenu) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let mut sets = vec![];
        let mut params: Vec<Box<(dyn sqlx::Encode<sqlx::MySql, sqlx::MySqlTypeInfo> + Send + Sync)>> = vec![];

        if row.role_id.is_some() {
            sets.push("role_id = ?");
            params.push(Box::new(&row.role_id));
        }

        if row.menu_id.is_some() {
            sets.push("menu_id = ?");
            params.push(Box::new(&row.menu_id));
        }

        if sets.is_empty() {
            return Ok(0);
        }

        let mut sql = format!("UPDATE sys_role_menu SET {}", sets.join(", "));
        sql.push_str(" WHERE id = ?");
        params.push(Box::new(&row.id));

        let mut query = sqlx::query(&sql);
        for param in &params {
            query = query.bind(param.as_ref());
        }
        query = query.bind(&row.id);

        let result = query.execute(self.pool.as_ref()).await?;
        Ok(result.rows_affected())
    }

    /// 根据主键删除角色菜单关联
    async fn delete_by_id(&self, id: &str) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_role_menu WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected())
    }*/

    /// 根据角色ID查询角色菜单列表
    async fn select_role_menu_by_role_id(&self, role_id: &str) -> Result<Vec<RoleMenu>, Box<dyn StdError + Send + Sync>> {
        let sql = format!("SELECT {} FROM sys_role_menu WHERE role_id = ?", ROLE_MENU_FIELDS);
        let result: Vec<RoleMenuRow> = sqlx::query_as(&sql)
            .bind(role_id)
            .fetch_all(self.pool.as_ref())
            .await?;
        Ok(result.into_iter().map(RoleMenu::from).collect())
    }

    /// 批量插入角色菜单
    async fn batch_insert(&self, list: &[RoleMenu]) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let mut fields = vec![];
        let mut placeholders = vec![];

        fields.push("id");
        fields.push("role_id");
        fields.push("menu_id");

        let sql = format!("INSERT INTO sys_role_menu ({}) VALUES ({})", fields.join(", "), "?, ?, ?");

        for role_menu in list {
            sqlx::query(&sql)
                .bind(&role_menu.id)
                .bind(&role_menu.role_id)
                .bind(&role_menu.menu_id)
                .execute(self.pool.as_ref())
                .await?;
        }

        Ok(())
    }

    /// 根据角色ID和菜单ID列表批量删除角色菜单
    async fn batch_delete_by_role_id_and_menu_ids(&self, role_id: &str, list: &[String]) -> Result<(), Box<dyn StdError + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = list.iter().map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM sys_role_menu WHERE role_id = ? AND menu_id IN ({})", placeholders.join(","));

        let mut query = sqlx::query(&sql).bind(role_id);
        for menu_id in list {
            query = query.bind(menu_id);
        }

        query.execute(self.pool.as_ref()).await?;
        Ok(())
    }

    /// 根据角色ID删除角色菜单
    async fn delete_by_role_id(&self, role_id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let sql = "DELETE FROM sys_role_menu WHERE role_id = ?";
        sqlx::query(sql)
            .bind(role_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
