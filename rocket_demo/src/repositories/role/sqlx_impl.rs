//! 角色数据访问层SQLx实现

use chrono::{NaiveDateTime, Utc};
use sqlx::Row;
use sqlx::mysql::{MySqlPool, MySqlRow};
use std::error::Error as StdError;

use crate::models::Role;
use crate::repositories::role::role_repository::RoleRepository;
use common_wrapper::PageInfo;

/// 角色表的所有字段，用于SQL查询
pub const ROLE_FIELDS: &str = "id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark";

/// 数据库映射器
struct DbMapper;

impl DbMapper {
    /// 将数据库行映射为角色对象
    fn map_to_role(row: &MySqlRow) -> Result<Role, sqlx::Error> {
        Ok(Role {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            role_key: row.try_get("role_key")?,
            seq_no: row.try_get("seq_no")?,
            status: row.try_get("status")?,
            create_by: row.try_get("create_by")?,
            create_time: row
                .try_get::<Option<NaiveDateTime>, _>("create_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            update_by: row.try_get("update_by")?,
            update_time: row
                .try_get::<Option<NaiveDateTime>, _>("update_time")?
                .map(|t| chrono::DateTime::<Utc>::from_naive_utc_and_offset(t, Utc)),
            remark: row.try_get("remark")?,
        })
    }
}

/// SQLx实现的角色数据访问
#[derive(Debug)]
pub struct RoleRepositorySqlxImpl {
    pool: MySqlPool,
}

impl RoleRepositorySqlxImpl {
    /// 创建新的角色数据访问实例
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// 从数据库URL创建连接池并初始化Repository
    pub async fn from_database_url(database_url: &str) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        let pool = MySqlPool::connect(database_url).await?;
        Ok(Self::new(pool))
    }

    /// 获取单个角色记录
    async fn get_role_record(&self, id: &str) -> Result<Option<Role>, sqlx::Error> {
        sqlx::query(&format!("SELECT {} FROM sys_role WHERE id = ?", ROLE_FIELDS))
            .bind(id)
            .map(|row: MySqlRow| DbMapper::map_to_role(&row))
            .fetch_optional(&self.pool)
            .await?
            .transpose()
    }

    /// 获取角色列表记录
    async fn get_roles_records(&self) -> Result<Vec<Role>, sqlx::Error> {
        sqlx::query(&format!("SELECT {} FROM sys_role ORDER BY seq_no", ROLE_FIELDS))
            .map(|row: MySqlRow| DbMapper::map_to_role(&row))
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }

    /// 分页获取角色记录
    async fn get_paged_roles_records(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Role>, u64, u64), sqlx::Error> {
        let page_info = PageInfo::new(page_num, page_size);
        let _current_page = page_info.get_current_page_num();
        let page_size_value = page_info.get_page_size();
        let offset = page_info.get_page_offset();

        // 查询总数
        let count_query = sqlx::query(&format!("SELECT COUNT(*) as count FROM sys_role"))
            .fetch_one(&self.pool)
            .await?;

        let total_count = u64::try_from(count_query.get::<i64, _>("count")).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let total_pages = (total_count as f64 / page_size_value as f64).ceil() as u64;

        // 分页查询
        let roles_query = format!("SELECT {} FROM sys_role ORDER BY seq_no LIMIT ? OFFSET ?", ROLE_FIELDS);
        let roles = sqlx::query(&roles_query)
            .bind(page_size_value as i64)
            .bind(offset as i64)
            .map(|row: MySqlRow| DbMapper::map_to_role(&row))
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;

        Ok((roles, total_count, total_pages))
    }
}

#[rocket::async_trait]
impl RoleRepository for RoleRepositorySqlxImpl {
    /// 根据ID获取角色信息
    async fn get_role_by_id(&self, id: &str) -> Result<Role, Box<dyn StdError + Send + Sync>> {
        match self.get_role_record(id).await? {
            Some(role) => Ok(role),
            None => Err(Box::from("角色不存在")),
        }
    }

