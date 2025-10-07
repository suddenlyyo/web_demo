//! SeaORM实现的部门数据访问层
//!
//! 该模块提供了基于SeaORM的部门数据访问实现，支持异步数据库操作。
//! 实现了DeptRepository trait定义的所有方法。

use crate::config::Config;
use crate::entities::prelude::SysDept;
use crate::entities::sys_dept::{ActiveModel, Column, Model};
use crate::models::Dept;
use crate::repositories::dept::dept_repository::DeptRepository;
use async_trait::async_trait;
use sea_orm::*;
use std::error::Error as StdError;
use std::fmt::Debug;

/// 实现从Dept模型到SeaORM ActiveModel的转换
impl From<&Dept> for ActiveModel {
    fn from(dept: &Dept) -> Self {
        ActiveModel {
            id: Set(dept.id.clone()),
            name: Set(dept.name.clone()),
            email: Set(dept.email.clone()),
            telephone: Set(dept.telephone.clone()),
            address: Set(dept.address.clone()),
            logo: Set(dept.logo.clone()),
            parent_id: Set(dept.parent_id.clone()),
            seq_no: Set(dept.seq_no),
            status: Set(dept.status),
            create_by: Set(dept.create_by.clone()),
            create_time: Set(dept.create_time),
            update_by: Set(dept.update_by.clone()),
            update_time: Set(dept.update_time),
            remark: Set(dept.remark.clone()),
        }
    }
}

/// 实现从SeaORM Model到Dept模型的转换
impl From<Model> for Dept {
    fn from(model: Model) -> Self {
        Dept {
            id: model.id,
            parent_id: model.parent_id,
            name: model.name,
            email: model.email,
            telephone: model.telephone,
            address: model.address,
            logo: model.logo,
            seq_no: model.seq_no,
            status: model.status,
            create_by: model.create_by,
            create_time: model.create_time,
            update_by: model.update_by,
            update_time: model.update_time,
            remark: model.remark,
        }
    }
}

/// SeaORM实现的部门仓储
#[derive(Debug)]
pub struct DeptRepositorySeaormImpl {
    connection: DatabaseConnection,
}

impl DeptRepositorySeaormImpl {
    /// 创建新的SeaORM部门仓储实例
    ///
    /// # 返回值
    /// 返回新的部门仓储实例
    pub async fn new() -> Result<Self, Box<dyn StdError + Send + Sync>> {
        // 从配置文件中读取数据库URL
        let config = Config::from_default_file().expect("无法加载配置文件");
        let database_url = config.database.url;
        let connection = Database::connect(&database_url)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(Self { connection })
    }
}

