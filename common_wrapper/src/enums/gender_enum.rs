//! # 性别枚举
//!
//! 对应数据库中的性别字段，提供性别相关的操作

/// 性别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenderEnum {
    /// 男性
    Male = 1,
    /// 女性
    Female = 2,
    /// 未说明性别
    Unknown = 0,
}

impl GenderEnum {
    /// 获取性别名称
    ///
    /// # 返回值
    ///
    /// 返回与枚举值对应的性别名称
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Male => "男",
            Self::Female => "女",
            Self::Unknown => "未说明性别",
        }
    }

    /// 根据性别名称获取性别枚举
    ///
    /// # 参数
    ///
    /// * `name` - 性别名称
    ///
    /// # 返回值
    ///
    /// 对应的GenderEnum枚举值，如果找不到则返回None
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "男" => Some(Self::Male),
            "女" => Some(Self::Female),
            "未说明性别" => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl From<i32> for GenderEnum {
    /// 从i32转换为GenderEnum
    ///
    /// # 参数
    ///
    /// * `value` - 性别值
    ///
    /// # 返回值
    ///
    /// 对应的GenderEnum枚举值，如果找不到则返回Unknown
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Male,
            2 => Self::Female,
            0 => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

impl From<GenderEnum> for i32 {
    /// 从GenderEnum转换为i32
    ///
    /// # 参数
    ///
    /// * `gender` - GenderEnum枚举值
    ///
    /// # 返回值
    ///
    /// 对应的i32值
    fn from(gender: GenderEnum) -> Self {
        gender as i32
    }
}
