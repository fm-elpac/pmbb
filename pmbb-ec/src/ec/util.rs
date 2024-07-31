//! 工具函数.
use std::env;
use std::error::Error;
use std::fs::{read, read_to_string, write, File};
use std::io::{self, stdin, Read, Stdin, Write};
use std::path::{absolute, Path};

use chrono::{format::SecondsFormat, offset::Utc};
use pm_bin::log::debug;
use reed_solomon_erasure::galois_8::ReedSolomon;
use relative_path::PathExt;
use sha2::{Digest, Sha256};

use super::{EC元数据, EC参数, E, ENV_PMBB_EC, L, L2};

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
        "4MB" => L,
        "2KB" => L2,
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

/// 读取 _pmbb_ec.json 文件
pub fn 读取_ec元数据(f: &str) -> Result<EC元数据, Box<dyn Error>> {
    debug!("read {}", f);
    let b = read(f)?;
    let o = serde_json::from_slice(&b)?;
    Ok(o)
}

/// 保存 _pmbb_ec.json 文件
pub fn 保存_ec元数据(f: &str, m: &mut EC元数据) -> Result<(), Box<dyn Error>> {
    // 更新保存时间
    let t = Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true);
    m._last_update = t;

    let b = serde_json::to_vec(m)?;
    debug!("write {}", f);
    write(f, b)?;
    Ok(())
}

/// 读取数据块 sha256.txt 文件
pub fn 读取sha256(f: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let t = read_to_string(f)?;
    Ok(t.lines().map(From::from).collect())
}

/// 文件写入器, 一次写入一行
#[derive(Debug)]
pub struct 写入行 {
    f: File,
}

impl 写入行 {
    /// 打开文件
    pub fn new(p: &str) -> Result<Self, Box<dyn Error>> {
        debug!("write {}", p);
        let f = File::create(p)?;

        Ok(Self { f })
    }

    /// 写入一行
    pub fn 写(&mut self, t: &str) -> Result<(), Box<dyn Error>> {
        let b = format!("{}\n", t);
        self.f.write_all(b.as_bytes())?;
        Ok(())
    }
}

/// 计算一块数据的 sha256
pub fn 计算sha256(b: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(b);
    // 输出 hex
    let h1 = h.finalize();
    base16ct::lower::encode_string(&h1)
}

/// 文件写入器, 写入数据块, 自动计算 sha256 (块大小 4MB)
#[derive(Debug)]
pub struct 写入sha256 {
    /// 写入的目标文件
    f: File,
    /// _sha256.txt 文件
    h: 写入行,
    /// 写入缓冲区 (4MB)
    b: Vec<u8>,
}

impl 写入sha256 {
    /// 准备写入
    pub fn new(p: &str, s: &str) -> Result<Self, Box<dyn Error>> {
        debug!("write {}", p);
        let f = File::create(p)?;
        let h = 写入行::new(s)?;

        Ok(Self {
            f,
            h,
            b: Vec::new(),
        })
    }

    // b: None -> &self.b
    fn 写一块(&mut self, b: Option<&[u8]>) -> Result<(), Box<dyn Error>> {
        let b = match b {
            Some(b) => b,
            None => &self.b,
        };

        self.f.write_all(b)?;

        let h = 计算sha256(b);
        self.h.写(&h)?;
        Ok(())
    }

    /// 写入数据块
    pub fn 写(&mut self, b: &[u8]) -> Result<(), Box<dyn Error>> {
        // 最大数据块长度: 4MB
        if b.len() + self.b.len() > L {
            // 情况 1: 超长, 需要分块写
            // TODO 不允许 b.len() > L

            // 写满整块
            let i = L - self.b.len();
            self.b.extend_from_slice(&b[0..i]);
            self.写一块(None)?;
            // 剩余数据放入缓冲区
            self.b = Vec::new();
            self.b.extend_from_slice(&b[i..]);
        } else if (b.len() == L) && (self.b.len() == 9) {
            // 情况 2: 恰好整块写
            self.写一块(Some(b))?;
        } else {
            // 将数据加入缓冲区
            self.b.extend_from_slice(b);
            // 检查是否达到整块
            if self.b.len() == L {
                self.写一块(None)?;
                // 重置缓冲区
                self.b = Vec::new();
            }
        }
        Ok(())
    }

    /// 写入结束
    pub fn close(mut self) -> Result<(), Box<dyn Error>> {
        if self.b.len() > 0 {
            // 写入最后一块数据
            self.写一块(None)?;
        }
        Ok(())
    }
}

/// 计算文件的相对路径
pub fn 相对路径(从: &str, 至: &str) -> Result<String, Box<dyn Error>> {
    let f = absolute(Path::new(从))?;
    let t = absolute(Path::new(至))?;
    let r = t.relative_to(f)?;
    Ok(r.into_string())
}
