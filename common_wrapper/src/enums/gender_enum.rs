//! # 性别枚举
//!
//! 对应数据库中的性别字段，提供性别相关的操作
//!
//! 该枚举用于表示人员的性别信息，包含男性、女性和未说明性别三种状态。
//! 每种状态都有对应的数值和中文名称。

/// 性别枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenderEnum {
    /// 男性
    ///
    /// 数值: 1
    /// 名称: "男"
    Male = 1,
    /// 女性
    ///
    /// 数值: 2
    /// 名称: "女"
    Female = 2,
    /// 未说明性别
    ///
    /// 数值: 0
    /// 名称: "未说明性别"
    Unknown = 0,
}

impl GenderEnum {
    /// 获取性别名称
    ///
    /// 获取与枚举值对应的中文性别名称
    ///
    /// # 返回值
    ///
    /// [&'static str] - 返回与枚举值对应的性别名称
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Male => "男",
            Self::Female => "女",
            Self::Unknown => "未说明性别",
        }
    }

    /// 根据性别名称获取性别枚举
    ///
    /// 根据提供的中文性别名称查找对应的枚举值
    ///
    /// # 参数
    ///
    /// * `name` - 性别名称，类型: [&str]
    ///
    /// # 返回值
    ///
    /// [Option]<[GenderEnum]> - 对应的GenderEnum枚举值，如果找不到则返回None
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "男" => Some(Self::Male),
            "女" => Some(Self::Female),
            "未说明性别" => Some(Self::Unknown),
            _ => None,
        }
    }
}

/// 实现从i32到GenderEnum的转换
impl From<i32> for GenderEnum {
    /// 从i32转换为GenderEnum
    ///
    /// 根据提供的数值查找对应的枚举值
    ///
    /// # 参数
    ///
    /// * `value` - 性别值，类型: [i32]
    ///
    /// # 返回值
    ///
    /// [GenderEnum] - 对应的GenderEnum枚举值，如果找不到则返回Unknown
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Male,
            2 => Self::Female,
            0 => Self::Unknown,
            _ => Self::Unknown,
        }
    }
}

/// 实现从GenderEnum到i32的转换
impl From<GenderEnum> for i32 {
    /// 从GenderEnum转换为i32
    ///
    /// 获取枚举值对应的数值
    ///
    /// # 参数
    ///
    /// * `gender` - GenderEnum枚举值
    ///
    /// # 返回值
    ///
    /// [i32] - 对应的i32值
    fn from(gender: GenderEnum) -> Self {
        gender as i32
    }
}
