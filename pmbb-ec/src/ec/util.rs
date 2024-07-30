//! 工具函数.
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, stdin, Read, Stdin};

use reed_solomon_erasure::galois_8::ReedSolomon;

use super::{EC参数, E, ENV_PMBB_EC};

/// 解析 PMBB_EC 环境变量
///
/// 格式: `PMBB_EC=RS_3_2_4MB`
pub fn 解析_ec(原始: String) -> Result<EC参数, E> {
    let 切分: Vec<String> = 原始.split("_").map(|i| i.into()).collect();
    // 长度至少为 4
    if 切分.len() < 4 {
        return Err(format!("bad PMBB_EC={}", 原始).into());
    }
    let 名称 = 切分[0].clone();

    let 原始块数 =
        u16::from_str_radix(&切分[1], 10).map_err(|_| E::new(format!("bad PMBB_EC={}", 原始)))?;
    let 冗余块数 =
        u16::from_str_radix(&切分[2], 10).map_err(|_| E::new(format!("bad PMBB_EC={}", 原始)))?;
    let 块长 = match 切分[3].as_str() {
        "4MB" => 4 * 1024 * 1024,
        "2KB" => 2 * 1024,
        _ => {
            return Err(format!("unknown PMBB_EC block size {}", &切分[3]).into());
        }
    };

    Ok(EC参数 {
        原始,
        切分,
        名称,
        原始块数,
        冗余块数,
        块长,
    })
}

/// 读取 PMBB_EC 环境变量, 并解析内容
pub fn 获取_ec(默认: &str) -> Result<EC参数, Box<dyn Error>> {
    let ec = env::var(ENV_PMBB_EC).unwrap_or(默认.into());
    Ok(解析_ec(ec)?)
}

/// 可以读取的抽象
pub enum Read1 {
    F(File),
    S(Stdin),
}

impl Read for Read1 {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::F(r) => r.read(buf),
            Self::S(r) => r.read(buf),
        }
    }
}

/// 读取文件, 或者 stdin
pub fn file_or_stdin(n: String) -> Result<Read1, Box<dyn Error>> {
    // `-` -> stdin
    match n.as_str() {
        "-" => Ok(Read1::S(stdin())),
        _ => Ok(Read1::F(File::open(n)?)),
    }
}

/// EC_RS 计算
#[derive(Debug, Clone)]
pub struct EC {
    /// 参数
    a: EC参数,
    /// Reed-Solomon erasure code
    rs: ReedSolomon,
}

impl EC {
    /// 根据参数创建实例
    pub fn new(a: EC参数) -> Result<Self, Box<dyn Error>> {
        // EC 类型
        match a.名称.as_str() {
            // 支持的算法
            "RS" => {
                // TODO 检查更多 EC 参数

                let rs = ReedSolomon::new(a.原始块数 as usize, a.冗余块数 as usize)?;
                Ok(Self { a, rs })
            }
            // 不支持
            _ => Err(Box::new(E::new(format!("unknown EC {}", a.名称)))),
        }
    }

    /// 获取参数
    pub fn 参数(&self) -> EC参数 {
        self.a.clone()
    }

    /// 根据原始数据块, 计算冗余数据
    pub fn 编码(&self, b: &mut Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
        self.rs.encode(b)?;
        Ok(())
    }

    /// 恢复数据块
    pub fn 恢复(&self, b: &mut Vec<Option<Vec<u8>>>) -> Result<(), Box<dyn Error>> {
        self.rs.reconstruct(b)?;
        Ok(())
    }
}
