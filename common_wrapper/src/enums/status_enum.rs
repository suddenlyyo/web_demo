//! # 状态枚举
//!
//! 用于表示启用/禁用状态

/// 状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusEnum {
    /// 启用状态
    Enable = 1,
    /// 禁用状态
    Disable = 0,
}

impl StatusEnum {
    /// 获取状态描述
    ///
    /// # 返回值
    ///
    /// 返回与枚举值对应的状态描述
    pub const fn desc(&self) -> &'static str {
        match self {
            Self::Enable => "启用",
            Self::Disable => "禁用",
        }
    }

    /// 获取布尔值
    ///
    /// # 返回值
    ///
    /// 返回与枚举值对应的布尔值
    pub const fn bool_value(&self) -> bool {
        match self {
            Self::Enable => true,
            Self::Disable => false,
        }
    }

    /// 根据描述获取状态枚举
    ///
    /// # 参数
    ///
    /// * `desc` - 状态描述
    ///
    /// # 返回值
    ///
    /// 对应的StatusEnum枚举值，如果找不到则返回None
    pub fn from_desc(desc: &str) -> Option<Self> {
        match desc {
            "启用" => Some(Self::Enable),
            "禁用" => Some(Self::Disable),
            _ => None,
        }
    }

    /// 根据code获取状态枚举
    ///
    /// # 参数
    ///
    /// * `code` - 状态码
    ///
    /// # 返回值
    ///
    /// 对应的StatusEnum枚举值，如果找不到则返回None
    pub const fn from_code(code: i32) -> Option<Self> {
        match code {
            1 => Some(Self::Enable),
            0 => Some(Self::Disable),
            _ => None,
        }
    }
}

impl From<i32> for StatusEnum {
    /// 从i32转换为StatusEnum
    ///
    /// # 参数
    ///
    /// * `code` - 状态码
    ///
    /// # 返回值
    ///
    /// 对应的StatusEnum枚举值，如果找不到则返回Disable
    fn from(code: i32) -> Self {
        match code {
            1 => Self::Enable,
            0 => Self::Disable,
            _ => Self::Disable,
        }
    }
}

impl From<StatusEnum> for i32 {
    /// 从StatusEnum转换为i32
    ///
    /// # 参数
    ///
    /// * `status` - StatusEnum枚举值
    ///
    /// # 返回值
    ///
    /// 对应的i32值
    fn from(status: StatusEnum) -> Self {
        status as i32
    }
}

impl From<bool> for StatusEnum {
    /// 从bool转换为StatusEnum
    ///
    /// # 参数
    ///
    /// * `value` - 布尔值
    ///
    /// # 返回值
    ///
    /// 对应的StatusEnum枚举值
    fn from(value: bool) -> Self {
        if value { Self::Enable } else { Self::Disable }
    }
}

impl From<StatusEnum> for bool {
    /// 从StatusEnum转换为bool
    ///
    /// # 参数
    ///
    /// * `status` - StatusEnum枚举值
    ///
    /// # 返回值
    ///
    /// 对应的bool值
    fn from(status: StatusEnum) -> Self {
        status.bool_value()
    }
}
