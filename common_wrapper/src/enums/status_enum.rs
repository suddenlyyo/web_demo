//! # 状态枚举
//!
//! 用于表示启用/禁用状态
//!
//! 该枚举用于表示系统中各种实体的启用/禁用状态，如部门状态、用户状态等。
//! 包含启用和禁用两种状态，每种状态都有对应的数值、描述和布尔值。

/// 状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusEnum {
    /// 启用状态
    ///
    /// 数值: 1
    /// 描述: "启用"
    /// 布尔值: true
    Enable = 1,
    /// 禁用状态
    ///
    /// 数值: 0
    /// 描述: "禁用"
    /// 布尔值: false
    Disable = 0,
}

impl StatusEnum {
    /// 获取状态描述
    ///
    /// 获取与枚举值对应的中文状态描述
    ///
    /// # 返回值
    ///
    /// [&'static str] - 返回与枚举值对应的状态描述
    pub const fn desc(&self) -> &'static str {
        match self {
            Self::Enable => "启用",
            Self::Disable => "禁用",
        }
    }

    /// 获取布尔值
    ///
    /// 获取与枚举值对应的布尔值
    ///
    /// # 返回值
    ///
    /// [bool] - 返回与枚举值对应的布尔值
    pub const fn bool_value(&self) -> bool {
        match self {
            Self::Enable => true,
            Self::Disable => false,
        }
    }

    /// 根据描述获取状态枚举
    ///
    /// 根据提供的中文状态描述查找对应的枚举值
    ///
    /// # 参数
    ///
    /// * `desc` - 状态描述，类型: [&str]
    ///
    /// # 返回值
    ///
    /// [Option]<[StatusEnum]> - 对应的StatusEnum枚举值，如果找不到则返回None
    pub fn from_desc(desc: &str) -> Option<Self> {
        match desc {
            "启用" => Some(Self::Enable),
            "禁用" => Some(Self::Disable),
            _ => None,
        }
    }

    /// 根据code获取状态枚举
    ///
    /// 根据提供的状态码查找对应的枚举值
    ///
    /// # 参数
    ///
    /// * `code` - 状态码，类型: [i32]
    ///
    /// # 返回值
    ///
    /// [Option]<[StatusEnum]> - 对应的StatusEnum枚举值，如果找不到则返回None
    pub const fn from_code(code: i32) -> Option<Self> {
        match code {
            1 => Some(Self::Enable),
            0 => Some(Self::Disable),
            _ => None,
        }
    }
}

/// 实现从i32到StatusEnum的转换
impl From<i32> for StatusEnum {
    /// 从i32转换为StatusEnum
    ///
    /// 根据提供的状态码查找对应的枚举值
    ///
    /// # 参数
    ///
    /// * `code` - 状态码，类型: [i32]
    ///
    /// # 返回值
    ///
    /// [StatusEnum] - 对应的StatusEnum枚举值，如果找不到则返回Disable
    fn from(code: i32) -> Self {
        match code {
            1 => Self::Enable,
            0 => Self::Disable,
            _ => Self::Disable,
        }
    }
}

/// 实现从StatusEnum到i32的转换
impl From<StatusEnum> for i32 {
    /// 从StatusEnum转换为i32
    ///
    /// 获取枚举值对应的状态码
    ///
    /// # 参数
    ///
    /// * `status` - StatusEnum枚举值
    ///
    /// # 返回值
    ///
    /// [i32] - 对应的i32值
    fn from(status: StatusEnum) -> Self {
        status as i32
    }
}

/// 实现从bool到StatusEnum的转换
impl From<bool> for StatusEnum {
    /// 从bool转换为StatusEnum
    ///
    /// 根据提供的布尔值查找对应的枚举值
    ///
    /// # 参数
    ///
    /// * `value` - 布尔值，类型: [bool]
    ///
    /// # 返回值
    ///
    /// [StatusEnum] - 对应的StatusEnum枚举值
    fn from(value: bool) -> Self {
        if value { Self::Enable } else { Self::Disable }
    }
}

/// 实现从StatusEnum到bool的转换
impl From<StatusEnum> for bool {
    /// 从StatusEnum转换为bool
    ///
    /// 获取枚举值对应的布尔值
    ///
    /// # 参数
    ///
    /// * `status` - StatusEnum枚举值
    ///
    /// # 返回值
    ///
    /// [bool] - 对应的bool值
    fn from(status: StatusEnum) -> Self {
        status.bool_value()
    }
}
