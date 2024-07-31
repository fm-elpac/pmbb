//! 集中类型定义
use serde::{Deserialize, Serialize};

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

/// EC 元数据, 保存为 *_pmbb_ec.json 文件
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EC元数据 {
    /// pmbb-ec 版本号 (`P_VERSION`)
    pub pmbb: String,
    /// PMBB_EC 参数 (环境变量)
    pub pmbb_ec: String,
    /// 文件列表
    pub file: Vec<EC元数据_文件>,
    /// 数据保存时间 (UTC)
    pub _last_update: String,
}

impl Default for EC元数据 {
    fn default() -> Self {
        Self {
            pmbb: P_VERSION.into(),
            pmbb_ec: "".into(),
            file: vec![],
            _last_update: "".into(),
        }
    }
}

/// EC元数据.文件
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EC元数据_文件 {
    /// 指向文件相对 _pmbb_ec.json 文件的路径
    pub p: String,
    /// 文件长度 (字节)
    pub b: usize,
    /// 数据块的 sha256 文件
    pub sha256: Option<String>,
    /// EC 类型
    pub ec: Option<u32>,
}

impl Default for EC元数据_文件 {
    fn default() -> Self {
        Self {
            p: "".into(),
            b: 0,
            sha256: None,
            ec: None,
        }
    }
}

/// EC元数据.文件.EC类型: 本文件为冗余数据块
pub const EC元数据_EC1: u32 = 1;

/// 计算 sha256 的数据块长度 (4MB)
pub const L: usize = 4 * 1024 * 1024;

/// 数据块长度 (2KB)
pub const L2: usize = 2 * 1024;

/// _sha256.txt 文件名后缀
pub const 后缀_SHA256: &'static str = "_sha256.txt";
