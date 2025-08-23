//! 菜单数据访问层SQLx实现

use chrono::Utc;
use sqlx::Row;
use sqlx::mysql::MySqlPool;
use std::error::Error as StdError;

use crate::models::Menu;
use crate::repositories::menu::menu_repository::MenuRepository;
use common_wrapper::PageInfo;

/// 菜单表的所有字段，用于SQL查询
const MENU_FIELDS: &str = "id, name, menu_type, url, perms, icon, seq_no, status, create_by, create_time, update_by, update_time, remark, parent_id, hidden, always_show, redirect, component, href, no_cache, affix, breadcrumb, active_menu";

/// 数据库映射器
struct DbMapper;

impl DbMapper {
    /// 将数据库行映射为菜单对象
    fn map_to_menu(row: &sqlx::mysql::MySqlRow) -> Result<Menu, sqlx::Error> {
        Ok(Menu {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            parent_id: row.try_get("parent_id")?,
            seq_no: row.try_get("seq_no")?,
            menu_type: row.try_get("menu_type")?,
            url: row.try_get("url")?,
            perms: row.try_get("perms")?,
            status: row.try_get("status")?,
            hidden: row.try_get("hidden")?,
            always_show: row.try_get("always_show")?,
            redirect: row.try_get("redirect")?,
            component: row.try_get("component")?,
            href: row.try_get("href")?,
            icon: row.try_get("icon")?,
            no_cache: row.try_get("no_cache")?,
            affix: row.try_get("affix")?,
            breadcrumb: row.try_get("breadcrumb")?,
            active_menu: row.try_get("active_menu")?,
            create_by: row.try_get("create_by")?,
            create_time: row
                .try_get::<Option<chrono::NaiveDateTime>, _>("create_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            update_by: row.try_get("update_by")?,
            update_time: row
                .try_get::<Option<chrono::NaiveDateTime>, _>("update_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            remark: row.try_get("remark")?,
        })
    }
}

/// SQLx实现的菜单数据访问
#[derive(Debug)]
pub struct MenuRepositorySqlxImpl {
    pool: MySqlPool,
}

impl MenuRepositorySqlxImpl {
    /// 创建新的菜单数据访问实例
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 从数据库URL创建连接池并初始化Repository
    pub async fn from_database_url(database_url: &str) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        let pool = MySqlPool::connect(database_url).await?;
        Ok(Self::new(pool))
    }
}

#[rocket::async_trait]
impl MenuRepository for MenuRepositorySqlxImpl {
    /// 根据ID获取菜单信息
    async fn get_menu_by_id(&self, id: &str) -> Result<Menu, Box<dyn StdError + Send + Sync>> {
        let menu = sqlx::query(&format!("SELECT {} FROM sys_menu WHERE id = ?", MENU_FIELDS))
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)
            .map(|row| DbMapper::map_to_menu(&row))??;

        Ok(menu)
    }

    /// 获取菜单列表
    async fn list_menus(&self) -> Result<Vec<Menu>, Box<dyn StdError + Send + Sync>> {
        let menus_query = sqlx::query(&format!("SELECT {} FROM sys_menu ORDER BY seq_no", MENU_FIELDS))
            .fetch_all(&self.pool)
            .await?;

        let menus: Result<Vec<Menu>, _> = menus_query.iter().map(DbMapper::map_to_menu).collect();

        Ok(menus?)
    }

    /// 分页查询菜单列表
    async fn list_menus_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Menu>, u64, u64), Box<dyn StdError + Send + Sync>> {
        let page_info = PageInfo::new(page_num, page_size);
        let offset = page_info.get_page_offset();
        let page_size_value = page_info.get_page_size();

        // 查询总数
        let count_query = sqlx::query("SELECT COUNT(*) as count FROM sys_menu")
            .fetch_one(&self.pool)
            .await?;

        let total_count = u64::try_from(count_query.get::<i64, &str>("count"))?;
        let total_pages = (total_count + page_size_value - 1) / page_size_value;

        // 查询数据
        let menus_query = sqlx::query(&format!("SELECT {} FROM sys_menu ORDER BY seq_no LIMIT ? OFFSET ?", MENU_FIELDS))
            .bind(page_size_value as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await?;

        let menus: Vec<Menu> = menus_query
            .iter()
            .map(|row| DbMapper::map_to_menu(row))
            .collect::<Result<_, _>>()?;

        Ok((menus, total_count, total_pages))
    }

    /// 新增菜单
    async fn add_menu(&self, _menu: Menu) -> Result<Menu, Box<dyn StdError + Send + Sync>> {
        todo!()
    }

    async fn update_menu(&self, _menu: Menu) -> Result<Menu, Box<dyn StdError + Send + Sync>> {
        todo!()
    }

    async fn delete_menu(&self, _id: &str) -> Result<Menu, Box<dyn StdError + Send + Sync>> {
        todo!()
    }

    async fn update_menu_status(&self, _id: &str, _status: i32) -> Result<Menu, Box<dyn StdError + Send + Sync>> {
        todo!()
    }
}
