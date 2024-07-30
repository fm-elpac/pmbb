//! 集中类型定义

/// 程序名称及版本号
pub const P_VERSION: &'static str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

/// 环境变量名称 PMBB_EC
pub const ENV_PMBB_EC: &'static str = "PMBB_EC";

/// EC 参数
#[derive(Debug, Clone)]
pub struct EC参数 {
    /// 原始 PMBB_EC 字符串, 比如 RS_3_2_4MB
    pub 原始: String,
    /// 使用下划线字符切分
    pub 切分: Vec<String>,
    /// EC 算法名称, 比如 RS
    pub 名称: String,
    /// 原始数据块个数
    pub 原始块数: u16,
    /// 冗余数据块个数
    pub 冗余块数: u16,
    /// 数据块长度 (字节)
    pub 块长: usize,
}

impl Default for EC参数 {
    fn default() -> Self {
        Self {
            原始: "RS_1_0_4MB".into(),
            切分: vec!["RS".into(), "1".into(), "0".into(), "4MB".into()],
            名称: "RS".into(),
            原始块数: 1,
            冗余块数: 0,
            块长: 4 * 1024 * 1024,
        }
    }
}

// TODO
