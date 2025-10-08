//! 配置文件解析模块
//!
//! 用于解析项目中的 TOML 配置文件

use serde::Deserialize;
use std::fs;

// 只在使用 Diesel 时才定义相关配置
#[cfg(feature = "diesel_impl")]
mod diesel_config {
    use serde::Deserialize;

    /// Diesel连接池配置
    #[derive(Debug, Deserialize, Default)]
    pub struct DieselPoolConfig {
        /// 连接池最大连接数
        pub max_size: Option<u32>,
        /// 连接池最小空闲连接数
        pub min_idle: Option<u32>,
        /// 获取连接的超时时间（秒）
        pub connection_timeout: Option<u64>,
        /// 连接池中连接的最大存活时间（秒）
        pub max_lifetime: Option<u64>,
        /// 连接池中空闲连接的超时时间（秒）
        pub idle_timeout: Option<u64>,
        /// 借出连接时测试其有效性
        pub test_on_check_out: Option<bool>,
    }
}

// 只在使用 SeaORM 时才定义相关配置
#[cfg(feature = "seaorm_impl")]
mod seaorm_config {
    use serde::Deserialize;

    /// SeaORM连接池配置
    #[derive(Debug, Deserialize, Default)]
    pub struct SeaormPoolConfig {
        /// 连接池最大连接数
        pub max_connections: Option<u32>,
        /// 连接池最小连接数
        pub min_connections: Option<u32>,
        /// 连接超时时间（秒）
        pub connect_timeout: Option<u64>,
        /// 获取连接的超时时间（秒）
        pub acquire_timeout: Option<u64>,
        /// 空闲连接的超时时间（秒）
        pub idle_timeout: Option<u64>,
        /// 连接的最大存活时间（秒）
        pub max_lifetime: Option<u64>,
    }
}

// 只在使用 SQLx 时才定义相关配置
#[cfg(feature = "sqlx_impl")]
mod sqlx_config {
    use serde::Deserialize;

    /// SQLx连接池配置
    #[derive(Debug, Deserialize, Default)]
    pub struct SqlxPoolConfig {
        /// 连接池最大连接数
        pub max_connections: Option<u32>,
        /// 连接池最小连接数
        pub min_connections: Option<u32>,
        /// 连接超时时间（秒）
        pub acquire_timeout: Option<u64>,
        /// 空闲连接的超时时间（秒）
        pub idle_timeout: Option<u64>,
        /// 连接的最大存活时间（秒）
        pub max_lifetime: Option<u64>,
    }
}

// 只在使用 Diesel 时才导入相关配置
#[cfg(feature = "diesel_impl")]
pub use crate::config::diesel_config::DieselPoolConfig;

// 只在使用 SeaORM 时才导入相关配置
#[cfg(feature = "seaorm_impl")]
pub use crate::config::seaorm_config::SeaormPoolConfig;

// 只在使用 SQLx 时才导入相关配置
#[cfg(feature = "sqlx_impl")]
pub use crate::config::sqlx_config::SqlxPoolConfig;

/// 数据库配置
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    // 只在使用 Diesel 时才包含 diesel 配置
    #[cfg(feature = "diesel_impl")]
    #[serde(default)]
    pub diesel: crate::config::DieselPoolConfig,
    // 只在使用 SeaORM 时才包含 seaorm 配置
    #[cfg(feature = "seaorm_impl")]
    #[serde(default)]
    pub seaorm: crate::config::SeaormPoolConfig,
    // 只在使用 SQLx 时才包含 sqlx 配置
    #[cfg(feature = "sqlx_impl")]
    #[serde(default)]
    pub sqlx: crate::config::SqlxPoolConfig,
}

/// 应用配置
#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
}

impl Config {
    /// 从文件加载配置
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// 从默认位置加载配置
    pub fn from_default_file() -> Result<Self, Box<dyn std::error::Error>> {
        // 首先尝试在当前目录查找配置文件
        if let Ok(config) = Self::from_file("config.toml") {
            return Ok(config);
        }

        // 如果当前目录没有找到，则尝试在上级目录查找
        Self::from_file("../config.toml")
    }
}
