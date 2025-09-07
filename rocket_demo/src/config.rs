//! 配置文件解析模块
//!
//! 用于解析项目中的 TOML 配置文件

use serde::Deserialize;
use std::fs;

/// 数据库配置
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
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