#[async_trait]
impl DeptRepository for DeptRepositorySeaormImpl {
    /// 根据主键删除部门
    async fn delete_by_primary_key(&self, id: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
        SysDept::delete_by_id(id)
            .exec(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(())
    }

    /// 插入部门记录
    async fn insert(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        let active_model: ActiveModel = row.into();
        active_model
            .insert(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(())
    }

    /// 选择性插入部门记录
    async fn insert_selective(&self, row: &Dept) -> Result<(), Box<dyn StdError + Send + Sync>> {
        // 转换为ActiveModel
        let mut active_model = ActiveModel { ..Default::default() };

        // id字段在Dept模型中是String类型，不是Option
        active_model.id = sea_orm::ActiveValue::Set(row.id.clone());

        // 只有 Some 值的字段才插入
        if let Some(ref name) = row.name {
            active_model.name = sea_orm::ActiveValue::Set(Some(name.clone()));
        }

        if let Some(ref email) = row.email {
            active_model.email = sea_orm::ActiveValue::Set(Some(email.clone()));
        }

        if let Some(ref telephone) = row.telephone {
            active_model.telephone = sea_orm::ActiveValue::Set(Some(telephone.clone()));
        }

        if let Some(ref address) = row.address {
            active_model.address = sea_orm::ActiveValue::Set(Some(address.clone()));
        }

        if let Some(ref logo) = row.logo {
            active_model.logo = sea_orm::ActiveValue::Set(Some(logo.clone()));
        }

        if let Some(ref parent_id) = row.parent_id {
            active_model.parent_id = sea_orm::ActiveValue::Set(Some(parent_id.clone()));
        }

        if let Some(ref seq_no) = row.seq_no {
            active_model.seq_no = sea_orm::ActiveValue::Set(Some(*seq_no));
        }

        if let Some(ref status) = row.status {
            active_model.status = sea_orm::ActiveValue::Set(Some(*status));
        }

        if let Some(ref create_by) = row.create_by {
            active_model.create_by = sea_orm::ActiveValue::Set(Some(create_by.clone()));
        }

        if let Some(ref create_time) = row.create_time {
            active_model.create_time = sea_orm::ActiveValue::Set(Some(*create_time));
        }

        if let Some(ref update_by) = row.update_by {
            active_model.update_by = sea_orm::ActiveValue::Set(Some(update_by.clone()));
        }

        if let Some(ref update_time) = row.update_time {
            active_model.update_time = sea_orm::ActiveValue::Set(Some(*update_time));
        }

        if let Some(ref remark) = row.remark {
            active_model.remark = sea_orm::ActiveValue::Set(Some(remark.clone()));
        }

        active_model
            .save(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(())
    }

    /// 根据主键查询部门
    async fn select_by_primary_key(&self, id: &str) -> Result<Option<Dept>, Box<dyn StdError + Send + Sync>> {
        let model = SysDept::find_by_id(id)
            .one(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(model.map(|m| m.into()))
    }

    /// 根据父部门ID查询部门列表
    async fn select_dept_by_parent_id(&self, parent_id: &str) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let models = SysDept::find()
            .filter(Column::ParentId.eq(parent_id))
            .all(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    /// 查询部门列表
    async fn select_dept_list(&self, row: &Dept) -> Result<Vec<Dept>, Box<dyn StdError + Send + Sync>> {
        let models = SysDept::find()
            .filter(Column::Name.like(format!("%{}%", row.name.as_ref().unwrap_or(&String::new()))))
            .filter(Column::Status.eq(row.status.unwrap_or(0)))
            .all(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    /// 根据主键更新部门
    async fn update_by_primary_key(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        //转换为ActiveModel
        let mut active_model: ActiveModel = row.into();
        //由于ID是主键,所以不需要设置
        active_model.id = sea_orm::ActiveValue::Unchanged(row.id.clone());
        // 执行更新
        let result: UpdateResult = SysDept::update_many()
            .set(active_model)
            .filter(<SysDept as sea_orm::EntityTrait>::Column::Id.eq(row.id.clone()))
            .exec(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(result.rows_affected)
    }

    /// 根据主键选择性更新部门
    async fn update_by_primary_key_selective(&self, row: &Dept) -> Result<u64, Box<dyn StdError + Send + Sync>> {
        //转换为ActiveModel
        let mut active_model = ActiveModel {
            id: sea_orm::ActiveValue::Unchanged(row.id.clone()), // 主键必须设置
            ..Default::default()
        };

        // 只有 Some 值的字段才更新
        if let Some(ref name) = row.name {
            active_model.name = sea_orm::ActiveValue::Set(Some(name.clone()));
        }

        if let Some(ref email) = row.email {
            active_model.email = sea_orm::ActiveValue::Set(Some(email.clone()));
        }

        if let Some(ref telephone) = row.telephone {
            active_model.telephone = sea_orm::ActiveValue::Set(Some(telephone.clone()));
        }

        if let Some(ref address) = row.address {
            active_model.address = sea_orm::ActiveValue::Set(Some(address.clone()));
        }

        if let Some(ref logo) = row.logo {
            active_model.logo = sea_orm::ActiveValue::Set(Some(logo.clone()));
        }

        if let Some(ref parent_id) = row.parent_id {
            active_model.parent_id = sea_orm::ActiveValue::Set(Some(parent_id.clone()));
        }

        if let Some(ref seq_no) = row.seq_no {
            active_model.seq_no = sea_orm::ActiveValue::Set(Some(*seq_no));
        }

        if let Some(ref status) = row.status {
            active_model.status = sea_orm::ActiveValue::Set(Some(*status));
        }

        if let Some(ref create_by) = row.create_by {
            active_model.create_by = sea_orm::ActiveValue::Set(Some(create_by.clone()));
        }

        if let Some(ref create_time) = row.create_time {
            active_model.create_time = sea_orm::ActiveValue::Set(Some(*create_time));
        }

        if let Some(ref update_by) = row.update_by {
            active_model.update_by = sea_orm::ActiveValue::Set(Some(update_by.clone()));
        }

        if let Some(ref update_time) = row.update_time {
            active_model.update_time = sea_orm::ActiveValue::Set(Some(*update_time));
        }

        if let Some(ref remark) = row.remark {
            active_model.remark = sea_orm::ActiveValue::Set(Some(remark.clone()));
        }

        // 执行更新
        let result: UpdateResult = SysDept::update_many()
            .set(active_model)
            .filter(<SysDept as sea_orm::EntityTrait>::Column::Id.eq(row.id.clone()))
            .exec(&self.connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError + Send + Sync>)?;
        Ok(result.rows_affected)
    }
}