    /// 获取角色列表
    async fn list_roles(&self) -> Result<Vec<Role>, Box<dyn StdError + Send + Sync>> {
        self.get_roles_records().await.map_err(|e| e.into())
    }

    /// 分页查询角色列表
    async fn list_roles_by_page(&self, page_num: Option<u64>, page_size: Option<u64>) -> Result<(Vec<Role>, u64, u64), Box<dyn StdError + Send + Sync>> {
        self.get_paged_roles_records(page_num, page_size)
            .await
            .map_err(|e| e.into())
    }

    /// 新增角色
    async fn add_role(&self, role: Role) -> Result<Role, Box<dyn StdError + Send + Sync>> {
        // 构建SQL查询
        let sql = "INSERT INTO sys_role (id, name, role_key, seq_no, status, create_by, create_time, update_by, update_time, remark) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        // 执行插入操作
        let result = sqlx::query(sql)
            .bind(&role.id)
            .bind(&role.name)
            .bind(&role.role_key)
            .bind(role.seq_no)
            .bind(role.status)
            .bind(&role.create_by)
            .bind(role.create_time.map(|t| t.naive_utc()))
            .bind(&role.update_by)
            .bind(role.update_time.map(|t| t.naive_utc()))
            .bind(&role.remark)
            .execute(&self.pool)
            .await?;

        // 验证影响行数
        if result.rows_affected() == 0 {
            return Err(Box::from("角色插入失败"));
        }

        // 返回插入的角色
        Ok(role)
    }

    /// 修改角色
    async fn update_role(&self, role: Role) -> Result<Role, Box<dyn StdError + Send + Sync>> {
        // 构建SQL查询
        let sql = "UPDATE sys_role SET name = ?, role_key = ?, seq_no = ?, status = ?, update_by = ?, update_time = ?, remark = ? WHERE id = ?";

        // 执行更新操作
        let result = sqlx::query(sql)
            .bind(&role.name)
            .bind(&role.role_key)
            .bind(role.seq_no)
            .bind(role.status)
            .bind(&role.update_by)
            .bind(role.update_time.map(|t| t.naive_utc()))
            .bind(&role.remark)
            .bind(&role.id)
            .execute(&self.pool)
            .await?;

        // 验证影响行数
        if result.rows_affected() == 0 {
            return Err(Box::from("角色更新失败"));
        }

        // 返回更新的角色
        Ok(role)
    }

    /// 删除角色
    async fn delete_role(&self, id: &str) -> Result<Role, Box<dyn StdError + Send + Sync>> {
        // 构建SQL查询
        let sql = "DELETE FROM sys_role WHERE id = ?";

        // 获取要删除的角色
        let role_to_delete = self.get_role_record(id).await?.ok_or("角色不存在")?;

        // 执行删除操作
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;

        // 验证影响行数
        if result.rows_affected() == 0 {
            return Err(Box::from("角色删除失败"));
        }

        // 返回删除的角色
        Ok(role_to_delete)
    }

    /// 修改角色状态
    async fn update_role_status(&self, id: &str, status: i32) -> Result<Role, Box<dyn StdError + Send + Sync>> {
        // 构建SQL查询
        let sql = "UPDATE sys_role SET status = ?, update_by = ?, update_time = ? WHERE id = ?";

        // 获取要更新的角色
        let mut role_to_update = self.get_role_record(id).await?.ok_or("角色不存在")?;

        // 获取当前时间
        let now = Utc::now().naive_utc();

        // 执行更新操作
        let result = sqlx::query(sql)
            .bind(status)
            .bind(&role_to_update.update_by)
            .bind(&now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        // 验证影响行数
        if result.rows_affected() == 0 {
            return Err(Box::from("角色状态更新失败"));
        }

        // 更新内存中的角色状态
        role_to_update.status = Some(status);
        role_to_update.update_time = Some(chrono::DateTime::<Utc>::from_naive_utc_and_offset(now, Utc));

        // 返回更新的角色
        Ok(role_to_update)
    }
}
