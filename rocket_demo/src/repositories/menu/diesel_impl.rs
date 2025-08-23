//! 菜单数据访问层 Diesel 实现

use diesel::sql_types::{BigInt, Integer, Text, Timestamp};
use diesel::{QueryableByName, RunQueryDsl, sql_query};

use crate::models::Menu;
use crate::repositories::menu::menu_repository::MenuRepository;
use common_wrapper::PageInfo;

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
    /// 根据ID获取菜单信息
    async fn get_menu_by_id(&self, id: &str) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询菜单信息
        let menu_query = sql_query("SELECT id, name, permission, component, seq_no, icon, path, status, create_by, create_time, update_by, update_time, remark FROM sys_menu WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<Menu>(&mut self.connection)?;

        Ok(menu_query)
    }

    /// 获取菜单列表
    async fn list_menus(&self) -> Result<Vec<Menu>, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel查询菜单列表
        let menus_query = sql_query("SELECT id, name, permission, component, seq_no, icon, path, status, create_by, create_time, update_by, update_time, remark FROM sys_menu ORDER BY seq_no").load::<Menu>(&mut self.connection)?;

        Ok(menus_query)
    }

    /// 分页查询菜单列表
    async fn list_menus_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Menu>, u64, u64), Box<dyn std::error::Error + Send + Sync>> {
        // 处理分页参数
        let current_page = page_num.unwrap_or(PageInfo::DEFAULT_CURRENT_PAGE);
        let page_size = page_size
            .unwrap_or(PageInfo::DEFAULT_PAGE_SIZE)
            .min(PageInfo::MAX_PAGE_SIZE);
        let offset = (current_page - 1) * page_size;

        // 构建查询SQL
        let sql = "SELECT id, name, permission, component, seq_no, icon, path, status, create_by, create_time, update_by, update_time, remark FROM sys_menu ORDER BY seq_no LIMIT ? OFFSET ?";

        // 构建统计查询
        let count_sql = "SELECT COUNT(*) as count FROM sys_menu";

        // 查询总记录数
        let count_result = sql_query(count_sql).get_result::<CountResult>(&mut self.connection)?;
        let total_count = count_result.count;

        // 计算总页数
        let total_pages = (total_count + page_size - 1) / page_size;

        // 查询当前页数据
        let menus_result = sql_query(&sql)
            .bind::<BigInt, _>(page_size as i64)
            .bind::<BigInt, _>(offset as i64)
            .load::<Menu>(&mut self.connection)?;

        Ok((menus_result, total_count, total_pages))
    }

    /// 新增菜单
    async fn add_menu(&self, menu: Menu) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel新增菜单
        let insert_query = sql_query("INSERT INTO sys_menu (id, name, permission, component, seq_no, icon, path, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind::<Text, _>(menu.id)
            .bind::<Text, _>(menu.name)
            .bind::<Text, _>(menu.permission.unwrap_or_default())
            .bind::<Text, _>(menu.component.unwrap_or_default())
            .bind::<Integer, _>(menu.seq_no)
            .bind::<Text, _>(menu.icon.unwrap_or_default())
            .bind::<Text, _>(menu.path.unwrap_or_default())
            .bind::<Integer, _>(menu.status)
            .bind::<Text, _>(menu.create_by.unwrap_or_default())
            .bind::<Timestamp, _>(menu.create_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(menu.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(menu.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(menu.remark.unwrap_or_default());

        insert_query.execute(&mut self.connection)?;

        Ok(menu)
    }

    /// 修改菜单
    async fn update_menu(&self, menu: Menu) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改菜单
        let update_query = sql_query("UPDATE sys_menu SET name = ?, permission = ?, component = ?, seq_no = ?, icon = ?, path = ?, status = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?")
            .bind::<Text, _>(menu.name)
            .bind::<Text, _>(menu.permission.unwrap_or_default())
            .bind::<Text, _>(menu.component.unwrap_or_default())
            .bind::<Integer, _>(menu.seq_no)
            .bind::<Text, _>(menu.icon.unwrap_or_default())
            .bind::<Text, _>(menu.path.unwrap_or_default())
            .bind::<Integer, _>(menu.status)
            .bind::<Text, _>(menu.update_by.unwrap_or_default())
            .bind::<Timestamp, _>(menu.update_time.unwrap_or_default().naive_utc())
            .bind::<Text, _>(menu.remark.unwrap_or_default())
            .bind::<Text, _>(menu.id);

        update_query.execute(&mut self.connection)?;

        Ok(menu)
    }

    /// 删除菜单
    async fn delete_menu(&self, id: &str) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel删除菜单
        let delete_query = sql_query("DELETE FROM sys_menu WHERE id = ?").bind::<Text, _>(id);
        delete_query.execute(&mut self.connection)?;

        // 查询删除的菜单信息（模拟返回）
        let menu = Menu { id: id.to_string(), ..Default::default() };

        Ok(menu)
    }

    /// 修改菜单状态
    async fn update_menu_status(&self, id: &str, status: i32) -> Result<Menu, Box<dyn std::error::Error + Send + Sync>> {
        // 使用Diesel修改菜单状态
        let update_query = sql_query("UPDATE sys_menu SET status = ?, update_time = CURRENT_TIMESTAMP WHERE id = ?")
            .bind::<Integer, _>(status)
            .bind::<Text, _>(id);

        update_query.execute(&mut self.connection)?;

        // 查询更新后的菜单信息
        let menu_query = sql_query("SELECT id, name, permission, component, seq_no, icon, path, status, create_by, create_time, update_by, update_time, remark FROM sys_menu WHERE id = ?")
            .bind::<Text, _>(id)
            .get_result::<Menu>(&mut self.connection)?;

        Ok(menu_query)
    }
}
