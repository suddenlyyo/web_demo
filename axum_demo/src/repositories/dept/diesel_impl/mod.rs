//! Diesel实现的部门数据访问层
//!
//! 该模块提供了基于Diesel ORM的部门数据访问实现，支持类型安全的数据库操作。
//! 实现了DeptRepository trait定义的所有方法。

use crate::config::Config;
use crate::models::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;
use crate::schema::sys_dept;
use async_trait::async_trait;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sql_query;
use std::error::Error as StdError;
use std::fmt::Debug;

/// Diesel实现的部门仓储
#[derive(Debug)]
pub struct DeptRepositoryDieselImpl {
    pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl DeptRepositoryDieselImpl {
    /// 创建新的Diesel部门仓储实例（使用默认配置）
    ///
    /// # 返回值
    /// 返回新的部门仓储实例
    pub fn new() -> Self {
        // 从配置文件中读取数据库URL
        let config = Config::from_default_file().expect("无法加载配置文件");
        let database_url = config.database.url;
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create pool");

        Self { pool }
    }
}

#[async_trait]
impl DeptRepository for DeptRepositoryDieselImpl {
    // 为了方便维护和管理model和sqlx的共用,且不像官方教程一样crud都定义一个结构体,而是通过业务控制,这样更通用合理

    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // Diesel是同步ORM，这里为了适配异步trait，需要在阻塞线程中执行
        let id_value = id.to_string();
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            diesel::delete(sys_dept::table.filter(sys_dept::id.eq(id_value)))
                .execute(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            Ok::<(), Box<dyn StdError + Send + Sync>>(())
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?
        .map_err(|e| e)
    }

    /// 插入部门记录
    ///
    /// 由于Diesel是通过Option字段来实现选择性插入的ORM，
    /// 且当前Dept模型中所有字段都是Option类型，
    /// 完整插入和选择性插入的逻辑是一致的，都是根据字段是否有值来决定是否插入。
    /// 因此直接调用选择性插入方法即可，避免代码重复。
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 直接调用选择性插入方法，因为当前模型所有字段都是Option类型
        // 无论字段是否有值，insert_selective都能正确处理
        self.insert_selective(row).await
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let row = row.clone();
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            diesel::insert_into(sys_dept::table)
                .values(&row)
                .execute(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            Ok::<(), Box<dyn StdError + Send + Sync>>(())
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?
        .map_err(|e| e)
    }

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let id_value = id.to_string();
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            let result = sys_dept::table
                .filter(sys_dept::id.eq(id_value))
                .first::<Dept>(&mut conn)
                .optional()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            Ok::<Option<Dept>, Box<dyn StdError + Send + Sync>>(result)
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?
        .map_err(|e| e)
    }

    /// 根据父部门ID查询部门
    ///
    /// 使用原生SQL实现以优化性能，避免ORM可能带来的性能开销
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let parent_id_value = parent_id.to_string(); // 克隆字符串以解决生命周期问题
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            // 使用原生SQL查询以优化性能
            use crate::models::constants::DEPT_FIELDS;
            // 使用DEPT_FIELDS常量构建SQL查询
            let sql = format!("SELECT {DEPT_FIELDS} FROM sys_dept WHERE parent_id = ?");
            let results = sql_query(sql)
                .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(parent_id_value)
                .load::<Dept>(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            Ok::<Vec<Dept>, Box<dyn StdError + Send + Sync>>(results)
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?
        .map_err(|e| e)
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let row = row.clone();
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            // 构建查询条件
            let mut query = sys_dept::table.into_boxed();

            // 添加所有可能的查询条件
            if !row.id.is_empty() {
                query = query.filter(sys_dept::id.eq(&row.id));
            }

            if let Some(ref name_value) = row.name {
                query = query.filter(sys_dept::name.like(format!("%{}%", name_value)));
            }

            if let Some(ref email_value) = row.email {
                query = query.filter(sys_dept::email.eq(email_value));
            }

            if let Some(ref telephone_value) = row.telephone {
                query = query.filter(sys_dept::telephone.eq(telephone_value));
            }

            if let Some(ref address_value) = row.address {
                query = query.filter(sys_dept::address.eq(address_value));
            }

            if let Some(ref logo_value) = row.logo {
                query = query.filter(sys_dept::logo.eq(logo_value));
            }

            if let Some(ref parent_id_value) = row.parent_id {
                query = query.filter(sys_dept::parent_id.eq(parent_id_value));
            }

            if let Some(seq_no_value) = row.seq_no {
                query = query.filter(sys_dept::seq_no.eq(seq_no_value));
            }

            if let Some(status_value) = row.status {
                query = query.filter(sys_dept::status.eq(status_value));
            }

            if let Some(ref create_by_value) = row.create_by {
                query = query.filter(sys_dept::create_by.eq(create_by_value));
            }

            if let Some(create_time_value) = row.create_time {
                query = query.filter(sys_dept::create_time.eq(create_time_value));
            }

            if let Some(ref update_by_value) = row.update_by {
                query = query.filter(sys_dept::update_by.eq(update_by_value));
            }

            if let Some(update_time_value) = row.update_time {
                query = query.filter(sys_dept::update_time.eq(update_time_value));
            }

            if let Some(ref remark_value) = row.remark {
                query = query.filter(sys_dept::remark.eq(remark_value));
            }

            // 执行查询
            let results = query
                .load::<Dept>(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            Ok::<Vec<Dept>, Box<dyn StdError + Send + Sync>>(results)
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?
        .map_err(|e| e)
    }

    /// 根据主键更新部门
    ///
    /// 由于Diesel是通过Option字段来实现选择性更新的ORM，
    /// 且当前Dept模型中所有字段都是Option类型，
    /// 完整更新和选择性更新的逻辑是一致的，都是根据字段是否有值来决定是否更新。
    /// 因此直接调用选择性更新方法即可，避免代码重复。
    async fn update_by_primary_key(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        // 直接调用选择性更新方法，因为当前模型所有字段都是Option类型
        // 无论字段是否有值，update_by_primary_key_selective都能正确处理
        self.update_by_primary_key_selective(row).await
    }

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        let row = row.clone();
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || {
            let mut conn = pool
                .get()
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            let result = diesel::update(sys_dept::table.filter(sys_dept::id.eq(&row.id)))
                .set(&row)
                .execute(&mut conn)
                .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;

            Ok::<u64, Box<dyn StdError + Send + Sync>>(result as u64)
        })
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?
        .map_err(|e| e)
    }
}
