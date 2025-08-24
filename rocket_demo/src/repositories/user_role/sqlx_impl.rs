//! 用户角色数据访问层SQLx实现

use crate::models::UserRole;
use crate::repositories::user_role::user_role_repository::UserRoleRepository;
use rocket::async_trait;
use sqlx::PgPool;

/// 用户角色数据访问SQLx实现
#[derive(Debug)]
pub struct UserRoleRepositorySqlxImpl {
    pool: PgPool,
}

impl UserRoleRepositorySqlxImpl {
    /// 创建新的用户角色仓库SQLx实现
    ///
    /// # 参数
    ///
    /// * `pool` - PostgreSQL连接池，类型: [PgPool]
    ///
    /// # 返回值
    ///
    /// 新的用户角色仓库SQLx实现实例
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRoleRepository for UserRoleRepositorySqlxImpl {
    /// 根据用户ID和角色ID删除用户角色
    async fn delete_by_primary_key(&self, user_id: &str, role_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!("DELETE FROM sys_user_role WHERE user_id = $1 AND role_id = $2", user_id, role_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 插入用户角色记录
    async fn insert(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!(
            "INSERT INTO sys_user_role (user_id, role_id) VALUES ($1, $2)",
            row.user_id,
            row.role_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 选择性插入用户角色记录
    async fn insert_selective(&self, row: &UserRole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 对于用户角色，insert和insert_selective是相同的
        self.insert(row).await
    }

    /// 根据角色ID查询用户角色列表
    async fn select_user_role_by_role_id(&self, role_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = sqlx::query_as!(UserRole, "SELECT user_id, role_id FROM sys_user_role WHERE role_id = $1", role_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    /// 根据用户ID查询用户角色列表
    async fn select_user_role_by_user_id(&self, user_id: &str) -> Result<Vec<UserRole>, Box<dyn std::error::Error + Send + Sync>> {
        let rows = sqlx::query_as!(UserRole, "SELECT user_id, role_id FROM sys_user_role WHERE user_id = $1", user_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows)
    }

    /// 批量插入用户角色
    async fn batch_insert(&self, list: &[UserRole]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 构建批量插入SQL语句
        if list.is_empty() {
            return Ok(());
        }

        let mut query = "INSERT INTO sys_user_role (user_id, role_id) VALUES ".to_string();
        let mut params: Vec<&str> = Vec::new();
        
        for (i, user_role) in list.iter().enumerate() {
            if i > 0 {
                query.push_str(", ");
            }
            query.push_str(&format!("(${}, ${})", params.len() + 1, params.len() + 2));
            params.push(&user_role.user_id);
            params.push(&user_role.role_id);
        }
        
        // 执行批量插入
        let mut query_builder = sqlx::QueryBuilder::new(query);
        for param in params {
            query_builder.push_bind(param);
        }
        
        query_builder.build().execute(&self.pool).await?;
        Ok(())
    }

    /// 根据用户ID和角色ID列表批量删除用户角色
    async fn batch_delete_by_user_and_role_ids(&self, user_id: &str, list: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if list.is_empty() {
            return Ok(());
        }

        let mut query = "DELETE FROM sys_user_role WHERE user_id = $1 AND role_id IN (".to_string();
        for (i, _) in list.iter().enumerate() {
            if i > 0 {
                query.push(',');
            }
            query.push_str(&format!("${}", i + 2));
        }
        query.push(')');

        let mut query_builder = sqlx::QueryBuilder::new(query);
        query_builder.push_bind(user_id);
        for role_id in list {
            query_builder.push_bind(role_id);
        }

        query_builder.build().execute(&self.pool).await?;
        Ok(())
    }

    /// 根据用户ID删除用户角色
    async fn delete_by_user_id(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query!("DELETE FROM sys_user_role WHERE user_id = $1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}