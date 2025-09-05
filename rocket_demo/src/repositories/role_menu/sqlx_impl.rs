use sqlx::FromRow;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;
use std::sync::OnceLock;

use crate::models::RoleMenu;
use crate::repositories::role_menu::role_menu_repository::RoleMenuRepository;

// 数据库连接池
static DB_POOL: OnceLock<MySqlPool> = OnceLock::new();

/// 角色菜单关联仓库SQLx实现
#[derive(Debug)]
pub struct RoleMenuRepositorySqlxImpl {
    pool: MySqlPool,
}

// ==================== 表结构体映射 ====================
#[derive(Debug, FromRow)]
struct RoleMenuRow {
    id: String,
    role_id: Option<String>,
    menu_id: Option<String>,
    create_by: Option<String>,
    #[sqlx(rename = "create_time")]
    create_time_raw: Option<chrono::NaiveDateTime>,
}

impl From<RoleMenuRow> for RoleMenu {
    fn from(row: RoleMenuRow) -> Self {
        RoleMenu {
            id: row.id,
            role_id: row.role_id,
            menu_id: row.menu_id,
            create_by: row.create_by,
            create_time: row
                .create_time_raw
                .map(|t| chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(t, chrono::Utc)),
        }
    }
}

impl RoleMenuRepositorySqlxImpl {
    pub fn new() -> Self {
        let pool = DB_POOL.get().expect("数据库连接池未初始化").clone();
        Self { pool }
    }

    pub fn init_pool(pool: MySqlPool) {
        DB_POOL.set(pool).ok(); // 如果已经设置过，则忽略
    }
}

// ==================== SQL trait 实现 ====================
#[rocket::async_trait]
impl RoleMenuRepository for RoleMenuRepositorySqlxImpl {
    async fn delete_by_primary_key(&self, role_id: &str, menu_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

    async fn insert(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "INSERT INTO sys_role_menu (id, role_id, menu_id, create_by, create_time) VALUES (?, ?, ?, ?, ?)";

        let result = sqlx::query(sql)
            .bind(&row.id)
            .bind(&row.role_id)
            .bind(&row.menu_id)
            .bind(&row.create_by)
            .bind(row.create_time.map(|t| t.naive_utc()))
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Box::from("角色菜单插入失败"));
        }

        Ok(())
    }

    async fn insert_selective(&self, row: &RoleMenu) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建动态SQL
        let mut fields = vec![];
        let mut placeholders = vec![];
        let mut params: Vec<&(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)> = vec![];

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

        let sql = format!("INSERT INTO sys_role_menu ({}) VALUES ({})", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in params {
            query = query.bind(param);
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("角色菜单插入失败"));
        }

        Ok(())
    }

    async fn select_role_menu_by_role_id(&self, role_id: &str) -> Result<Vec<RoleMenu>, Box<dyn std::error::Error + Send + Sync>> {
        let sql = "SELECT id, role_id, menu_id, create_by, create_time FROM sys_role_menu WHERE role_id = ?";
        let result: Vec<RoleMenuRow> = sqlx::query_as(sql)
            .bind(role_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(result.into_iter().map(RoleMenu::from).collect())
    }

    async fn batch_insert(&self, list: &[RoleMenu]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let mut fields = vec![];
        let mut placeholders = vec![];

        fields.push("id");
        fields.push("role_id");
        fields.push("menu_id");
        fields.push("create_by");
        fields.push("create_time");

        let mut query_params: Vec<Box<(dyn sqlx::Encode<'_, sqlx::MySql> + Send + Sync)>> = vec![];

        for role_menu in list {
            placeholders.push("(?, ?, ?, ?, ?)".to_string());
            query_params.push(Box::new(&role_menu.id));
            query_params.push(Box::new(&role_menu.role_id));
            query_params.push(Box::new(&role_menu.menu_id));
            query_params.push(Box::new(&role_menu.create_by));
            query_params.push(Box::new(role_menu.create_time.map(|t| t.naive_utc())));
        }

        let sql = format!("INSERT INTO sys_role_menu ({}) VALUES {}", fields.join(", "), placeholders.join(", "));

        let mut query = sqlx::query(&sql);
        for param in &query_params {
            query = query.bind(param.as_ref());
        }

        let result = query.execute(&self.pool).await?;
        if result.rows_affected() == 0 {
            return Err(Box::from("角色菜单批量插入失败"));
        }

        Ok(())
    }

    async fn batch_delete_by_role_id_and_menu_ids(&self, role_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = list.iter().map(|_| "?".to_string()).collect();
        let sql = format!("DELETE FROM sys_role_menu WHERE role_id = ? AND menu_id IN ({})", placeholders.join(","));

        let mut query = sqlx::query(&sql);
        query = query.bind(role_id);
        for menu_id in list {
            query = query.bind(menu_id);
        }

        query.execute(&self.pool).await?;
        Ok(())
    }

    async fn delete_by_role_id(&self, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let sql = "DELETE FROM sys_role_menu WHERE role_id = ?";
        sqlx::query(sql).bind(role_id).execute(&self.pool).await?;
        Ok(())
    }
}
